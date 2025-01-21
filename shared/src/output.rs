use bullet_stream::global::print;
use bullet_stream::style;
use fun_run::CommandWithName;
use std::process::{Command, Output};
use std::time::{Duration, Instant};

pub fn print_buildpack_name(buildpack_name: impl AsRef<str>) {
    print::h2(buildpack_name);
}

pub fn print_section(text: impl Into<BuildpackOutputText>) {
    print::bullet(text.into().to_ansi_string());
}

pub fn print_subsection(text: impl Into<BuildpackOutputText>) {
    print::sub_bullet(text.into().to_ansi_string());
}

pub fn print_timing_done_subsection(duration: &Duration) {
    println!("{ANSI_RESET_CODE}  - Done ({})", format_duration(duration));
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

pub fn run_command<E, F: FnOnce(std::io::Error) -> E, F2: FnOnce(Output) -> E>(
    mut command: Command,
    quiet: bool,
    io_error_fn: F,
    exit_status_fn: F2,
) -> Result<Output, E> {
    let title = format!("Running {}", style::value(command.name()));
    if quiet {
        let _timer = print::sub_start_timer(title);
        command.named_output()
    } else {
        print::sub_stream_with(&title, |stdout, stderr| {
            command.stream_output(stdout, stderr)
        })
    }
    .map(Into::<Output>::into)
    .map_err(|o| match o {
        fun_run::CmdError::SystemError(_, error) => io_error_fn(error),
        fun_run::CmdError::NonZeroExitNotStreamed(named_output)
        | fun_run::CmdError::NonZeroExitAlreadyStreamed(named_output) => {
            exit_status_fn(Into::<Output>::into(named_output))
        }
    })
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
const ANSI_URL_CODE: &str = "\u{1b}[0;34m";
const ANSI_COMMAND_CODE: &str = "\u{1b}[1;36m";
