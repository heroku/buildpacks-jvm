use crate::output::{print_error, print_section, print_subsection};
use fun_run::CmdError;
use indoc::formatdoc;
use std::fmt::Debug;

pub fn log_please_try_again<H: AsRef<str>, M: AsRef<str>>(header: H, message: M) {
    print_error(
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
    print_error(
        header,
        formatdoc! {"
            {message}

            Please try again. If this error persists, please open an issue on GitHub:
            https://github.com/heroku/buildpacks-jvm/issues/new

            Details: {error:?}
        ", message = message.as_ref(), error = error },
    );
}

pub fn log_build_tool_command_error(build_tool_name: &str, error: &CmdError) {
    print_section("Debug info:");
    print_subsection(format!("{error}"));

    let exit_code_string = error
        .status()
        .code()
        .map_or(String::from("<unknown>"), |code| code.to_string());

    print_error(
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
