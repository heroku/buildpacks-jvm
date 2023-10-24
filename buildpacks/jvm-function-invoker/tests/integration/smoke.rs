use base64::Engine;
use buildpacks_jvm_shared_test::{
    DEFAULT_INTEGRATION_TEST_BUILDER, UREQ_RESPONSE_AS_STRING_EXPECT_MESSAGE,
    UREQ_RESPONSE_RESULT_EXPECT_MESSAGE,
};
use libcnb_test::{BuildConfig, BuildpackReference, ContainerConfig, TestRunner};
use std::time::Duration;

#[test]
#[ignore = "integration test"]
fn smoke_test_simple_function() {
    TestRunner::default().build(
        BuildConfig::new(DEFAULT_INTEGRATION_TEST_BUILDER, "test-apps/simple-function").buildpacks([
            BuildpackReference::Other(String::from("heroku/jvm")),
            BuildpackReference::Other(String::from("heroku/maven")),
            BuildpackReference::CurrentCrate,
        ]),
        |context| {
            context.start_container(
                ContainerConfig::new()
                    .env("PORT", PORT.to_string())
                    .expose_port(PORT),
                |container| {
                    // Give the function a little time to boot
                    std::thread::sleep(Duration::from_secs(10));

                    let request_payload = "\"All those moments will be lost in time, like tears in rain...\"";

                    // Absolute minimum request that can be served by the function runtime.
                    let response_payload = ureq::post(&format!("http://{}", container.address_for_port(PORT)))
                        .set("Content-Type", "application/json")
                        .set("Authorization", "")
                        .set("ce-id", "function")
                        .set("ce-time", "2020-09-03T20:56:28.297915Z")
                        .set("ce-type", "")
                        .set("ce-source", "")
                        .set("ce-specversion", "1.0")
                        .set("ce-sfcontext", &base64::engine::general_purpose::STANDARD.encode(r#"{ "apiVersion": "", "payloadVersion": "", "userContext": { "orgId": "", "userId": "", "username": "", "orgDomainUrl": "", "onBehalfOfUserId": null, "salesforceBaseUrl": "" } }"#))
                        .set("ce-sffncontext", &base64::engine::general_purpose::STANDARD.encode(r#"{ "resource": "", "requestId": "", "accessToken": "", "apexClassId": null, "apexClassFQN": null, "functionName": "", "functionInvocationId": null }"#))
                        .send_string(request_payload)
                        .expect(UREQ_RESPONSE_RESULT_EXPECT_MESSAGE)
                        .into_string()
                        .expect(UREQ_RESPONSE_AS_STRING_EXPECT_MESSAGE);

                    assert_eq!(response_payload, request_payload.chars().rev().collect::<String>());
                },
            );
        },
    )
}

const PORT: u16 = 8080;
