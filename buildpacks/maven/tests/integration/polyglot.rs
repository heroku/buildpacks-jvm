use crate::default_buildpacks;
use buildpacks_jvm_shared_test::DEFAULT_INTEGRATION_TEST_BUILDER;
use libcnb_test::{assert_contains, BuildConfig, TestRunner};

#[test]
#[ignore = "integration test"]
fn polyglot_maven_app() {
    TestRunner::default().build(
        BuildConfig::new(
            DEFAULT_INTEGRATION_TEST_BUILDER,
            "test-apps/simple-http-service-groovy-polyglot",
        )
        .buildpacks(default_buildpacks()),
        |context| {
            assert_contains!(context.pack_stdout, "[INFO] BUILD SUCCESS");
        },
    );
}
