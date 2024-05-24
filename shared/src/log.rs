use indoc::formatdoc;
use libherokubuildpack::log::log_error;
use std::fmt::Debug;
use std::process::ExitStatus;

pub fn log_please_try_again<H: AsRef<str>, M: AsRef<str>>(header: H, message: M) {
    log_error(
        header,
        formatdoc! {"
            {message}

            Please try again. If this error persists, please open an issue on GitHub:
            https://github.com/heroku/buildpacks-jvm/issues/new
        ", message = message.as_ref()},
    );
}

pub fn log_please_try_again_error<H: AsRef<str>, M: AsRef<str>, E: Debug>(
    header: H,
    message: M,
    error: E,
) {
    log_error(
        header,
        formatdoc! {"
            {message}

            Please try again. If this error persists, please open an issue on GitHub:
            https://github.com/heroku/buildpacks-jvm/issues/new

            Details: {error:?}
        ", message = message.as_ref(), error = error },
    );
}

pub fn log_build_tool_unexpected_exit_code_error(build_tool_name: &str, exit_status: ExitStatus) {
    let exit_code_string = exit_status
        .code()
        .map_or(String::from("<unknown>"), |code| code.to_string());

    log_error(
        format!("Unexpected {build_tool_name} exit code"),
        formatdoc! { "
            {build_tool_name} unexpectedly exited with code '{exit_code_string}'. The most common reason for this are
            problems with your application code and/or build configuration.

            Please refer to the {build_tool_name} output above for details. If you believe this error is not
            caused by your application, please open an issue on GitHub:
            https://github.com/heroku/buildpacks-jvm/issues/new
        " },
    );
}

pub fn log_build_tool_io_error(build_tool_name: &str, error: std::io::Error) {
    log_please_try_again_error(
        "Running {build_tool_name} failed",
        format!("An unexpected IO error occurred while running {build_tool_name}."),
        error,
    );
}
