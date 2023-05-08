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
fn smoke_test_getting_started_guide() {
    smoke_test(
        DEFAULT_INTEGRATION_TEST_BUILDER,
        "test-apps/heroku-java-getting-started",
        default_buildpacks(),
        "Getting Started with Java on Heroku",
    );
}
