use std::path::Path;

use libcnb::data::build_plan::BuildPlanBuilder;
use libcnb::{read_toml_file, DetectContext, DetectOutcome, Error, GenericPlatform};

use crate::error::JvmFunctionInvokerBuildpackError;
use crate::JvmFunctionInvokerBuildpackMetadata;
use libherokubuildpack::toml_select_value;
use toml::Value;

// https://github.com/Malax/libcnb.rs/issues/63
#[allow(clippy::needless_pass_by_value)]
// https://github.com/Malax/libcnb.rs/issues/86
#[allow(clippy::unnecessary_wraps)]
pub fn detect(
    context: DetectContext<GenericPlatform, JvmFunctionInvokerBuildpackMetadata>,
) -> Result<DetectOutcome, Error<JvmFunctionInvokerBuildpackError>> {
    let function_toml_path = context.app_dir.join("function.toml");
    let project_toml_path = context.app_dir.join("project.toml");

    if function_toml_path.exists() || project_toml_salesforce_type_is_function(&project_toml_path) {
        Ok(DetectOutcome::Pass(
            BuildPlanBuilder::new()
                .requires("jdk")
                .requires("jvm-application")
                .build(),
        ))
    } else {
        Ok(DetectOutcome::Fail)
    }
}

fn project_toml_salesforce_type_is_function(project_toml_path: &Path) -> bool {
    read_toml_file(&project_toml_path)
        .ok()
        .and_then(|table: Value| {
            toml_select_value(vec!["com", "salesforce", "type"], &table)
                .and_then(toml::Value::as_str)
                .map(|value| value == "function")
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod test {
    use crate::detect::project_toml_salesforce_type_is_function;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_project_toml_salesforce_type_is_function() {
        let temp_dir = tempdir().unwrap();
        let project_toml_path = temp_dir.path().join("project.toml");

        fs::write(
            &project_toml_path,
            "[com.salesforce]\ntype = \"function\"\n",
        )
        .unwrap();

        assert!(project_toml_salesforce_type_is_function(&project_toml_path));
    }
}
