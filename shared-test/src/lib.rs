// Enable rustc and Clippy lints that are disabled by default.
// https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html#unused-crate-dependencies
#![warn(unused_crate_dependencies)]
// https://rust-lang.github.io/rust-clippy/stable/index.html
#![warn(clippy::pedantic)]
// This lint is too noisy and enforces a style that reduces readability in many cases.
#![allow(clippy::module_name_repetitions)]

use libcnb_test::{assert_contains, ContainerConfig, TestContext};
use std::time::Duration;

/// Extremely opinionated helper for testing containers that expose a HTTP interface.
///
/// It will start the container from a `TestContext`, sets the `PORT` environment variable and tries
/// to get a successful (HTTP 200) response from the container for a `GET` request to `/`. It will
/// then assert that the given string is contained in the request body.
///
/// This helper will retry failed requests with an exponential backoff to avoid flappy tests.
///
/// The JVM buildpacks all have smoke integration tests that need to ensure the container runs as
/// expected. This function is catering to that use-case and is not useful in different contexts.
#[allow(clippy::missing_panics_doc)]
pub fn start_container_assert_basic_http_response(
    context: &TestContext,
    expected_http_response_body_contains: &str,
) {
    context.start_container(
        ContainerConfig::default()
            .expose_port(PORT)
            .env("PORT", PORT.to_string()),
        |context| {
            let url = format!(
                "http://{}",
                context
                    .address_for_port(PORT)
                    .expect("address for container port should be available from libcnb-test")
            );

            let backoff = exponential_backoff::Backoff::new(
                32,
                Duration::from_secs(10),
                Duration::from_secs(5 * 60),
            );

            let mut last_response = None;
            for delay in backoff.iter() {
                std::thread::sleep(delay);

                match ureq::get(&url).call() {
                    Err(_) => continue,
                    Ok(response) => {
                        last_response = Some(response);
                        break;
                    }
                }
            }

            let response_body = last_response
                .and_then(|response| response.into_string().ok())
                .expect("response body should be available");

            assert_contains!(&response_body, expected_http_response_body_contains);
        },
    );
}

const PORT: u16 = 8080;
