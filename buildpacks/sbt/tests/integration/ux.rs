use crate::default_build_config;
use libcnb_test::{assert_contains, assert_not_contains, PackResult, TestRunner};

/// Tests that no confusing or non-actionable warnings caused by the buildpack are shown in the
/// sbt 1.x log during build.
#[test]
#[ignore = "integration test"]
fn test_sbt_1_x_logging() {
    TestRunner::default().build(
        default_build_config("test-apps/sbt-1.8.2-coursier-scala-2.13.10"),
        |context| {
            assert_not_contains!(
                &context.pack_stdout,
                "Executing in batch mode. For better performance use sbt's shell"
            );
        },
    );
}

/// The buildpack requires (unless otherwise configured) that the application build defines a
/// `stage` task. That task is not a default sbt task but is usually added by sbt-native-packager.
///
/// To guide new users that might not be aware that they need a `stage` task, we need to output a
/// descriptive message that explains the issue instead of only relying on sbt telling the user
/// that the `stage` task could not be found.
#[test]
#[ignore = "integration test"]
fn test_missing_stage_task_logging() {
    let build_config = default_build_config("test-apps/sbt-1.8.2-scala-2.13.10-no-native-packager")
        .expected_pack_result(PackResult::Failure)
        .to_owned();

    TestRunner::default().build(&build_config, |context| {
        assert_contains!(&context.pack_stdout, "[error] Not a valid key: stage");

        assert_contains!(
            &context.pack_stderr,
            "It looks like your build.sbt does not have a valid 'stage' task."
        );
    });
}
