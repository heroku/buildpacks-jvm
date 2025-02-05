use libherokubuildpack::command::CommandExt;
use libherokubuildpack::write::line_mapped;
use std::process::{Command, Output};
use std::time::{Duration, Instant};

pub fn print_buildpack_name(buildpack_name: impl AsRef<str>) {
    let buildpack_name = buildpack_name.as_ref();
    print!("\n{ANSI_BUILDPACK_NAME_CODE}## {buildpack_name}{ANSI_RESET_CODE}\n\n");
}

pub fn print_section(text: impl Into<BuildpackOutputText>) {
    let text = text.into().to_ansi_string();
    println!("{ANSI_RESET_CODE}- {text}");
}

pub fn print_subsection(text: impl Into<BuildpackOutputText>) {
    let text = text.into().to_ansi_string();
    println!("{ANSI_RESET_CODE}  - {text}");
}

pub fn print_timing_done_subsection(duration: &Duration) {
    println!("{ANSI_RESET_CODE}  - Done ({})", format_duration(duration));
}

pub fn print_warning(title: impl AsRef<str>, body: impl Into<BuildpackOutputText>) {
    let title = title.as_ref();

    let mut sections = vec![BuildpackOutputTextSection::regular(format!(
        "WARNING: {title}\n\n"
    ))];

    let mut body = body.into();
    sections.append(&mut body.sections);

    let text = BuildpackOutputText {
        default_code: Some(String::from(ANSI_YELLOW_CODE)),
        line_prefix: Some(String::from("! ")),
        sections,
        ..BuildpackOutputText::default()
    };

    eprintln!("{}", text.to_ansi_string());
}

pub fn print_error(title: impl AsRef<str>, body: impl Into<BuildpackOutputText>) {
    let title = title.as_ref();

    let mut sections = vec![BuildpackOutputTextSection::regular(format!(
        "ERROR: {title}\n\n"
    ))];

    let mut body = body.into();
    sections.append(&mut body.sections);

    let text = BuildpackOutputText {
        default_code: Some(String::from(ANSI_RED_CODE)),
        line_prefix: Some(String::from(ERROR_WARNING_LINE_PREFIX)),
        sections,
        ..BuildpackOutputText::default()
    };

    eprintln!("{}", text.to_ansi_string());
}

pub fn run_command<E, F: FnOnce(std::io::Error) -> E, F2: FnOnce(Output) -> E>(
    mut command: Command,
    quiet: bool,
    io_error_fn: F,
    exit_status_fn: F2,
) -> Result<Output, E> {
    let child = if quiet {
        command.output_and_write_streams(std::io::sink(), std::io::sink())
    } else {
        const SPACE_ASCII: u8 = 0x20;
        let prefix = vec![SPACE_ASCII; 6];

        println!();

        let output = command.output_and_write_streams(
            line_mapped(std::io::stdout(), add_prefix_to_non_empty(prefix.clone())),
            line_mapped(std::io::stderr(), add_prefix_to_non_empty(prefix)),
        );

        println!();

        output
    };

    child.map_err(io_error_fn).and_then(|output| {
        if output.status.success() {
            Ok(output)
        } else {
            Err(exit_status_fn(output))
        }
    })
}

fn add_prefix_to_non_empty<P: Into<Vec<u8>>>(prefix: P) -> impl Fn(Vec<u8>) -> Vec<u8> {
    let prefix = prefix.into();

    move |mut input| {
        if input.is_empty() {
            vec![]
        } else {
            let mut result = prefix.clone();
            result.append(&mut input);
            result
        }
    }
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

pub fn track_timing<F, E, T>(f: F) -> Result<E, T>
where
    F: FnOnce() -> Result<E, T>,
{
    let start_time = Instant::now();
    let ret = f();
    let end_time = Instant::now();

    print_timing_done_subsection(&end_time.duration_since(start_time));
    ret
}

fn format_duration(duration: &Duration) -> String {
    let hours = (duration.as_secs() / 3600) % 60;
    let minutes = (duration.as_secs() / 60) % 60;
    let seconds = duration.as_secs() % 60;
    let milliseconds = duration.subsec_millis();
    let tenths = milliseconds / 100;

    if hours > 0 {
        format!("{hours}h {minutes}m {seconds}s")
    } else if minutes > 0 {
        format!("{minutes}m {seconds}s")
    } else if seconds > 0 || milliseconds >= 100 {
        format!("{seconds}.{tenths}s")
    } else {
        String::from("< 0.1s")
    }
}

const VALUE_DELIMITER_CHAR: char = '`';
const ANSI_RESET_CODE: &str = "\u{1b}[0m";
const ANSI_VALUE_CODE: &str = "\u{1b}[0;33m";
const ANSI_YELLOW_CODE: &str = "\u{1b}[0;33m";
const ANSI_RED_CODE: &str = "\u{1b}[0;31m";
const ANSI_BUILDPACK_NAME_CODE: &str = "\u{1b}[1;35m";
const ANSI_URL_CODE: &str = "\u{1b}[0;34m";
const ANSI_COMMAND_CODE: &str = "\u{1b}[1;36m";
const ERROR_WARNING_LINE_PREFIX: &str = "! ";

#[cfg(test)]
mod test {
    use super::*;

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

    #[test]
    fn test_display_duration() {
        let duration = Duration::ZERO;
        assert_eq!(format_duration(&duration), "< 0.1s");

        let duration = Duration::from_millis(99);
        assert_eq!(format_duration(&duration), "< 0.1s");

        let duration = Duration::from_millis(100);
        assert_eq!(format_duration(&duration), "0.1s");

        let duration = Duration::from_millis(210);
        assert_eq!(format_duration(&duration), "0.2s");

        let duration = Duration::from_millis(1100);
        assert_eq!(format_duration(&duration), "1.1s");

        let duration = Duration::from_millis(9100);
        assert_eq!(format_duration(&duration), "9.1s");

        let duration = Duration::from_millis(10100);
        assert_eq!(format_duration(&duration), "10.1s");

        let duration = Duration::from_millis(52100);
        assert_eq!(format_duration(&duration), "52.1s");

        let duration = Duration::from_millis(60 * 1000);
        assert_eq!(format_duration(&duration), "1m 0s");

        let duration = Duration::from_millis(60 * 1000 + 2000);
        assert_eq!(format_duration(&duration), "1m 2s");

        let duration = Duration::from_millis(60 * 60 * 1000 - 1);
        assert_eq!(format_duration(&duration), "59m 59s");

        let duration = Duration::from_millis(60 * 60 * 1000);
        assert_eq!(format_duration(&duration), "1h 0m 0s");

        let duration = Duration::from_millis(75 * 60 * 1000 - 1);
        assert_eq!(format_duration(&duration), "1h 14m 59s");
    }
}
