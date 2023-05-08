use crate::default_config;
use libcnb_test::{assert_contains, assert_not_contains, TestRunner};

#[test]
#[ignore = "integration test"]
fn cache_dependencies_between_builds() {
    TestRunner::default().build(default_config(), |context| {
        assert_contains!(context.pack_stdout, "Downloading from central");

        context.rebuild(default_config(), |context| {
            assert_not_contains!(context.pack_stdout, "Downloading from central");
        });
    });
}
