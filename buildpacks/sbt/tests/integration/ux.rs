use libcnb_test::{assert_not_contains, BuildConfig, BuildpackReference, TestRunner};

/// Tests that no confusing or non-actionable warnings caused by the buildpack are shown in the
/// sbt 1.x log during build.
#[test]
#[ignore = "integration test"]
fn test_sbt_1_x_logging() {
    let build_config = BuildConfig::new(
        "heroku/builder:22",
        "test-apps/sbt-1.8.2-coursier-scala-2.13.10",
    )
    .buildpacks(vec![
        BuildpackReference::Other(String::from("heroku/jvm")),
        BuildpackReference::Crate,
        BuildpackReference::Other(String::from("heroku/procfile")),
    ])
    .to_owned();

    TestRunner::default().build(&build_config, |context| {
        assert_not_contains!(
            &context.pack_stdout,
            "Executing in batch mode. For better performance use sbt's shell"
        );
    });
}
