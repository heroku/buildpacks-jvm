//! Smoke tests that ensure a set of basic apps build successfully and the resulting container
//! exposes the HTTP interface of that app as expected. They also re-build the app and assert the
//! resulting container again to ensure that potential caching logic in the buildpack does not
//! break subsequent builds.
//!
//! These tests are strictly happy-path tests and do not assert any output of the buildpack.

use buildpacks_jvm_shared_test::start_container_assert_basic_http_response;
use libcnb_test::{BuildConfig, BuildpackReference, TestRunner};
use std::path::Path;

#[test]
#[ignore = "integration test"]
fn smoke_test_getting_started_guide() {
    smoke_test(
        "heroku/builder:22",
        "test-apps/heroku-java-getting-started",
        "Getting Started with Java on Heroku",
    );
}

fn smoke_test<P>(builder_name: &str, app_dir: P, expected_http_response_body_contains: &str)
where
    P: AsRef<Path>,
{
    let build_config = BuildConfig::new(builder_name, app_dir)
        .buildpacks(vec![
            BuildpackReference::Other(String::from("heroku/jvm")),
            BuildpackReference::Crate,
            BuildpackReference::Other(String::from("heroku/procfile")),
        ])
        .to_owned();

    TestRunner::default().build(&build_config, |context| {
        start_container_assert_basic_http_response(&context, expected_http_response_body_contains);

        context.rebuild(&build_config, |context| {
            start_container_assert_basic_http_response(
                &context,
                expected_http_response_body_contains,
            );
        });
    });
}
