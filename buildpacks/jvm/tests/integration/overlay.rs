use crate::default_build_config;
use libcnb_test::{TestRunner, assert_contains};
use std::path::PathBuf;

#[test]
#[ignore = "integration test"]
fn overlay() {
    const ADDITIONAL_PATH: &str = "earth.txt";
    const ADDITIONAL_CONTENTS: &str = "Un diminuto punto azul";

    const CACERTS_PATH: &str = "lib/security/cacerts";
    const CACERTS_CONTENTS: &str = "overwritten";

    TestRunner::default().build(
        default_build_config("test-apps/java-21-app").app_dir_preprocessor(|app_dir| {
            let overlay_path = app_dir.join(".jdk-overlay");
            std::fs::create_dir_all(&overlay_path).unwrap();
            std::fs::write(overlay_path.join(ADDITIONAL_PATH), ADDITIONAL_CONTENTS).unwrap();

            let cacerts_path = overlay_path.join(PathBuf::from(CACERTS_PATH));
            std::fs::create_dir_all(cacerts_path.parent().unwrap()).unwrap();
            std::fs::write(&cacerts_path, CACERTS_CONTENTS).unwrap();
        }),
        |context| {
            // Validate that adding a new file works
            assert_contains!(
                context
                    .run_shell_command(format!("cat $JAVA_HOME/{ADDITIONAL_PATH}"))
                    .stdout,
                ADDITIONAL_CONTENTS
            );

            // Validate that overwriting a file works
            assert_contains!(
                context
                    .run_shell_command(format!("cat $JAVA_HOME/{CACERTS_PATH}"))
                    .stdout,
                CACERTS_CONTENTS
            );
        },
    );
}
