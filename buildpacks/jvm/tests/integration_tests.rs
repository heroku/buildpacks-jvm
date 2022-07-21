use libcnb_test::{assert_contains, BuildConfig, TestRunner};

#[test]
fn test() {
    TestRunner::default().build(
        BuildConfig::new("heroku/buildpacks:20", "../../test-fixtures/java-8-app"),
        |context| {
            assert_contains!(
                context.run_shell_command("java -version").stderr,
                "openjdk version \"1.8.0_332-heroku\""
            );
        },
    )
}
