use crate::default_build_config;
use libcnb_test::{assert_contains, assert_not_contains, TestRunner};

#[test]
#[ignore = "integration test"]
fn cache_dependencies_between_builds() {
    TestRunner::default().build(
        default_build_config("test-apps/simple-http-service"),
        |context| {
            assert_contains!(context.pack_stderr, "Downloading from central");

            context.rebuild(
                default_build_config("test-apps/simple-http-service"),
                |context| {
                    assert_not_contains!(context.pack_stderr, "Downloading from central");
                },
            );
        },
    );
}
