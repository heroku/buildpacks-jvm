use libcnb_test::{assert_contains, BuildConfig, ContainerConfig, TestContext, TestRunner};
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
            let url = format!("http://{}", context.address_for_port(PORT));

            let response_body = http_request_backoff(|| ureq::get(&url).call())
                .expect(UREQ_RESPONSE_RESULT_EXPECT_MESSAGE)
                .into_string()
                .expect(UREQ_RESPONSE_AS_STRING_EXPECT_MESSAGE);

            assert_contains!(&response_body, expected_http_response_body_contains);
        },
    );
}

pub fn http_request_backoff<F, T, E>(request_fn: F) -> Result<T, E>
where
    F: Fn() -> Result<T, E>,
{
    let backoff =
        exponential_backoff::Backoff::new(32, Duration::from_secs(1), Duration::from_secs(5 * 60));

    let mut backoff_durations = backoff.into_iter();

    loop {
        match request_fn() {
            result @ Ok(_) => return result,
            result @ Err(_) => match backoff_durations.next() {
                None | Some(None) => return result,
                Some(Some(backoff_duration)) => {
                    std::thread::sleep(backoff_duration);
                }
            },
        }
    }
}

/// Opinionated helper for smoke-testing JVM buildpacks.
///
/// Builds the app with the given build config, asserts that the build finished successfully and
/// builds the app again to ensure that any caching logic does not break subsequent builds. After
/// each build, an HTTP request is made to the resulting container, asserting that the given string
/// is present in the response.
pub fn smoke_test(build_config: &BuildConfig, expected_http_response_body_contains: &str) {
    TestRunner::default().build(build_config, |context| {
        start_container_assert_basic_http_response(&context, expected_http_response_body_contains);

        context.rebuild(build_config, |context| {
            start_container_assert_basic_http_response(
                &context,
                expected_http_response_body_contains,
            );
        });
    });
}

pub const UREQ_RESPONSE_RESULT_EXPECT_MESSAGE: &str = "http request should be successful";

pub const UREQ_RESPONSE_AS_STRING_EXPECT_MESSAGE: &str =
    "http response body should be convertable to a string";

const PORT: u16 = 8080;
