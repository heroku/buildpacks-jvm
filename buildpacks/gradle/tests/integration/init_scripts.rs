use crate::default_buildpacks;
use buildpacks_jvm_shared_test::DEFAULT_INTEGRATION_TEST_BUILDER;
use indoc::indoc;
use libcnb_test::{assert_contains, BuildConfig, TestRunner};
use std::fs;

#[test]
#[ignore = "integration test"]
fn init() {
    let build_config = BuildConfig::new(
        DEFAULT_INTEGRATION_TEST_BUILDER,
        "test-apps/heroku-gradle-getting-started",
    )
    .buildpacks(default_buildpacks())
    .app_dir_preprocessor(|dir| {
        let init_script_dir = dir.join(".heroku/gradle/init.d");
        fs::create_dir_all(&init_script_dir).unwrap();

        fs::write(
            init_script_dir.join("kotlin_custom_init.init.gradle.kts"),
            GRADLE_INIT_SCRIPT_KOTLIN,
        )
        .unwrap();

        fs::write(
            init_script_dir.join("groovy_custom_init.gradle"),
            GRADLE_INIT_SCRIPT_GROOVY,
        )
        .unwrap();
    })
    .to_owned();

    TestRunner::default().build(&build_config, |context| {
        assert_contains!(context.pack_stdout, "Kotlin init-script running...");
        assert_contains!(context.pack_stdout, "Groovy init-script running...");

        assert_contains!(
            context.pack_stdout,
            "Kotlin output from Apache Commons Mathematics: 1"
        );
        assert_contains!(
            context.pack_stdout,
            "Groovy output from Apache Commons Mathematics: 1"
        );
    });
}

const GRADLE_INIT_SCRIPT_KOTLIN: &str = indoc! {r#"
    import org.apache.commons.math.fraction.Fraction

    initscript {
        repositories {
            mavenCentral()
        }
        dependencies {
            classpath("org.apache.commons:commons-math:2.0")
        }
    }

    println("Kotlin init-script running...")
    println("Kotlin output from Apache Commons Mathematics: " + Fraction.ONE_FIFTH.multiply(5))
"#};

const GRADLE_INIT_SCRIPT_GROOVY: &str = indoc! {"
    import org.apache.commons.math.fraction.Fraction

    initscript {
        repositories {
            mavenCentral()
        }
        dependencies {
            classpath 'org.apache.commons:commons-math:2.0'
        }
    }

    println 'Groovy init-script running...'
    println 'Groovy output from Apache Commons Mathematics: ' + Fraction.ONE_FIFTH.multiply(5)
"};
