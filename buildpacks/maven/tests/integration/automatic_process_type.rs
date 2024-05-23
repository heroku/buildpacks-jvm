use crate::default_build_config;
use buildpacks_jvm_shared_test::start_container_assert_basic_http_response;
use libcnb_test::TestRunner;

#[test]
#[ignore = "integration test"]
fn spring_boot_process_type() {
    TestRunner::default().build(
        default_build_config("test-apps/buildpack-java-spring-boot-test"),
        |context| {
            start_container_assert_basic_http_response(&context, "Hello from Spring Boot!");
        },
    );
}
