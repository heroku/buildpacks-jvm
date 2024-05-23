use crate::default_build_config;
use buildpacks_jvm_shared_test::{
    http_request_backoff, UREQ_RESPONSE_AS_STRING_EXPECT_MESSAGE,
    UREQ_RESPONSE_RESULT_EXPECT_MESSAGE,
};
use libcnb_test::{assert_contains, assert_not_contains, ContainerConfig, TestRunner};

/// Users can request to have sbt and all caches to be available at launch. One use-case for this
/// is not using native-packager and wanting to rely on `sbt run` to run the application in prod.
///
/// This test uses an app that is deployed this way and configures the buildpack accordingly.
#[test]
#[ignore = "integration test"]
fn test_the_thing() {
    let build_config = default_build_config("test-apps/sbt-1.8.2-scala-2.13.10-no-native-packager")
        .env("SBT_TASKS", "compile")
        .env("SBT_AVAILABLE_AT_LAUNCH", "true")
        .to_owned();

    TestRunner::default().build(&build_config, |context| {
        context.start_container(
            ContainerConfig::default()
                .expose_port(PORT)
                .env("PORT", PORT.to_string()),
            |context| {
                let addr = context.address_for_port(PORT);

                let response = http_request_backoff(|| ureq::get(&format!("http://{addr}")).call())
                    .expect(UREQ_RESPONSE_RESULT_EXPECT_MESSAGE)
                    .into_string()
                    .expect(UREQ_RESPONSE_AS_STRING_EXPECT_MESSAGE);

                assert_contains!(&response, "Hello from Scala!");

                // The caches written during the build for sbt, Scala and application dependencies
                // are expected to be available at launch as well to avoid duplicate downloads and
                // compilation:

                let logs = context.logs_now();
                assert_not_contains!(
                        &logs.stderr,
                        "[info] [launcher] getting org.scala-sbt sbt 1.8.2  (this may take some time)..."
                    );
                assert_not_contains!(
                        &logs.stderr,
                        "[info] [launcher] getting Scala 2.12.17 (for sbt)..."
                    );
                assert_not_contains!(
                        &logs.stdout,
                        "[info] Non-compiled module 'compiler-bridge_2.12' for Scala 2.12.17. Compiling..."
                    );
                assert_not_contains!(
                        &logs.stdout,
                        "[info] Non-compiled module 'compiler-bridge_2.13' for Scala 2.13.10. Compiling..."
                    );

                assert_not_contains!(&logs.stderr, "Downloading sbt launcher for 1.8.2:");
            },
        );
    });
}

const PORT: u16 = 8080;
