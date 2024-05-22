use crate::default_build_config;
use buildpacks_jvm_shared::system_properties::write_system_properties;
use libcnb_test::{assert_contains, PackResult, TestRunner};
use std::collections::HashMap;

#[test]
#[ignore = "integration test"]
fn test_unsupported_java_version() {
    let build_config = default_build_config("test-apps/heroku-gradle-getting-started")
        .expected_pack_result(PackResult::Failure)
        .app_dir_preprocessor(|dir| {
            write_system_properties(
                &dir,
                &HashMap::from([(String::from("java.runtime.version"), String::from("7"))]),
            )
            .unwrap();
        })
        .to_owned();

    TestRunner::default().build(&build_config, |context| {
        assert_contains!(
            context.pack_stderr,
            "Gradle 7.6.1 requires Java 1.8 or later to run. You are currently using Java 1.7."
        );
    });
}
