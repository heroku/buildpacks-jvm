use bullet_stream::global::print;
use bullet_stream::style;
pub use fun_run::CmdError;
use fun_run::CommandWithName;
use std::process::{Command, Output};
use std::time::Instant;

pub fn print_buildpack_name(buildpack_name: impl AsRef<str>) {
    print::h2(buildpack_name);
}

pub fn print_section(text: impl Into<BuildpackOutputText>) {
    print::bullet(text.into().to_ansi_string());
}

pub fn print_subsection(text: impl Into<BuildpackOutputText>) {
    print::sub_bullet(text.into().to_ansi_string());
}

pub fn print_all_done(timer: Instant) {
    print::all_done(&Some(timer));
}

pub fn print_warning(title: impl AsRef<str>, body: impl Into<BuildpackOutputText>) {
    let title = title.as_ref();
    print::warning(format!(
        "WARNING: {title}\n\n{}",
        body.into().to_ansi_string()
    ));
}

pub fn print_error(title: impl AsRef<str>, body: impl Into<BuildpackOutputText>) {
    let title = title.as_ref();

    print::error(format!(
        "ERROR: {title}\n\n{}",
        body.into().to_ansi_string()
    ));
}

pub fn run_command(mut command: Command, quiet: bool) -> Result<Output, CmdError> {
    let title = format!("Running {}", style::value(command.name()));
    if quiet {
        let timer = print::sub_start_timer(title);
        let output = command.named_output();
        let _ = timer.done();
        output
    } else {
        print::sub_stream_with(&title, |stdout, stderr| {
            command.stream_output(stdout, stderr)
        })
    }
    .map(Into::<Output>::into)
}

#[derive(Clone, Debug)]
pub struct BuildpackOutputText {
    pub line_prefix: Option<String>,
    pub default_code: Option<String>,
    pub reset_code: String,
    pub value_code: String,
    pub sections: Vec<BuildpackOutputTextSection>,
}

impl Default for BuildpackOutputText {
    fn default() -> Self {
        Self {
            line_prefix: None,
            default_code: None,
            reset_code: String::from(ANSI_RESET_CODE),
            value_code: String::from(ANSI_VALUE_CODE),
            sections: vec![],
        }
    }
}

impl BuildpackOutputText {
    pub fn new(sections: impl Into<Vec<BuildpackOutputTextSection>>) -> Self {
        Self {
            sections: sections.into(),
            ..Self::default()
        }
    }

    fn to_ansi_string(&self) -> String {
        let mut result = String::new();

        // Every line must start with a style reset, the default ANSI code and the line prefix if
        // it exists.
        let line_start = format!(
            "{}{}{}",
            ANSI_RESET_CODE,
            self.default_code.clone().unwrap_or_default(),
            self.line_prefix.clone().unwrap_or_default()
        );

        result.push_str(&line_start);

        for section in &self.sections {
            let text = match section {
                BuildpackOutputTextSection::Regular(text)
                | BuildpackOutputTextSection::Value(text)
                | BuildpackOutputTextSection::Url(text)
                | BuildpackOutputTextSection::Command(text) => text,
            };

            match section {
                BuildpackOutputTextSection::Regular(_) => {}
                BuildpackOutputTextSection::Value(_) => {
                    result.push(VALUE_DELIMITER_CHAR);
                    result.push_str(ANSI_VALUE_CODE);
                }
                BuildpackOutputTextSection::Url(_) => {
                    result.push(VALUE_DELIMITER_CHAR);
                    result.push_str(ANSI_URL_CODE);
                }
                BuildpackOutputTextSection::Command(_) => {
                    result.push(VALUE_DELIMITER_CHAR);
                    result.push_str(ANSI_COMMAND_CODE);
                }
            }

            for char in text.chars() {
                if char == '\n' {
                    // Before ending a line, reset the text style so that the styling does not
                    // interfere with i.e. `pack` output.
                    result.push_str(ANSI_RESET_CODE);

                    result.push('\n');

                    result.push_str(&line_start);

                    match section {
                        BuildpackOutputTextSection::Value(_)
                        | BuildpackOutputTextSection::Url(_)
                        | BuildpackOutputTextSection::Command(_) => {
                            result.push_str(ANSI_VALUE_CODE);
                        }
                        BuildpackOutputTextSection::Regular(_) => {}
                    }
                } else {
                    result.push(char);
                }
            }

            match section {
                BuildpackOutputTextSection::Value(_)
                | BuildpackOutputTextSection::Url(_)
                | BuildpackOutputTextSection::Command(_) => {
                    result.push_str(ANSI_RESET_CODE);
                    result.push(VALUE_DELIMITER_CHAR);
                    result.push_str(&self.default_code.clone().unwrap_or_default());
                }
                BuildpackOutputTextSection::Regular(_) => {}
            }
        }

        result
    }
}

