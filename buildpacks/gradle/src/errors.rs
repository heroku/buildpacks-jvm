use crate::GradleBuildpackError;
use buildpacks_jvm_shared::log::{
    log_build_tool_io_error, log_build_tool_unexpected_exit_code_error, log_please_try_again_error,
};
use indoc::indoc;
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
        GradleBuildpackError::GradleBuildIoError(error) => log_build_tool_io_error("Gradle", error),
        GradleBuildpackError::GradleBuildUnexpectedStatusError(exit_status) => {
            log_build_tool_unexpected_exit_code_error("Gradle", exit_status);
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
        GradleBuildpackError::StartGradleDaemonError(error) => log_please_try_again_error(
            "Failed to start Gradle daemon",
            "The Gradle daemon for this build could not be started.",
            error,
        ),
        GradleBuildpackError::BuildTaskUnknown => log_error(
            "Failed to determine build task",
            indoc! {"
                It looks like your project does not contain a 'stage' task, which Heroku needs in order
                to build your app. Our Dev Center article on preparing a Gradle application for Heroku
                describes how to create this task:
                https://devcenter.heroku.com/articles/deploying-gradle-apps-on-heroku
            "},
        ),
        GradleBuildpackError::DetectError(error) => {
            log_please_try_again_error(
                "Failed to determine if a file exists during detect",
                "Failed to determine if a file exists during detect",
                error,
            );
        }
        GradleBuildpackError::CannotDetermineDefaultAppProcess(error) => {
            log_please_try_again_error(
                "Failed to determine default app process",
                "Failed to determine default app process",
                error,
            );
        }
    }
}
