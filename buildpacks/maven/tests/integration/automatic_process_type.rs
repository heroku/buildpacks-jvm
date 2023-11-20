use crate::default_buildpacks;
use buildpacks_jvm_shared_test::{
    start_container_assert_basic_http_response, DEFAULT_INTEGRATION_TEST_BUILDER,
};
use libcnb_test::{BuildConfig, TestRunner};

#[test]
#[ignore = "integration test"]
fn spring_boot_process_type() {
    TestRunner::default().build(
        BuildConfig::new(
            DEFAULT_INTEGRATION_TEST_BUILDER,
            "test-apps/buildpack-java-spring-boot-test",
        )
        .buildpacks(default_buildpacks()),
        |context| {
            start_container_assert_basic_http_response(&context, "Hello from Spring Boot!");
        },
    );
}
