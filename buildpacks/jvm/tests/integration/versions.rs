/*
 * These tests are commented out for now until we have the ability to test meta-buildpacks.
 *
 * Previously, heroku/jvm could be used without another buildpack requiring 'jdk'. This feature has
 * been removed so that heroku/jvm can be composed with other buildpacks that only need 'jdk'
 * conditionally.
 *
 * The tests in this file aren't critical as installing OpenJDK will be exercised by other
 * buildpacks in this repository.
 */

/*
use libcnb_test::{assert_contains, BuildConfig, TestRunner};

#[test]
#[ignore = "integration test"]
fn test_openjdk_8_distribution_heroku_20() {
    TestRunner::default().build(
        BuildConfig::new("heroku/buildpacks:20", "test-apps/java-8-app"),
        |context| {
            assert_contains!(
                context.run_shell_command("java -version").stderr,
                "openjdk version \"1.8.0_382-heroku\""
            );
        },
    )
}

#[test]
#[ignore = "integration test"]
fn test_openjdk_8_distribution_heroku_22() {
    TestRunner::default().build(
        BuildConfig::new("heroku/builder:22", "test-apps/java-8-app"),
        |context| {
            assert_contains!(
                context.run_shell_command("java -version").stderr,
                "openjdk version \"1.8.0_382\""
            );
        },
    )
}
*/
