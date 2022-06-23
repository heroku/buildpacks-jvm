use indoc::formatdoc;
use libcnb_test::{
    assert_contains, assert_not_contains, BuildpackReference, PackResult, TestConfig, TestRunner,
};
use std::fs;
use std::path::PathBuf;

#[test]
fn maven_settings_url_success() {
    TestRunner::default().run_test(
        default_config().env("MAVEN_SETTINGS_URL", SETTINGS_XML_URL),
        |context| {
            assert_contains!(
                context.pack_stdout,
                &format!(
                    "[BUILDPACK INTEGRATION TEST - SETTINGS TEST VALUE] {SETTINGS_XML_URL_VALUE}"
                )
            );
        },
    );
}

#[test]
fn maven_settings_url_failure() {
    TestRunner::default().run_test(
            default_config()
                .env("MAVEN_SETTINGS_URL", SETTINGS_XML_URL_404)
                .expected_pack_result(PackResult::Failure),
            |context| {
                assert_contains!(
                    context.pack_stderr,
                    &format!("You have set MAVEN_SETTINGS_URL to \"{SETTINGS_XML_URL_404}\". We tried to download the file at this\nURL, but the download failed. Please verify that the given URL is correct and try again.")
                );

                // This error message comes from Maven itself. We don't expect Maven to to be executed at all.
                assert_not_contains!(context.pack_stdout, "[INFO] BUILD FAILURE");
            },
        );
}

#[test]
fn maven_settings_path() {
    let settings_xml_filename = "forgreatjustice.xml";
    let settings_xml_test_value = "Take off every 'ZIG'!!";

    TestRunner::default().run_test(
        default_config()
            .app_dir_preprocessor(|dir| {
                write_settings_xml(
                    dir.join(settings_xml_filename.to_owned()),
                    settings_xml_test_value,
                )
            })
            .env("MAVEN_SETTINGS_PATH", settings_xml_filename),
        |context| {
            assert_contains!(
                context.pack_stdout,
                &format!(
                    "[BUILDPACK INTEGRATION TEST - SETTINGS TEST VALUE] {settings_xml_test_value}"
                )
            );
        },
    );
}

#[test]
fn maven_settings_path_and_settings_url() {
    let settings_xml_filename = "zerowing.xml";
    let settings_xml_test_value = "We get signal.";

    TestRunner::default().run_test(
        default_config()
            .app_dir_preprocessor(|dir| {
                write_settings_xml(
                    dir.join(settings_xml_filename.to_owned()),
                    settings_xml_test_value,
                )
            })
            .env("MAVEN_SETTINGS_PATH", settings_xml_filename)
            .env("MAVEN_SETTINGS_URL", SETTINGS_XML_URL),
        |context| {
            // Assert MAVEN_SETTINGS_PATH takes precedence
            assert_contains!(
                context.pack_stdout,
                &format!(
                    "[BUILDPACK INTEGRATION TEST - SETTINGS TEST VALUE] {settings_xml_test_value}"
                )
            );
        },
    );
}

#[test]
fn maven_settings_xml_in_app_root() {
    let settings_xml_filename = "settings.xml";
    let settings_xml_test_value = "Somebody set up us the bomb.";

    TestRunner::default().run_test(
        // Note that there is no MAVEN_SETTINGS_PATH here
        default_config().app_dir_preprocessor(|dir| {
            write_settings_xml(
                dir.join(settings_xml_filename.to_owned()),
                settings_xml_test_value,
            )
        }),
        |context| {
            assert_contains!(
                context.pack_stdout,
                &format!(
                    "[BUILDPACK INTEGRATION TEST - SETTINGS TEST VALUE] {settings_xml_test_value}"
                )
            );
        },
    );
}

#[test]
fn maven_settings_xml_in_app_root_and_explicit_settings_path() {
    let settings_xml_filename = "settings.xml";
    let settings_xml_test_value = "Somebody set up us the bomb.";
    let zero_wing_filename = "zerowing.xml";
    let zero_wing_test_value = "How are you gentlemen !!";

    TestRunner::default().run_test(
        // Note that there is no MAVEN_SETTINGS_PATH here
        default_config()
            .app_dir_preprocessor(|dir| {
                write_settings_xml(
                    dir.join(settings_xml_filename.to_owned()),
                    settings_xml_test_value,
                );

                write_settings_xml(
                    dir.join(zero_wing_filename.to_owned()),
                    zero_wing_test_value,
                );
            })
            .env("MAVEN_SETTINGS_PATH", zero_wing_filename),
        |context| {
            assert_contains!(
                context.pack_stdout,
                &format!(
                    "[BUILDPACK INTEGRATION TEST - SETTINGS TEST VALUE] {zero_wing_test_value}"
                )
            );
        },
    );
}

#[test]
fn maven_settings_xml_in_app_root_and_explicit_settings_url() {
    let settings_xml_filename = "settings.xml";
    let settings_xml_test_value = "Somebody set up us the bomb.";

    TestRunner::default().run_test(
        // Note that there is no MAVEN_SETTINGS_PATH here
        default_config()
            .app_dir_preprocessor(|dir| {
                write_settings_xml(
                    dir.join(settings_xml_filename.to_owned()),
                    settings_xml_test_value,
                );
            })
            .env("MAVEN_SETTINGS_URL", SETTINGS_XML_URL),
        |context| {
            assert_contains!(
                context.pack_stdout,
                &format!(
                    "[BUILDPACK INTEGRATION TEST - SETTINGS TEST VALUE] {SETTINGS_XML_URL_VALUE}"
                )
            );
        },
    );
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
    .clone()
}

fn write_settings_xml(path: PathBuf, test_value: &str) {
    fs::write(
            path,
            formatdoc! {"
                <settings xmlns=\"http://maven.apache.org/SETTINGS/1.0.0\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\"
                  xsi:schemaLocation=\"http://maven.apache.org/SETTINGS/1.0.0 https://maven.apache.org/xsd/settings-1.0.0.xsd\">

                  <profiles>
                      <profile>
                          <activation>
                              <activeByDefault>true</activeByDefault>
                          </activation>
                          <properties>
                              <heroku.maven.settings-test.value>{test_value}</heroku.maven.settings-test.value>
                          </properties>
                      </profile>
                  </profiles>
                </settings>
            ", test_value = test_value},
        ).unwrap();
}

const SETTINGS_XML_URL: &str = "https://gist.githubusercontent.com/Malax/d47323823a3d59249cbb5593c4f1b764/raw/83f196719d2c4d56aec6720964ba7d7c86b71727/download-settings.xml";
const SETTINGS_XML_URL_VALUE: &str = "Main screen turn on.";
const SETTINGS_XML_URL_404: &str = "https://gist.githubusercontent.com/Malax/settings.xml";
