use crate::default_build_config;
use indoc::indoc;
use libcnb_test::{assert_contains, assert_not_contains, PackResult, TestRunner};
use std::fs;
use std::os::unix::fs::PermissionsExt;

#[test]
#[ignore = "integration test"]
fn mvnw_executable_bit() {
    TestRunner::default().build(
        default_build_config("test-apps/simple-http-service").app_dir_preprocessor(|dir| {
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
#[ignore = "integration test"]
fn mvn_dependency_list() {
    TestRunner::default().build(default_build_config("test-apps/simple-http-service"), |context| {
        assert_eq!(
            context.run_shell_command("cat /workspace/target/mvn-dependency-list.log").stdout,
            indoc! {"

                The following files have been resolved:
                   io.undertow:undertow-core:jar:2.3.17.Final:compile
                   org.jboss.logging:jboss-logging:jar:3.4.3.Final:compile
                   org.jboss.xnio:xnio-api:jar:3.8.16.Final:compile
                   org.wildfly.common:wildfly-common:jar:1.5.4.Final:compile
                   org.wildfly.client:wildfly-client-config:jar:1.0.1.Final:compile
                   org.jboss.xnio:xnio-nio:jar:3.8.16.Final:runtime
                   org.jboss.threads:jboss-threads:jar:3.5.0.Final:compile
                   com.google.guava:guava:jar:32.0.0-jre:compile
                   com.google.guava:failureaccess:jar:1.0.1:compile
                   com.google.guava:listenablefuture:jar:9999.0-empty-to-avoid-conflict-with-guava:compile
                   com.google.code.findbugs:jsr305:jar:3.0.2:compile
                   org.checkerframework:checker-qual:jar:3.33.0:compile
                   com.google.errorprone:error_prone_annotations:jar:2.18.0:compile
                   com.google.j2objc:j2objc-annotations:jar:2.8:compile
                   junit:junit:jar:4.13.1:test
                   org.hamcrest:hamcrest-core:jar:1.3:test

            "}
        );
    });
}

#[test]
#[ignore = "integration test"]
fn no_unexpected_files_in_app_dir() {
    TestRunner::default().build(default_build_config("test-apps/simple-http-service"), |context| {
        assert_eq!(
            context.run_shell_command("find /workspace -type f | sort -s").stdout,
            indoc! {"
                /workspace/.mvn/wrapper/MavenWrapperDownloader.java
                /workspace/.mvn/wrapper/maven-wrapper.jar
                /workspace/.mvn/wrapper/maven-wrapper.properties
                /workspace/Procfile
                /workspace/mvnw
                /workspace/mvnw.cmd
                /workspace/pom.xml
                /workspace/src/main/java/com/heroku/App.java
                /workspace/src/test/java/com/heroku/AppTest.java
                /workspace/system.properties
                /workspace/target/classes/com/heroku/App$1.class
                /workspace/target/classes/com/heroku/App.class
                /workspace/target/dependency/checker-qual-3.33.0.jar
                /workspace/target/dependency/error_prone_annotations-2.18.0.jar
                /workspace/target/dependency/failureaccess-1.0.1.jar
                /workspace/target/dependency/guava-32.0.0-jre.jar
                /workspace/target/dependency/hamcrest-core-1.3.jar
                /workspace/target/dependency/j2objc-annotations-2.8.jar
                /workspace/target/dependency/jboss-logging-3.4.3.Final.jar
                /workspace/target/dependency/jboss-threads-3.5.0.Final.jar
                /workspace/target/dependency/jsr305-3.0.2.jar
                /workspace/target/dependency/junit-4.13.1.jar
                /workspace/target/dependency/listenablefuture-9999.0-empty-to-avoid-conflict-with-guava.jar
                /workspace/target/dependency/undertow-core-2.3.17.Final.jar
                /workspace/target/dependency/wildfly-client-config-1.0.1.Final.jar
                /workspace/target/dependency/wildfly-common-1.5.4.Final.jar
                /workspace/target/dependency/xnio-api-3.8.16.Final.jar
                /workspace/target/dependency/xnio-nio-3.8.16.Final.jar
                /workspace/target/maven-archiver/pom.properties
                /workspace/target/maven-status/maven-compiler-plugin/compile/default-compile/createdFiles.lst
                /workspace/target/maven-status/maven-compiler-plugin/compile/default-compile/inputFiles.lst
                /workspace/target/maven-status/maven-compiler-plugin/testCompile/default-testCompile/createdFiles.lst
                /workspace/target/maven-status/maven-compiler-plugin/testCompile/default-testCompile/inputFiles.lst
                /workspace/target/mvn-dependency-list.log
                /workspace/target/simple-http-service-1.0-SNAPSHOT.jar
                /workspace/target/test-classes/com/heroku/AppTest.class
            "}
        );
    });
}

#[test]
#[ignore = "integration test"]
fn no_internal_maven_options_logging() {
    TestRunner::default().build(
        default_build_config("test-apps/simple-http-service"),
        |context| {
            assert_not_contains!(context.pack_stdout, "-Dmaven.repo.local=");
            assert_not_contains!(context.pack_stdout, "-Duser.home=");
            assert_not_contains!(
                context.pack_stdout,
                "-DoutputFile=target/mvn-dependency-list.log"
            );
        },
    );
}

#[test]
#[ignore = "integration test"]
fn descriptive_error_message_on_failed_build() {
    TestRunner::default().build(default_build_config("test-apps/app-with-compile-error").expected_pack_result(PackResult::Failure), |context| {
            assert_contains!(context.pack_stdout, "[INFO] BUILD FAILURE");

            assert_contains!(
                context.pack_stderr,
                "! ERROR: Unexpected Maven exit code"
            );

            assert_contains!(
                context.pack_stderr,
                "! Maven unexpectedly exited with code '1'. The most common reason for this are\n! problems with your application code and/or build configuration."
            );
        });
}