#[derive(Clone, Debug)]
pub enum BuildpackOutputTextSection {
    Regular(String),
    Value(String),
    Url(String),
    Command(String),
}

impl BuildpackOutputTextSection {
    pub fn regular(value: impl Into<String>) -> Self {
        BuildpackOutputTextSection::Regular(value.into())
    }

    pub fn value(value: impl Into<String>) -> Self {
        BuildpackOutputTextSection::Value(value.into())
    }

    pub fn command(value: impl Into<String>) -> Self {
        BuildpackOutputTextSection::Command(value.into())
    }
}

impl From<String> for BuildpackOutputText {
    fn from(value: String) -> Self {
        Self {
            sections: vec![BuildpackOutputTextSection::Regular(value)],
            ..Self::default()
        }
    }
}

impl From<&str> for BuildpackOutputText {
    fn from(value: &str) -> Self {
        Self {
            sections: vec![BuildpackOutputTextSection::Regular(String::from(value))],
            ..Self::default()
        }
    }
}

impl From<Vec<BuildpackOutputTextSection>> for BuildpackOutputText {
    fn from(value: Vec<BuildpackOutputTextSection>) -> Self {
        Self {
            sections: value,
            ..Self::default()
        }
    }
}

pub fn track_timing_subsection<F, E>(title: impl Into<BuildpackOutputText>, f: F) -> E
where
    F: FnOnce() -> E,
{
    let timer = print::sub_start_timer(title.into().to_ansi_string());
    let output = f();
    let _ = timer.done();
    output
}

const VALUE_DELIMITER_CHAR: char = '`';
const ANSI_RESET_CODE: &str = "\u{1b}[0m";
const ANSI_VALUE_CODE: &str = "\u{1b}[0;33m";
const ANSI_URL_CODE: &str = "\u{1b}[0;34m";
const ANSI_COMMAND_CODE: &str = "\u{1b}[1;36m";

#[cfg(test)]
mod test {
    use super::*;

    const ANSI_YELLOW_CODE: &str = "\u{1b}[0;33m";
    const ERROR_WARNING_LINE_PREFIX: &str = "! ";
    #[test]
    fn test_prefixing() {
        const DEFAULT_CODE: &str = "\x1B[0;33m";

        let text = BuildpackOutputText {
            default_code: Some(String::from(DEFAULT_CODE)),
            sections: vec![
                BuildpackOutputTextSection::regular("Hello\n"),
                BuildpackOutputTextSection::value("World"),
                BuildpackOutputTextSection::regular("\n"),
                BuildpackOutputTextSection::regular("How\nare you?"),
            ],
            line_prefix: Some(String::from(ERROR_WARNING_LINE_PREFIX)),
            ..Default::default()
        };

        assert_eq!(text.to_ansi_string(), "\u{1b}[0m\u{1b}[0;33m! Hello\u{1b}[0m\n\u{1b}[0m\u{1b}[0;33m! `\u{1b}[0;33mWorld\u{1b}[0m`\u{1b}[0;33m\u{1b}[0m\n\u{1b}[0m\u{1b}[0;33m! How\u{1b}[0m\n\u{1b}[0m\u{1b}[0;33m! are you?");
    }

    #[test]
    fn test_prefixing_with_value() {
        let text = BuildpackOutputText {
            default_code: Some(String::from(ANSI_YELLOW_CODE)),
            sections: vec![
                BuildpackOutputTextSection::regular("Intro\n"),
                BuildpackOutputTextSection::value("With\nNewline"),
                BuildpackOutputTextSection::regular("\nOutro"),
            ],
            line_prefix: Some(String::from("! ")),
            ..Default::default()
        };

        assert_eq!(
            text.to_ansi_string(),
            "\u{1b}[0m\u{1b}[0;33m! Intro\u{1b}[0m\n\u{1b}[0m\u{1b}[0;33m! `\u{1b}[0;33mWith\u{1b}[0m\n\u{1b}[0m\u{1b}[0;33m! \u{1b}[0;33mNewline\u{1b}[0m`\u{1b}[0;33m\u{1b}[0m\n\u{1b}[0m\u{1b}[0;33m! Outro"
        );
    }
}
