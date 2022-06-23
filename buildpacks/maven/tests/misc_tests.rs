use indoc::indoc;
use libcnb_test::{
    assert_contains, assert_not_contains, BuildpackReference, PackResult, TestConfig, TestRunner,
};
use std::fs;
use std::os::unix::fs::PermissionsExt;

#[test]
fn mvnw_executable_bit() {
    TestRunner::default().run_test(
        default_config().app_dir_preprocessor(|dir| {
            fs::set_permissions(dir.join("mvnw"), fs::Permissions::from_mode(0o444)).unwrap();
        }),
        |context| {
            // Assert the build completed successfully, even with a missing executable bit on
            // the Maven Wrapper executable.
            assert_contains!(context.pack_stdout, "Successfully built image");
        },
    );
}

#[test]
fn mvn_dependency_list() {
    TestRunner::default().run_test(default_config(), |context| {
            context.prepare_container().start_with_shell_command(
                "cat /app/target/mvn-dependency-list.log",
                |container_context| {
                    assert_eq!(container_context.logs_wait().stdout, indoc! {"

                              The following files have been resolved:
                                 io.undertow:undertow-core:jar:2.1.1.Final:compile
                                 org.jboss.logging:jboss-logging:jar:3.4.1.Final:compile
                                 org.jboss.xnio:xnio-api:jar:3.8.0.Final:compile
                                 org.wildfly.common:wildfly-common:jar:1.5.2.Final:compile
                                 org.wildfly.client:wildfly-client-config:jar:1.0.1.Final:compile
                                 org.jboss.xnio:xnio-nio:jar:3.8.0.Final:runtime
                                 org.jboss.threads:jboss-threads:jar:3.1.0.Final:compile
                                 com.google.guava:guava:jar:30.0-jre:compile
                                 com.google.guava:failureaccess:jar:1.0.1:compile
                                 com.google.guava:listenablefuture:jar:9999.0-empty-to-avoid-conflict-with-guava:compile
                                 com.google.code.findbugs:jsr305:jar:3.0.2:compile
                                 org.checkerframework:checker-qual:jar:3.5.0:compile
                                 com.google.errorprone:error_prone_annotations:jar:2.3.4:compile
                                 com.google.j2objc:j2objc-annotations:jar:1.3:compile
                                 junit:junit:jar:4.13.1:test
                                 org.hamcrest:hamcrest-core:jar:1.3:test

                    "});
                },
            );
        });
}

#[test]
fn no_unexpected_files_in_app_dir() {
    TestRunner::default().run_test(default_config(), |context| {
            context.prepare_container().start_with_shell_command(
                "find /workspace -type f | sort -s",
                |container_context| {
                    assert_eq!(container_context.logs_wait().stdout, indoc! {"
                        /workspace/.mvn/wrapper/MavenWrapperDownloader.java
                        /workspace/.mvn/wrapper/maven-wrapper.jar
                        /workspace/.mvn/wrapper/maven-wrapper.properties
                        /workspace/Procfile
                        /workspace/mvnw
                        /workspace/mvnw.cmd
                        /workspace/pom.xml
                        /workspace/src/main/java/com/heroku/App.java
                        /workspace/src/test/java/com/heroku/AppTest.java
                        /workspace/target/classes/com/heroku/App$1.class
                        /workspace/target/classes/com/heroku/App.class
                        /workspace/target/dependency/checker-qual-3.5.0.jar
                        /workspace/target/dependency/error_prone_annotations-2.3.4.jar
                        /workspace/target/dependency/failureaccess-1.0.1.jar
                        /workspace/target/dependency/guava-30.0-jre.jar
                        /workspace/target/dependency/hamcrest-core-1.3.jar
                        /workspace/target/dependency/j2objc-annotations-1.3.jar
                        /workspace/target/dependency/jboss-logging-3.4.1.Final.jar
                        /workspace/target/dependency/jboss-threads-3.1.0.Final.jar
                        /workspace/target/dependency/jsr305-3.0.2.jar
                        /workspace/target/dependency/junit-4.13.1.jar
                        /workspace/target/dependency/listenablefuture-9999.0-empty-to-avoid-conflict-with-guava.jar
                        /workspace/target/dependency/undertow-core-2.1.1.Final.jar
                        /workspace/target/dependency/wildfly-client-config-1.0.1.Final.jar
                        /workspace/target/dependency/wildfly-common-1.5.2.Final.jar
                        /workspace/target/dependency/xnio-api-3.8.0.Final.jar
                        /workspace/target/dependency/xnio-nio-3.8.0.Final.jar
                        /workspace/target/maven-archiver/pom.properties
                        /workspace/target/maven-status/maven-compiler-plugin/compile/default-compile/createdFiles.lst
                        /workspace/target/maven-status/maven-compiler-plugin/compile/default-compile/inputFiles.lst
                        /workspace/target/maven-status/maven-compiler-plugin/testCompile/default-testCompile/createdFiles.lst
                        /workspace/target/maven-status/maven-compiler-plugin/testCompile/default-testCompile/inputFiles.lst
                        /workspace/target/mvn-dependency-list.log
                        /workspace/target/simple-http-service-1.0-SNAPSHOT.jar
                        /workspace/target/test-classes/com/heroku/AppTest.class
                    "})
                },
            );
        });
}

#[test]
fn no_internal_maven_options_logging() {
    TestRunner::default().run_test(default_config(), |context| {
        assert_not_contains!(context.pack_stdout, "-Dmaven.repo.local=");
        assert_not_contains!(context.pack_stdout, "-Duser.home=");
        assert_not_contains!(context.pack_stdout, "dependency:list");
        assert_not_contains!(
            context.pack_stdout,
            "-DoutputFile=target/mvn-dependency-list.log"
        );
    });
}

#[test]
fn cache_dependencies_between_builds() {
    TestRunner::default().run_test(default_config(), |context| {
        assert_contains!(context.pack_stdout, "Downloading from central");

        context.run_test(default_config(), |context| {
            assert_not_contains!(context.pack_stdout, "Downloading from central");
        });
    });
}

#[test]
fn descriptive_error_message_on_failed_build() {
    TestRunner::default().run_test(default_config().app_dir("../../test-fixtures/app-with-compile-error").expected_pack_result(PackResult::Failure), |context| {
            assert_contains!(context.pack_stdout, "[INFO] BUILD FAILURE");

            assert_contains!(
                context.pack_stderr,
                "[Error: Failed to build app with Maven]"
            );

            assert_contains!(
                context.pack_stderr,
                "We're sorry this build is failing! If you can't find the issue in application code,\nplease submit a ticket so we can help: https://help.heroku.com/"
            );
        });
}

fn default_config() -> TestConfig {
    TestConfig::new(
        "heroku/buildpacks:20",
        "../../test-fixtures/simple-http-service",
    )
    .buildpacks(vec![
        BuildpackReference::Other(String::from("heroku/jvm")),
        BuildpackReference::Crate,
    ])
    .clone()
}
