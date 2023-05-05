//! Bundle all integration tests into one binary to:
//! - Reduce compile times
//! - Reduce required disk space
//! - Increase parallelism
//!
//! See: https://matklad.github.io/2021/02/27/delete-cargo-integration-tests.html#Implications

mod customization_tests;
mod misc_tests;
mod polyglot_tests;
mod process_types_tests;
mod settings_xml_tests;
mod smoke;
mod version_handling_tests;
