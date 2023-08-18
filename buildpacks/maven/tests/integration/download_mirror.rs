use crate::{default_config, remove_maven_wrapper};
use libcnb_test::{assert_contains, PackResult, TestRunner};

#[test]
#[ignore = "integration test"]
fn download_mirror() {
    TestRunner::default().build(
        default_config()
            .env("MAVEN_DOWNLOAD_MIRROR", "https://repo1.maven.org/maven2/")
            .app_dir_preprocessor(|path| {
                remove_maven_wrapper(&path);
            }),
        |_| {
            // Intentionally empty, we only care that it builds.
        },
    )
}

#[test]
#[ignore = "integration test"]
fn invalid_url() {
    TestRunner::default().build(
        default_config()
            .env("MAVEN_DOWNLOAD_MIRROR", "Not a valid base URL")
            .app_dir_preprocessor(|path| {
                remove_maven_wrapper(&path);
            })
            .expected_pack_result(PackResult::Failure),
        |context| {
            assert_contains!(
                context.pack_stderr,
                "[Error: Invalid Maven Download Mirror]"
            );
        },
    )
}
