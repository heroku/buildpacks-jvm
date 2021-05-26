use crate::util::{self, logger::Logger};
use libcnb::{build::GenericBuildContext, layer::Layer};
use std::{convert::TryFrom, fs, path::Path, process::Command};

pub const RUNTIME_JAR_FILE_NAME: &str = "runtime.jar";

pub struct Builder<'a, 'b> {
    logger: &'b Logger,
    ctx: &'a GenericBuildContext,
}

impl<'a, 'b> Builder<'a, 'b> {
    pub fn new(ctx: &'a GenericBuildContext, logger: &'b Logger) -> anyhow::Result<Self> {
        Ok(Builder { ctx, logger })
    }

    pub fn contribute_opt_layer(&self) -> anyhow::Result<Layer> {
        let mut layer = self.ctx.layer("opt")?;
        layer.write_content_metadata_with_fn(|content_metadata| {
            content_metadata.launch = true;
            content_metadata.build = true;
            content_metadata.cache = false;
        })?;

        let contents = include_str!("../opt/run.sh");
        let run_sh_path = layer.as_path().join("run.sh");
        fs::write(&run_sh_path, contents)?;
        #[cfg(target_family = "unix")]
        set_executable(&run_sh_path)?;
        Ok(layer)
    }

    pub fn contribute_runtime_layer(&self) -> anyhow::Result<Layer> {
        self.logger.header("Installing Java function runtime")?;

        let mut runtime_layer = self.ctx.layer("sf-fx-runtime-java")?;
        let buildpack_toml: libcnb::data::buildpack::BuildpackToml = toml::from_str(
            &fs::read_to_string(self.ctx.buildpack_dir.join("buildpack.toml"))?,
        )?;
        let buildpack_toml_metadata =
            crate::data::buildpack_toml::Metadata::try_from(&buildpack_toml.metadata)?;
        let runtime_layer_metadata =
            crate::data::Runtime::from_runtime_layer(&runtime_layer.content_metadata().metadata);
        let runtime_jar_path = runtime_layer.as_path().join(RUNTIME_JAR_FILE_NAME);

        if buildpack_toml_metadata.runtime.sha256 == runtime_layer_metadata.sha256
            && runtime_jar_path.exists()
        {
            self.logger
                .info("Installed Java function runtime from cache")?;
        } else {
            self.logger.debug("Creating function runtime layer")?;
            runtime_layer.write_content_metadata_with_fn(|content_metadata| {
                content_metadata.launch = true;
                content_metadata.build = false;
                content_metadata.cache = true;

                content_metadata.metadata.insert(
                    String::from("runtime_jar_url"),
                    toml::Value::String(buildpack_toml_metadata.runtime.url.clone()),
                );
                content_metadata.metadata.insert(
                    String::from("runtime_jar_sha256"),
                    toml::Value::String(buildpack_toml_metadata.runtime.sha256.clone()),
                );
            })?;

            self.logger
                .debug("Function runtime layer successfully created")?;

            self.logger.info("Starting download of function runtime")?;
            util::download(&buildpack_toml_metadata.runtime.url, &runtime_jar_path).map_err(|_| {
              self.logger.error("Download of function runtime failed", format!(r#"
We couldn't download the function runtime at {}.

This is usually caused by intermittent network issues. Please try again and contact us should the error persist.
"#, buildpack_toml_metadata.runtime.url)).unwrap_err()
        })?;
            self.logger.info("Function runtime download successful")?;

            if buildpack_toml_metadata.runtime.sha256 != util::sha256(&fs::read(&runtime_jar_path)?)
            {
                self.logger.error(
                    "Function runtime integrity check failed",
                    r#"
We could not verify the integrity of the downloaded function runtime.
Please try again and contact us should the error persist.
        "#,
                )?;
            }

            self.logger
                .info("Function runtime installation successful")?;
        }

        Ok(runtime_layer)
    }

    pub fn contribute_function_bundle_layer(
        &self,
        runtime_jar_path: impl AsRef<Path>,
    ) -> anyhow::Result<Layer> {
        self.logger.header("Detecting function")?;

        let mut function_bundle_layer = self.ctx.layer("function-bundle")?;
        function_bundle_layer.write_content_metadata_with_fn(|content_metadata| {
            content_metadata.launch = true;
            content_metadata.build = false;
            content_metadata.cache = false;
        })?;

        let exit_status = Command::new("java")
            .arg("-jar")
            .arg(runtime_jar_path.as_ref())
            .arg("bundle")
            .arg(&self.ctx.app_dir)
            .arg(function_bundle_layer.as_path())
            .spawn()?
            .wait()?;

        if let Some(code) = exit_status.code() {
            match code {
                0 => {
                    self.logger.info("Detection successful")?;
                    Ok(())
                }
                1 => self.logger.error(
                    "No functions found",
                    r#"
Your project does not seem to contain any Java functions.
The output above might contain information about issues with your function.
"#,
                ),
                2 => self.logger.error(
                    "Multiple functions found",
                    r#"
Your project contains multiple Java functions.
Currently, only projects that contain exactly one (1) function are supported.
"#,
                ),
                3..=6 => self.logger.error(
                    "Detection failed",
                    format!(
                        r#"Function detection failed with internal error "{}""#,
                        code
                    ),
                ),
                _ => self.logger.error(
                    "Detection failed",
                    format!(
                        r#"
Function detection failed with unexpected error code {}.
The output above might contain hints what caused this error to happen.
"#,
                        code
                    ),
                ),
            }?;
        }

        let function_bundle_toml: crate::data::function_bundle::Toml = toml::from_slice(
            &fs::read(&function_bundle_layer.as_path().join("function-bundle.toml"))?,
        )?;

        self.logger.header(format!(
            "Detected function: {}",
            function_bundle_toml.function.class
        ))?;
        self.logger.info(format!(
            "Payload type: {}",
            function_bundle_toml.function.payload_class
        ))?;
        self.logger.info(format!(
            "Return type: {}",
            function_bundle_toml.function.return_class
        ))?;

        Ok(function_bundle_layer)
    }
}

#[cfg(target_family = "unix")]
fn set_executable(path: impl AsRef<Path>) -> anyhow::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(path, fs::Permissions::from_mode(0o755))?;

    Ok(())
}
