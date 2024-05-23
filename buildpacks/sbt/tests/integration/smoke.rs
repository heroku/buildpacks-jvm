//! Smoke tests that ensure a set of basic apps build successfully and the resulting container
//! exposes the HTTP interface of that app as expected. They also re-build the app and assert the
//! resulting container again to ensure that potential caching logic in the buildpack does not
//! break subsequent builds.
//!
//! These tests are strictly happy-path tests and do not assert any output of the buildpack.

use crate::default_buildpacks;
use buildpacks_jvm_shared_test::{smoke_test, DEFAULT_INTEGRATION_TEST_BUILDER};

#[test]
#[ignore = "integration test"]
fn smoke_test_play_framework_3_0_3() {
    smoke_test(
        DEFAULT_INTEGRATION_TEST_BUILDER,
        "test-apps/play-framework-3.0.3",
        default_buildpacks(),
        "Welcome to Play!",
    );
}

#[test]
#[ignore = "integration test"]
fn smoke_test_sbt_1_8_2_coursier_scala_2_13_10() {
    smoke_test(
        DEFAULT_INTEGRATION_TEST_BUILDER,
        "test-apps/sbt-1.8.2-coursier-scala-2.13.10",
        default_buildpacks(),
        "Hello from Scala!",
    );
}

#[test]
#[ignore = "integration test"]
fn smoke_test_sbt_1_8_2_ivy_scala_2_13_10() {
    smoke_test(
        DEFAULT_INTEGRATION_TEST_BUILDER,
        "test-apps/sbt-1.8.2-ivy-scala-2.13.10",
        default_buildpacks(),
        "Hello from Scala!",
    );
}

#[test]
#[ignore = "integration test"]
fn smoke_test_getting_started_guide() {
    smoke_test(
        DEFAULT_INTEGRATION_TEST_BUILDER,
        "test-apps/heroku-scala-getting-started",
        default_buildpacks(),
        "Getting Started with Scala on Heroku",
    );
}
