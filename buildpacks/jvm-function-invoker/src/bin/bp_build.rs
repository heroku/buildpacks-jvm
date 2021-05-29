use jvm_function_invoker_buildpack::{
    builder::{Builder, RUNTIME_JAR_FILE_NAME},
    util::logger::HerokuLogger,
};
use libcnb::{
    build::{cnb_runtime_build, GenericBuildContext},
    data,
    platform::Platform,
};

fn main() -> anyhow::Result<()> {
    cnb_runtime_build(build);

    Ok(())
}

fn build(ctx: GenericBuildContext) -> anyhow::Result<()> {
    let heroku_debug = ctx.platform.env().var("HEROKU_BUILDPACK_DEBUG").is_ok();
    let mut logger = HerokuLogger::new(heroku_debug);
    let mut builder = Builder::new(&ctx, &mut logger)?;

    let opt_layer = builder.contribute_opt_layer()?;
    let runtime_layer = builder.contribute_runtime_layer()?;
    let runtime_jar_path = runtime_layer.as_path().join(RUNTIME_JAR_FILE_NAME);
    let function_bundle_layer = builder.contribute_function_bundle_layer(&runtime_jar_path)?;

    let mut launch = data::launch::Launch::new();
    let cmd = format!(
        "{}/run.sh {} {}",
        opt_layer.as_path().display(),
        runtime_jar_path.display(),
        function_bundle_layer.as_path().display(),
    );
    launch.processes.push(data::launch::Process::new(
        "web",
        cmd,
        &[] as &[String],
        false,
    )?);
    ctx.write_launch(launch)?;

    Ok(())
}
