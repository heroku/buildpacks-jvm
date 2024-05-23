use crate::default_build_config;
use libcnb_test::{assert_contains, TestRunner};

#[test]
#[ignore = "integration test"]
fn openjdk_8() {
    TestRunner::default().build(default_build_config("test-apps/java-8-app"), |context| {
        assert_contains!(
            context.run_shell_command("java -version").stderr,
            "openjdk version \"1.8.0_412\""
        );
    });
}

#[test]
#[ignore = "integration test"]
fn openjdk_11() {
    TestRunner::default().build(default_build_config("test-apps/java-11-app"), |context| {
        assert_contains!(
            context.run_shell_command("java -version").stderr,
            "openjdk version \"11.0.23\""
        );
    });
}

#[test]
#[ignore = "integration test"]
fn openjdk_17() {
    TestRunner::default().build(default_build_config("test-apps/java-17-app"), |context| {
        assert_contains!(
            context.run_shell_command("java -version").stderr,
            "openjdk version \"17.0.11\""
        );
    });
}

#[test]
#[ignore = "integration test"]
fn openjdk_21() {
    TestRunner::default().build(default_build_config("test-apps/java-21-app"), |context| {
        assert_contains!(
            context.run_shell_command("java -version").stderr,
            "openjdk version \"21.0.3\""
        );
    });
}

#[test]
#[ignore = "integration test"]
fn openjdk_22() {
    TestRunner::default().build(default_build_config("test-apps/java-22-app"), |context| {
        assert_contains!(
            context.run_shell_command("java -version").stderr,
            "openjdk version \"22.0.1\""
        );
    });
}
