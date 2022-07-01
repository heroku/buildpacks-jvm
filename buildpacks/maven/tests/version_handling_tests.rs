use libcnb_test::{
    assert_contains, assert_not_contains, BuildpackReference, PackResult, TestConfig, TestRunner,
};
use std::fs::OpenOptions;
use std::path::Path;

#[test]
fn with_wrapper() {
    TestRunner::default().run_test(default_config(), |context| {
            assert_not_contains!(context.pack_stdout, "Selected Maven version:");
            assert_contains!(context.pack_stdout, "Maven wrapper detected, skipping installation.");
            assert_contains!(context.pack_stdout, "$ ./mvnw");
            assert_contains!(context.pack_stdout, &format!("[BUILDPACK INTEGRATION TEST - MAVEN VERSION] {SIMPLE_HTTP_SERVICE_MAVEN_WRAPPER_VERSION}"));
        })
}

#[test]
fn with_wrapper_and_system_properties() {
    TestRunner::default().run_test(
        default_config().app_dir_preprocessor(|path| {
            remove_maven_wrapper(&path);
            set_maven_version_app_dir_preprocessor(PREVIOUS_MAVEN_VERSION, &path)
        }),
        |context| {
            assert_contains!(
                context.pack_stdout,
                &format!("Selected Maven version: {PREVIOUS_MAVEN_VERSION}")
            );
            assert_not_contains!(context.pack_stdout, "$ ./mvnw");
            assert_contains!(
                context.pack_stdout,
                &format!("[BUILDPACK INTEGRATION TEST - MAVEN VERSION] {PREVIOUS_MAVEN_VERSION}")
            );
        },
    )
}

#[test]
fn with_wrapper_and_unknown_system_properties() {
    TestRunner::default().run_test(
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
fn without_wrapper_and_without_system_properties() {
    TestRunner::default().run_test(
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
fn without_wrapper_and_unknown_system_properties() {
    TestRunner::default().run_test(
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
fn without_wrapper_and_maven_3_6_2_system_properties() {
    TestRunner::default().run_test(
        default_config().app_dir_preprocessor(|path| {
            remove_maven_wrapper(&path);
            set_maven_version_app_dir_preprocessor("3.6.2", &path);
        }),
        |context| {
            assert_contains!(context.pack_stdout, "Selected Maven version: 3.6.2");
            assert_contains!(
                context.pack_stdout,
                "[BUILDPACK INTEGRATION TEST - MAVEN VERSION] 3.6.2"
            );
        },
    )
}

#[test]
fn without_wrapper_and_maven_3_5_4_system_properties() {
    TestRunner::default().run_test(
        default_config().app_dir_preprocessor(|path| {
            remove_maven_wrapper(&path);
            set_maven_version_app_dir_preprocessor("3.5.4", &path);
        }),
        |context| {
            assert_contains!(context.pack_stdout, "Selected Maven version: 3.5.4");
            assert_contains!(
                context.pack_stdout,
                "[BUILDPACK INTEGRATION TEST - MAVEN VERSION] 3.5.4"
            );
        },
    )
}

#[test]
fn without_wrapper_and_maven_3_3_9_system_properties() {
    TestRunner::default().run_test(
        default_config().app_dir_preprocessor(|path| {
            remove_maven_wrapper(&path);
            set_maven_version_app_dir_preprocessor("3.3.9", &path);
        }),
        |context| {
            assert_contains!(context.pack_stdout, "Selected Maven version: 3.3.9");
            assert_contains!(
                context.pack_stdout,
                "[BUILDPACK INTEGRATION TEST - MAVEN VERSION] 3.3.9"
            );
        },
    )
}

#[test]
fn without_wrapper_and_maven_3_2_5_system_properties() {
    TestRunner::default().run_test(
        default_config().app_dir_preprocessor(|path| {
            remove_maven_wrapper(&path);
            set_maven_version_app_dir_preprocessor("3.2.5", &path);
        }),
        |context| {
            assert_contains!(context.pack_stdout, "Selected Maven version: 3.2.5");
            assert_contains!(
                context.pack_stdout,
                "[BUILDPACK INTEGRATION TEST - MAVEN VERSION] 3.2.5"
            );
        },
    )
}

fn default_config() -> TestConfig {
    TestConfig::new(
        "heroku/buildpacks:20",
        "../../test-fixtures/simple-http-service",
    )
    .buildpacks(vec![
        BuildpackReference::Other(String::from("heroku/jvm")),
        BuildpackReference::Crate,
    ])
    .to_owned()
}

fn remove_maven_wrapper(path: &Path) {
    std::fs::remove_file(path.join("mvnw")).unwrap()
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
    properties.insert(String::from("maven.version"), version.clone());
    java_properties::write(&mut properties_file, &properties).unwrap();
}

const DEFAULT_MAVEN_VERSION: &str = "3.6.2";
const PREVIOUS_MAVEN_VERSION: &str = "3.5.4";
const UNKNOWN_MAVEN_VERSION: &str = "1.0.0-unknown-version";
const SIMPLE_HTTP_SERVICE_MAVEN_WRAPPER_VERSION: &str = "3.6.3";
