use libcnb_test::{assert_contains, BuildConfig, TestRunner};

#[test]
#[ignore = "integration test"]
fn test_openjdk_8_distribution_heroku_20() {
    TestRunner::default().build(
        BuildConfig::new("heroku/builder:20", "test-apps/java-8-app"),
        |context| {
            assert_contains!(
                context.run_shell_command("java -version").stderr,
                "openjdk version \"1.8.0_392-heroku\""
            );
        },
    );
}

#[test]
#[ignore = "integration test"]
fn test_openjdk_8_distribution_heroku_22() {
    TestRunner::default().build(
        BuildConfig::new("heroku/builder:22", "test-apps/java-8-app"),
        |context| {
            assert_contains!(
                context.run_shell_command("java -version").stderr,
                "openjdk version \"1.8.0_392\""
            );
        },
    );
}
