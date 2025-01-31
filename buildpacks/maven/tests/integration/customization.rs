use crate::default_build_config;
use indoc::indoc;
use libcnb_test::{assert_contains, TestRunner};

#[test]
#[ignore = "integration test"]
fn maven_custom_goals() {
    TestRunner::default().build(default_build_config("test-apps/simple-http-service").env("MAVEN_CUSTOM_GOALS", "site"), |context| {
        // Assert only the goals in MAVEN_CUSTOM_GOALS are executed
        assert_contains!(context.pack_stdout, "Running `./mvnw -DskipTests -B site`");
        assert_contains!(context.pack_stdout,"[INFO] --- maven-site-plugin:3.7.1:site (default-site) @ simple-http-service ---");

        // The dependency list is implemented by using the dependency:list goal. We need to
        // assert it won't be overwritten by the user's choice of goals.
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
fn maven_custom_opts() {
    TestRunner::default().build(
        default_build_config("test-apps/simple-http-service").env("MAVEN_CUSTOM_OPTS", "-X"),
        |context| {
            // Assert only the options in MAVEN_CUSTOM_GOALS are used
            assert_contains!(context.pack_stdout, "Running `./mvnw -X -B clean install`");
            assert_contains!(context.pack_stdout, "[DEBUG] -- end configuration --");

            // -DskipTests is part of the default Maven options. We expect it to be overridden by MAVEN_CUSTOM_OPTS and
            // therefore expect Maven to run tests.
            assert_contains!(
                context.pack_stdout,
                "[INFO] Tests run: 1, Failures: 0, Errors: 0, Skipped: 0"
            );
        },
    );
}
