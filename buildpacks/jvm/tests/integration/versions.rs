use crate::default_build_config;
use indoc::formatdoc;
use libcnb::data::buildpack_id;
use libcnb_test::{BuildpackReference, TestRunner, assert_contains, assert_not_contains};

#[test]
#[ignore = "integration test"]
fn openjdk_default() {
    TestRunner::default().build(
        default_build_config("test-apps/java-default-app").buildpacks([
            BuildpackReference::CurrentCrate,
            // We need another buildpack to require 'jvm' in the build plan to be able to use a
            // default OpenJDK version. It could be any other buildpack, heroku/maven is just
            // convenient to use here.
            BuildpackReference::WorkspaceBuildpack(buildpack_id!("heroku/maven")),
        ]),
        |context| {
            assert_contains!(
                context.pack_stdout,
                &formatdoc! {"
                    ! WARNING: No OpenJDK version specified
                    ! 
                    ! Your application does not explicitly specify an OpenJDK version. The latest
                    ! long-term support (LTS) version will be installed. This currently is OpenJDK 21.
                    ! 
                    ! This default version will change when a new LTS version is released. Your
                    ! application might fail to build with the new version. We recommend explicitly
                    ! setting the required OpenJDK version for your application.
                    ! 
                    ! To set the OpenJDK version, add or edit the system.properties file in the root
                    ! directory of your application to contain:
                    ! 
                    ! java.runtime.version = 21"});

            assert_contains!(
                context.run_shell_command("java -version").stderr,
                "openjdk version \"21.0.8\""
            );
        },
    );
}

#[test]
#[ignore = "integration test"]
fn openjdk_functions_default() {
    TestRunner::default().build(
        default_build_config("test-apps/salesforce-functions-app").buildpacks([
            BuildpackReference::CurrentCrate,
            // We need another buildpack to require 'jvm' in the build plan to be able to use a
            // default OpenJDK version. It could be any other buildpack, heroku/maven is just
            // convenient to use here.
            BuildpackReference::WorkspaceBuildpack(buildpack_id!("heroku/maven")),
        ]),
        |context| {
            assert_not_contains!(context.pack_stdout, "No OpenJDK version specified");

            assert_contains!(
                context.run_shell_command("java -version").stderr,
                "openjdk version \"1.8.0_462\""
            );
        },
    );
}

#[test]
#[ignore = "integration test"]
fn openjdk_8() {
    TestRunner::default().build(default_build_config("test-apps/java-8-app"), |context| {
        assert_contains!(
            context.run_shell_command("java -version").stderr,
            "openjdk version \"1.8.0_462\""
        );
    });
}

#[test]
#[ignore = "integration test"]
fn openjdk_11() {
    TestRunner::default().build(default_build_config("test-apps/java-11-app"), |context| {
        assert_contains!(
            context.run_shell_command("java -version").stderr,
            "openjdk version \"11.0.28\""
        );
    });
}

#[test]
#[ignore = "integration test"]
fn openjdk_17() {
    TestRunner::default().build(default_build_config("test-apps/java-17-app"), |context| {
        assert_contains!(
            context.run_shell_command("java -version").stderr,
            "openjdk version \"17.0.16\""
        );
    });
}

#[test]
#[ignore = "integration test"]
fn openjdk_21() {
    TestRunner::default().build(default_build_config("test-apps/java-21-app"), |context| {
        assert_contains!(
            context.run_shell_command("java -version").stderr,
            "openjdk version \"21.0.8\""
        );
    });
}

#[test]
#[ignore = "integration test"]
fn openjdk_24() {
    TestRunner::default().build(default_build_config("test-apps/java-24-app"), |context| {
        assert_contains!(
            context.run_shell_command("java -version").stderr,
            "openjdk version \"24.0.2\""
        );
    });
}
