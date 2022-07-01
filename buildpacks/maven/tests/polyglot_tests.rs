use libcnb_test::{assert_contains, BuildpackReference, TestConfig, TestRunner};

#[test]
fn polyglot_maven_app() {
    TestRunner::default().run_test(
        TestConfig::new(
            "heroku/buildpacks:20",
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
