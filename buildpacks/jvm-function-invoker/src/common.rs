use libcnb::read_toml_file;
use libherokubuildpack::toml::toml_select_value;
use std::path::Path;
use toml::Value;

pub fn project_toml_salesforce_type_is_function(project_toml_path: &Path) -> bool {
    read_toml_file(project_toml_path)
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
    use super::project_toml_salesforce_type_is_function;
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
