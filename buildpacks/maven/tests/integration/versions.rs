use crate::{default_config, remove_maven_wrapper};
use libcnb_test::{assert_contains, assert_not_contains, PackResult, TestRunner};
use std::fs::OpenOptions;
use std::path::Path;

#[test]
#[ignore = "integration test"]
fn with_wrapper() {
    TestRunner::default().build(default_config(), |context| {
            assert_not_contains!(context.pack_stdout, "Selected Maven version:");
            assert_contains!(context.pack_stdout, "Maven wrapper detected, skipping installation.");
            assert_contains!(context.pack_stdout, "$ ./mvnw");
            assert_contains!(context.pack_stdout, &format!("[BUILDPACK INTEGRATION TEST - MAVEN VERSION] {SIMPLE_HTTP_SERVICE_MAVEN_WRAPPER_VERSION}"));
        })
}

#[test]
#[ignore = "integration test"]
fn with_wrapper_and_system_properties() {
    TestRunner::default().build(
        default_config().app_dir_preprocessor(|path| {
            remove_maven_wrapper(&path);
            set_maven_version_app_dir_preprocessor(DEFAULT_MAVEN_VERSION, &path)
        }),
        |context| {
            assert_contains!(
                context.pack_stdout,
                &format!("Selected Maven version: {DEFAULT_MAVEN_VERSION}")
            );
            assert_not_contains!(context.pack_stdout, "$ ./mvnw");
            assert_contains!(
                context.pack_stdout,
                &format!("[BUILDPACK INTEGRATION TEST - MAVEN VERSION] {DEFAULT_MAVEN_VERSION}")
            );
        },
    )
}

#[test]
#[ignore = "integration test"]
fn with_wrapper_and_unknown_system_properties() {
    TestRunner::default().build(
            default_config().app_dir_preprocessor(|path| set_maven_version_app_dir_preprocessor(
                UNKNOWN_MAVEN_VERSION, &path
            )).expected_pack_result(PackResult::Failure),
            |context| {
                assert_contains!(context.pack_stderr, "[Error: Unsupported Maven version]");
                assert_contains!(context.pack_stderr, &format!("You have defined an unsupported Maven version ({UNKNOWN_MAVEN_VERSION}) in the system.properties file."));
            },
        )
}

#[test]
#[ignore = "integration test"]
fn without_wrapper_and_without_system_properties() {
    TestRunner::default().build(
        default_config().app_dir_preprocessor(|path| {
            remove_maven_wrapper(&path);
        }),
        |context| {
            assert_not_contains!(context.pack_stdout, "$ ./mvnw");
            assert_contains!(
                context.pack_stdout,
                &format!("Selected Maven version: {DEFAULT_MAVEN_VERSION}")
            );
            assert_contains!(
                context.pack_stdout,
                &format!("[BUILDPACK INTEGRATION TEST - MAVEN VERSION] {DEFAULT_MAVEN_VERSION}")
            );
        },
    )
}

#[test]
#[ignore = "integration test"]
fn without_wrapper_and_unknown_system_properties() {
    TestRunner::default().build(
            default_config().app_dir_preprocessor(|path| {
                remove_maven_wrapper(&path);
                set_maven_version_app_dir_preprocessor(UNKNOWN_MAVEN_VERSION, &path);
            }).expected_pack_result(PackResult::Failure),
            |context| {
                assert_contains!(context.pack_stderr, "[Error: Unsupported Maven version]");
                assert_contains!(context.pack_stderr, &format!("You have defined an unsupported Maven version ({UNKNOWN_MAVEN_VERSION}) in the system.properties file."));
            },
        )
}

#[test]
#[ignore = "integration test"]
fn without_wrapper_and_maven_3_9_4_system_properties() {
    TestRunner::default().build(
        default_config().app_dir_preprocessor(|path| {
            remove_maven_wrapper(&path);
            set_maven_version_app_dir_preprocessor("3.9.4", &path);
        }),
        |context| {
            assert_contains!(context.pack_stdout, "Selected Maven version: 3.9.4");
            assert_contains!(
                context.pack_stdout,
                "[BUILDPACK INTEGRATION TEST - MAVEN VERSION] 3.9.4"
            );
        },
    )
}

fn set_maven_version_app_dir_preprocessor(version: &str, path: &Path) {
    let version = version.to_string();

    let mut properties_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path.join("system.properties"))
        .unwrap();

    let mut properties = java_properties::read(&mut properties_file).unwrap();
    properties.insert(String::from("maven.version"), version);
    java_properties::write(&mut properties_file, &properties).unwrap();
}

const DEFAULT_MAVEN_VERSION: &str = "3.9.4";
const UNKNOWN_MAVEN_VERSION: &str = "1.0.0-unknown-version";
const SIMPLE_HTTP_SERVICE_MAVEN_WRAPPER_VERSION: &str = "3.6.3";
