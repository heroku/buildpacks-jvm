use libcnb_test::{assert_contains, TestConfig, TestRunner};

#[test]
fn test() {
    TestRunner::default().run_test(
        TestConfig::new("heroku/buildpacks:20", "../../test-fixtures/java-8-app"),
        |context| {
            context
                .prepare_container()
                .start_with_shell_command("java -version", |container| {
                    assert_contains!(
                        container.logs_wait().stderr,
                        "openjdk version \"1.8.0_332-heroku\""
                    )
                });
        },
    )
}
