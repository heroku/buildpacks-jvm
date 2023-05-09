pub(crate) fn parse_errors(stdout: &[u8]) -> Option<SbtError> {
    let stdout = String::from_utf8_lossy(stdout);

    if stdout.contains("Not a valid key: stage") {
        Some(SbtError::MissingTask(String::from("stage")))
    } else if stdout.contains("is already defined as object") {
        Some(SbtError::AlreadyDefinedAsObject)
    } else {
        None
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum SbtError {
    MissingTask(String),
    AlreadyDefinedAsObject,
}

#[cfg(test)]
mod test {
    use super::parse_errors;
    use super::SbtError;
    use indoc::formatdoc;

    #[test]
    fn check_missing_stage_error_is_reported() {
        let stdout = formatdoc! {"
            [error] Expected ';'
            [error] Not a valid command: stage (similar: last-grep, set, last)
            [error] Not a valid project ID: stage
            [error] Expected ':'
            [error] Not a valid key: stage (similar: state, target, tags)
            [error] stage
            [error]      ^
        "}
        .into_bytes();

        assert_eq!(
            parse_errors(&stdout),
            Some(SbtError::MissingTask(String::from("stage")))
        );
    }

    #[test]
    fn check_already_defined_as_error_is_reported() {
        let stdout = formatdoc! {"
            [error] Expected ';'
            [error] Not a valid command: stage (similar: last-grep, set, last)
            [error] Not a valid project ID: stage
            [error] Expected ':'
            [error] Blah is already defined as object Blah
        "}
        .into_bytes();

        assert_eq!(
            parse_errors(&stdout),
            Some(SbtError::AlreadyDefinedAsObject)
        );
    }
}
