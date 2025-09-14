pub(crate) fn parse_errors(stdout: &[u8]) -> Option<SbtError> {
    String::from_utf8_lossy(stdout)
        .contains("Not a valid key: stage")
        .then_some(SbtError::MissingTask(String::from("stage")))
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum SbtError {
    MissingTask(String),
}

#[cfg(test)]
mod test {
    use super::SbtError;
    use super::parse_errors;
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
}
