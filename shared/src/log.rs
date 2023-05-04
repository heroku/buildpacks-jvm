use indoc::formatdoc;
use libherokubuildpack::log::log_error;
use std::fmt::Debug;

pub fn log_please_try_again_error<H: AsRef<str>, M: AsRef<str>, E: Debug>(
    header: H,
    message: M,
    error: E,
) {
    log_error(
        header,
        formatdoc! {"
            {message}

            Please try again. If this error persists, please contact us:
            https://help.heroku.com/

            Details: {error:?}
        ", message = message.as_ref(), error = error },
    );
}
