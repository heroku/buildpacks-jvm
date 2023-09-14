// Enable rustc and Clippy lints that are disabled by default.
// https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html#unused-crate-dependencies
#![warn(unused_crate_dependencies)]
// https://rust-lang.github.io/rust-clippy/stable/index.html
#![warn(clippy::pedantic)]
// This lint is too noisy and enforces a style that reduces readability in many cases.
#![allow(clippy::module_name_repetitions)]
// This crate is for internal use only, we don't always need error docs.
#![allow(clippy::missing_errors_doc)]

pub mod env;
pub mod fs;
pub mod log;
pub mod result;
pub mod system_properties;
