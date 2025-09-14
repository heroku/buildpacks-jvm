use crate::default_build_config;
use libcnb_test::{TestRunner, assert_contains};

#[test]
#[ignore = "integration test"]
fn polyglot_maven_app() {
    TestRunner::default().build(
        default_build_config("test-apps/simple-http-service-groovy-polyglot"),
        |context| {
            assert_contains!(context.pack_stdout, "[INFO] BUILD SUCCESS");
        },
    );
}
