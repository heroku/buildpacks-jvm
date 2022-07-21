use libcnb_test::{BuildConfig, BuildpackReference, ContainerConfig, TestRunner};
use std::thread;
use std::time::Duration;

#[test]
fn spring_boot_process_type() {
    TestRunner::default().build(
        BuildConfig::new(
            "heroku/buildpacks:20",
            "../../test-fixtures/buildpack-java-spring-boot-test",
        )
        .buildpacks(vec![
            BuildpackReference::Other(String::from("heroku/jvm")),
            BuildpackReference::Crate,
        ]),
        |context| {
            context.start_container(
                ContainerConfig::new().expose_port(8080),
                |container_context| {
                    let addr = container_context.address_for_port(8080).unwrap();
                    let url = format!("http://{}:{}/", addr.ip().to_string(), addr.port());

                    // Give the application a little time to boot up:
                    // https://github.com/heroku/libcnb.rs/issues/280
                    thread::sleep(Duration::from_secs(5));

                    assert_eq!(
                        ureq::get(&url).call().unwrap().into_string().unwrap(),
                        "Hello from Spring Boot!"
                    );
                },
            );
        },
    )
}
