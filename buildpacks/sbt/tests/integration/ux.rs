use crate::default_buildpacks;
use buildpacks_jvm_shared_test::DEFAULT_INTEGRATION_TEST_BUILDER;
use libcnb_test::{assert_contains, assert_not_contains, BuildConfig, PackResult, TestRunner};

/// Tests that no confusing or non-actionable warnings caused by the buildpack are shown in the
/// sbt 1.x log during build.
#[test]
#[ignore = "integration test"]
fn test_sbt_1_x_logging() {
    let build_config = BuildConfig::new(
        DEFAULT_INTEGRATION_TEST_BUILDER,
        "test-apps/sbt-1.8.2-coursier-scala-2.13.10",
    )
    .buildpacks(default_buildpacks())
    .to_owned();

    TestRunner::default().build(&build_config, |context| {
        assert_not_contains!(
            &context.pack_stdout,
            "Executing in batch mode. For better performance use sbt's shell"
        );
    });
}

#[test]
#[ignore = "integration test"]
fn test_missing_stage_task_logging() {
    let build_config = BuildConfig::new(
        DEFAULT_INTEGRATION_TEST_BUILDER,
        "test-apps/sbt-1.8.2-scala-2.13.10-no-native-packager",
    )
    .buildpacks(default_buildpacks())
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
