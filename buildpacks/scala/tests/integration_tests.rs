use libcnb_test::{
    assert_contains, assert_not_contains, BuildConfig, BuildpackReference, ContainerConfig,
    TestContext, TestRunner,
};
use std::path::Path;
use std::thread;
use std::time::Duration;

#[test]
#[ignore = "integration test"]
fn test_scala_application_with_ivy() {
    test_scala_application("scala-app-using-ivy", |ctx| {
        assert_health_check_responds(&ctx)
    });
}

#[test]
#[ignore = "integration test"]
fn test_scala_application_with_ivy_uses_cache_on_rebuild() {
    test_scala_application("scala-app-using-ivy", |ctx| {
        assert_contains!(&ctx.pack_stdout, "Setting up sbt");
        assert_not_contains!(&ctx.pack_stdout, "Reusing sbt");
        ctx.rebuild(get_build_config("scala-app-using-ivy"), |rebuild_ctx| {
            assert_contains!(&rebuild_ctx.pack_stdout, "Reusing sbt");
            assert_not_contains!(&rebuild_ctx.pack_stdout, "Setting up sbt");
            assert_health_check_responds(&rebuild_ctx);
        })
    })
}

#[test]
#[ignore = "integration test"]
fn test_scala_application_with_coursier() {
    test_scala_application("scala-app-using-coursier", |ctx| {
        assert_health_check_responds(&ctx)
    });
}

#[test]
#[ignore = "integration test"]
fn test_scala_application_with_coursier_uses_cache_on_rebuild() {
    test_scala_application("scala-app-using-coursier", |ctx| {
        assert_contains!(&ctx.pack_stdout, "Setting up sbt");
        assert_not_contains!(&ctx.pack_stdout, "Reusing sbt");
        ctx.rebuild(
            get_build_config("scala-app-using-coursier"),
            |rebuild_ctx| {
                assert_contains!(&rebuild_ctx.pack_stdout, "Reusing sbt");
                assert_not_contains!(&rebuild_ctx.pack_stdout, "Setting up sbt");
                assert_health_check_responds(&rebuild_ctx);
            },
        )
    })
}

#[test]
#[ignore = "integration test"]
fn test_play_support_for_v2_8() {
    test_scala_application("scala-play-app-2.8", |ctx| {
        assert_health_check_responds(&ctx);
    })
}

#[test]
#[ignore = "integration test"]
fn test_play_support_for_v2_7() {
    test_scala_application("scala-play-app-2.7", |ctx| {
        assert_health_check_responds(&ctx);
    })
}

fn test_scala_application(fixture_name: &str, test_body: fn(TestContext)) {
    TestRunner::default().build(get_build_config(fixture_name), test_body);
}

fn get_build_config(fixture_name: &str) -> BuildConfig {
    let app_dir = Path::new("../../test-fixtures").join(fixture_name);
    let builder_name =
        std::env::var("INTEGRATION_TEST_CNB_BUILDER").unwrap_or("heroku/builder:22".into());
    BuildConfig::new(builder_name, app_dir)
        .buildpacks(vec![
            BuildpackReference::Other(String::from("heroku/procfile")),
            BuildpackReference::Other(String::from("heroku/jvm")),
            BuildpackReference::Crate,
        ])
        .to_owned()
}

fn assert_health_check_responds(ctx: &TestContext) {
    let port: u16 = 8080;
    let timeout: u64 = 5;

    ctx.start_container(ContainerConfig::new().expose_port(port), |container| {
        // Give the application a little time to boot up:
        // https://github.com/heroku/libcnb.rs/issues/280
        thread::sleep(Duration::from_secs(timeout));

        let addr = container
            .address_for_port(port)
            .expect("couldn't get container address");

        let res = ureq::get(&format!("http://{addr}"))
            .call()
            .expect("request to container failed")
            .into_string()
            .expect("response read error");

        assert_eq!(res, "Hello from Scala!");
    })
}
