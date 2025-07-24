use crate::default_build_config;
use libcnb_test::{assert_contains, TestRunner};

#[test]
#[ignore = "integration test"]
fn environment_variables_in_gradle() {
    TestRunner::default().build(
        default_build_config("test-apps/gradle-env-test")
            .envs([("GRADLE_TASK", "build"), ("HELLO", "world")]),
        |context| {
            // The Gradle build outputs all environment variables. We want to check that the
            // environment variable was exposed to the Gradle build.
            assert_contains!(context.pack_stdout, "HELLO=world");
        },
    );
}
