use buildpacks_jvm_shared_test::start_container_assert_basic_http_response;
use libcnb_test::{BuildConfig, BuildpackReference, TestRunner};

#[test]
#[ignore = "integration test"]
fn spring_boot_process_type() {
    TestRunner::default().build(
        BuildConfig::new(
            std::env::var("INTEGRATION_TEST_CNB_BUILDER").unwrap(),
            "test-apps/buildpack-java-spring-boot-test",
        )
        .buildpacks(vec![
            BuildpackReference::Other(String::from("heroku/jvm")),
            BuildpackReference::Crate,
        ]),
        |context| {
            start_container_assert_basic_http_response(&context, "Hello from Spring Boot!");
        },
    )
}
