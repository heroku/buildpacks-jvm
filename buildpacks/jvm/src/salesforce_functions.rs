use libcnb::read_toml_file;
use std::path::Path;

pub(crate) fn is_salesforce_function_app(app_dir: &Path) -> bool {
    read_toml_file::<toml::value::Value>(app_dir.join("project.toml"))
        .is_ok_and(|project_toml_table| matches!(value_at_path(&project_toml_table, &["com", "salesforce", "type"]), Some(toml::Value::String(salesforce_type)) if salesforce_type == "function"))
}

fn value_at_path<'a>(table: &'a toml::value::Value, path: &[&str]) -> Option<&'a toml::Value> {
    let mut value = table;

    for path_segment in path {
        if let toml::Value::Table(table) = value {
            match table.get(*path_segment) {
                Some(next_value) => value = next_value,
                None => return None,
            }
        }
    }

    Some(value)
}
