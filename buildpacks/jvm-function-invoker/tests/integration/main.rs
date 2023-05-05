//! Bundle all integration tests into one binary to:
//! - Reduce compile times
//! - Reduce required disk space
//! - Increase parallelism
//!
//! See: https://matklad.github.io/2021/02/27/delete-cargo-integration-tests.html#Implications

mod smoke;
