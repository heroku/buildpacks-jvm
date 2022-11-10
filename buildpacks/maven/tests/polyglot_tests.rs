use libcnb_test::{assert_contains, BuildConfig, BuildpackReference, TestRunner};

#[test]
#[ignore = "integration test"]
fn polyglot_maven_app() {
    TestRunner::default().build(
        BuildConfig::new(
            std::env::var("INTEGRATION_TEST_CNB_BUILDER").unwrap(),
            "../../test-fixtures/simple-http-service-groovy-polyglot",
        )
        .buildpacks(vec![
            BuildpackReference::Other(String::from("heroku/jvm")),
            BuildpackReference::Crate,
        ]),
        |context| {
            assert_contains!(context.pack_stdout, "[INFO] BUILD SUCCESS");
        },
    )
}
