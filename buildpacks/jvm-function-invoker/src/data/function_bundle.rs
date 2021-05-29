use serde::Deserialize;

#[derive(Deserialize)]
/// Struct representing the TOML file produced by executing the `sf-fx-runtime-java-runtime` in "bundle" mode.
pub struct Toml {
    pub function: Function,
}

#[derive(Deserialize)]
pub struct Function {
    pub class: String,
    pub payload_class: String,
    pub payload_media_type: String,
    pub return_class: String,
    pub return_media_type: String,
}
