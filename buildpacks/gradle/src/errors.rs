use crate::GradleBuildpackError;
use buildpacks_jvm_shared::log::log_please_try_again_error;
use indoc::{formatdoc, indoc};
use libherokubuildpack::log::log_error;

#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub(crate) fn on_error_gradle_buildpack(error: GradleBuildpackError) {
    match error {
        GradleBuildpackError::GradleWrapperNotFound => {
            log_error(
                "Missing Gradle Wrapper",
                indoc! {"
                    This buildpack leverages Gradle Wrapper to install the correct Gradle version to build your application.
                    However, it seems that your application does not contain the required Gradle Wrapper files.

                    To add the Gradle Wrapper, run the following command in your application's root directory:
                    $ gradle wrapper

                    Additional information about Gradle Wrapper and available configuration options can be found here:
                    https://docs.gradle.org/current/userguide/gradle_wrapper.html
                "},
            );
        }
        GradleBuildpackError::GradleBuildIoError(error) => log_error(
            "Failed to build app with Gradle",
            formatdoc! {"
                We're sorry this build is failing! If you can't find the issue in application code,
                please submit a ticket so we can help: https://help.heroku.com/

                Details: {error}
            ", error = error },
        ),
        GradleBuildpackError::GradleBuildUnexpectedStatusError(exit_status) => {
            let exit_code_string = exit_status.code().map_or_else(
                || String::from("<unknown>"),
                |exit_code| exit_code.to_string(),
            );

            log_error(
                "Failed to build app with Gradle",
                formatdoc! {"
                    We're sorry this build is failing! If you can't find the issue in application code,
                    please submit a ticket so we can help: https://help.heroku.com/

                    Gradle exit code was: {exit_code}
                ", exit_code = exit_code_string },
            );
        }
        GradleBuildpackError::GetTasksError(error) => log_please_try_again_error(
            "Failed to get Gradle tasks",
            "Failed to get Gradle tasks",
            error,
        ),
        GradleBuildpackError::GetDependencyReportError(error) => log_please_try_again_error(
            "Failed to get Gradle dependency report",
            "Failed to get Gradle dependency report",
            error,
        ),
        GradleBuildpackError::WriteGradlePropertiesError(error) => log_please_try_again_error(
            "Failed to write Gradle configuration",
            "Failed to write Gradle configuration",
            error,
        ),
        GradleBuildpackError::WriteGradleInitScriptError(error) => log_please_try_again_error(
            "Failed to write Gradle init script",
            "Failed to write Gradle init script",
            error,
        ),
        GradleBuildpackError::CannotSetGradleWrapperExecutableBit(error) => {
            log_please_try_again_error(
                "Failed to set executable bit for Gradle wrapper",
                "Failed to set executable bit for Gradle wrapper",
                error,
            );
        }
        GradleBuildpackError::StartGradleDaemonError(error) => log_error(
            "Failed to start Gradle daemon",
            formatdoc! {"
                We're sorry this build is failing! If you can't find the issue in application code,
                please submit a ticket so we can help: https://help.heroku.com/

                Details: {error:?}
            ", error = error },
        ),
        GradleBuildpackError::BuildTaskUnknown => log_error(
            "Failed to determine build task",
            indoc! {"
                It looks like your project does not contain a 'stage' task, which Heroku needs in order
                to build your app. Our Dev Center article on preparing a Gradle application for Heroku
                describes how to create this task:
                https://devcenter.heroku.com/articles/deploying-gradle-apps-on-heroku

                If you're stilling having trouble, please submit a ticket so we can help:
                https://help.heroku.com
            "},
        ),
    }
}
