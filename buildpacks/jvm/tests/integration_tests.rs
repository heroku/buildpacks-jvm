use libcnb_test::{assert_contains, IntegrationTest};

#[test]
fn test() {
    IntegrationTest::new("heroku/buildpacks:20", "fixtures/java-8-app").run_test(|context| {
        context
            .prepare_container()
            .start_with_shell_command("java -version", |container| {
                assert_contains!(
                    container.logs_wait().stderr,
                    "openjdk version \"1.8.0_332-heroku\""
                )
            });
    })
}
