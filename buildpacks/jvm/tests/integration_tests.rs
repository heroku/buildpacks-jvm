use libcnb_test::{assert_contains, BuildConfig, TestRunner};

#[test]
#[ignore = "integration test"]
fn test() {
    let builder_name = std::env::var("INTEGRATION_TEST_CNB_BUILDER").unwrap();

    TestRunner::default().build(
        BuildConfig::new(&builder_name, "../../test-fixtures/java-8-app"),
        |context| {
            assert_contains!(
                context.run_shell_command("java -version").stderr,
                match builder_name.as_str() {
                    "heroku/buildpacks:18" | "heroku/buildpacks:20" =>
                        "openjdk version \"1.8.0_352-heroku\"",
                    _ => "openjdk version \"1.8.0_352\"",
                }
            );
        },
    )
}
