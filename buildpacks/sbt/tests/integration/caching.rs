//! Tests that ensure subsequent builds use cached dependencies of various kinds, speeding up
//! builds.

use crate::default_buildpacks;
use buildpacks_jvm_shared_test::DEFAULT_INTEGRATION_TEST_BUILDER;
use libcnb_test::{assert_contains, assert_not_contains, BuildConfig, TestRunner};

#[test]
#[ignore = "integration test"]
fn test_caching_sbt_1_8_2_coursier() {
    let build_config = BuildConfig::new(
        DEFAULT_INTEGRATION_TEST_BUILDER,
        "test-apps/sbt-1.8.2-coursier-scala-2.13.10",
    )
    .buildpacks(default_buildpacks())
    .to_owned();

    TestRunner::default().build(&build_config, |context| {
        assert_contains!(&context.pack_stderr, "Downloading sbt launcher for 1.8.2:");
        assert_contains!(
            &context.pack_stderr,
            "[info] [launcher] getting org.scala-sbt sbt 1.8.2  (this may take some time)..."
        );
        assert_contains!(
            &context.pack_stderr,
            "[info] [launcher] getting Scala 2.12.17 (for sbt)..."
        );
        assert_contains!(
            &context.pack_stdout,
            "[info] Non-compiled module 'compiler-bridge_2.12' for Scala 2.12.17. Compiling..."
        );
        assert_contains!(
            &context.pack_stdout,
            "[info] Non-compiled module 'compiler-bridge_2.13' for Scala 2.13.10. Compiling..."
        );

        context.rebuild(&build_config, |context| {
            assert_not_contains!(&context.pack_stderr, "Downloading sbt launcher for 1.8.2:");
            assert_not_contains!(
                &context.pack_stderr,
                "[info] [launcher] getting org.scala-sbt sbt 1.8.2  (this may take some time)..."
            );
            assert_not_contains!(
                &context.pack_stderr,
                "[info] [launcher] getting Scala 2.12.17 (for sbt)..."
            );
            assert_not_contains!(
                &context.pack_stdout,
                "[info] Non-compiled module 'compiler-bridge_2.12' for Scala 2.12.17. Compiling..."
            );
            assert_not_contains!(
                &context.pack_stdout,
                "[info] Non-compiled module 'compiler-bridge_2.13' for Scala 2.13.10. Compiling..."
            );
        });
    });
}

#[test]
#[ignore = "integration test"]
fn test_caching_sbt_1_8_2_ivy() {
    let build_config = BuildConfig::new(
        DEFAULT_INTEGRATION_TEST_BUILDER,
        "test-apps/sbt-1.8.2-ivy-scala-2.13.10",
    )
    .buildpacks(default_buildpacks())
    .to_owned();

    let dependency_download_lines = [
        "[info] downloading https://repo1.maven.org/maven2/org/scala-lang/scala-library/2.13.10/scala-library-2.13.10.jar ...",
        "[info] downloading https://repo1.maven.org/maven2/com/twitter/finagle-toggle_2.13/22.12.0/finagle-toggle_2.13-22.12.0.jar ...",
        "[info] downloading https://repo1.maven.org/maven2/com/twitter/finagle-http_2.13/22.12.0/finagle-http_2.13-22.12.0.jar ...",
        "[info] downloading https://repo1.maven.org/maven2/com/twitter/finagle-base-http_2.13/22.12.0/finagle-base-http_2.13-22.12.0.jar ...",
        "[info] downloading https://repo1.maven.org/maven2/com/twitter/finagle-netty4-http_2.13/22.12.0/finagle-netty4-http_2.13-22.12.0.jar ...",
        "[info] downloading https://repo1.maven.org/maven2/com/twitter/finagle-http2_2.13/22.12.0/finagle-http2_2.13-22.12.0.jar ..."
    ];

    TestRunner::default().build(&build_config, |context| {
        assert_contains!(&context.pack_stderr, "Downloading sbt launcher for 1.8.2:");
        assert_contains!(
            &context.pack_stderr,
            "[info] [launcher] getting org.scala-sbt sbt 1.8.2  (this may take some time)..."
        );
        assert_contains!(
            &context.pack_stderr,
            "[info] [launcher] getting Scala 2.12.17 (for sbt)..."
        );
        assert_contains!(
            &context.pack_stdout,
            "[info] Non-compiled module 'compiler-bridge_2.12' for Scala 2.12.17. Compiling..."
        );
        assert_contains!(
            &context.pack_stdout,
            "[info] Non-compiled module 'compiler-bridge_2.13' for Scala 2.13.10. Compiling..."
        );

        for dependency_download_line in dependency_download_lines {
            assert_contains!(&context.pack_stdout, dependency_download_line);
        }

        context.rebuild(&build_config, |context| {
            assert_not_contains!(&context.pack_stderr, "Downloading sbt launcher for 1.8.2:");
            assert_not_contains!(
                &context.pack_stderr,
                "[info] [launcher] getting org.scala-sbt sbt 1.8.2  (this may take some time)..."
            );
            assert_not_contains!(
                &context.pack_stderr,
                "[info] [launcher] getting Scala 2.12.17 (for sbt)..."
            );
            assert_not_contains!(
                &context.pack_stdout,
                "[info] Non-compiled module 'compiler-bridge_2.12' for Scala 2.12.17. Compiling..."
            );
            assert_not_contains!(
                &context.pack_stdout,
                "[info] Non-compiled module 'compiler-bridge_2.13' for Scala 2.13.10. Compiling..."
            );

            for dependency_download_line in dependency_download_lines {
                assert_not_contains!(&context.pack_stdout, dependency_download_line);
            }
        });
    });
}
