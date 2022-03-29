// Enable rustc and Clippy lints that are disabled by default.
// https://rust-lang.github.io/rust-clippy/stable/index.html
#![warn(clippy::pedantic)]
// This lint is too noisy and enforces a style that reduces readability in many cases.
#![allow(clippy::module_name_repetitions)]

use libcnb::data::exec_d::ExecDProgramOutputKey;
use libcnb::data::exec_d_program_output_key;
use libcnb::exec_d::write_exec_d_program_output;
use libcnb::Env;
use std::collections::HashMap;

pub fn main() {
    write_exec_d_program_output(output_from_env(&Env::from_current()));
}

fn output_from_env(env: &Env) -> HashMap<ExecDProgramOutputKey, String> {
    let heroku_metrics_agent_path = env
        .get("HEROKU_METRICS_AGENT_PATH")
        .map(|value| value.to_string_lossy().to_string())
        .expect("HEROKU_METRICS_AGENT_PATH environment variable is not set!");

    let has_heroku_metrics_url = env.get("HEROKU_METRICS_URL").is_some();

    let disable_heroku_metrics_agent = env.get("DISABLE_HEROKU_METRICS_AGENT").is_some();

    let is_one_off_dyno = env
        .get("DYNO")
        .filter(|name| name.to_string_lossy().starts_with("run."))
        .is_some();

    if has_heroku_metrics_url && !disable_heroku_metrics_agent && !is_one_off_dyno {
        let prefix = format!("-javaagent:{}", heroku_metrics_agent_path);

        let suffix = env
            .get("JAVA_TOOL_OPTIONS")
            .map(|value| format!(" {}", value.to_string_lossy()))
            .unwrap_or_default();

        HashMap::from([(
            exec_d_program_output_key!("JAVA_TOOL_OPTIONS"),
            format!("{}{}", prefix, suffix),
        )])
    } else {
        HashMap::default()
    }
}

#[cfg(test)]
mod tests {
    use crate::output_from_env;
    use libcnb::data::exec_d_program_output_key;
    use libcnb::Env;

    #[test]
    fn basic() {
        let mut env = Env::new();
        env.insert("HEROKU_METRICS_AGENT_PATH", AGENT_PATH);
        env.insert("JAVA_TOOL_OPTIONS", JAVA_TOOL_OPTIONS);
        env.insert("DYNO", "web.1");
        env.insert("HEROKU_METRICS_URL", "https://example.com/metrics");

        let output = output_from_env(&env);

        assert_eq!(
            output.get(&exec_d_program_output_key!("JAVA_TOOL_OPTIONS")),
            Some(&format!("-javaagent:{} {}", AGENT_PATH, JAVA_TOOL_OPTIONS))
        );
    }

    #[test]
    fn basic_no_java_tool_options() {
        let mut env = Env::new();
        env.insert("HEROKU_METRICS_AGENT_PATH", AGENT_PATH);
        env.insert("DYNO", "web.1");
        env.insert("HEROKU_METRICS_URL", "https://example.com/metrics");

        let output = output_from_env(&env);

        assert_eq!(
            output.get(&exec_d_program_output_key!("JAVA_TOOL_OPTIONS")),
            Some(&format!("-javaagent:{}", AGENT_PATH))
        );
    }

    #[test]
    fn one_off_dyno() {
        let mut env = Env::new();
        env.insert("HEROKU_METRICS_AGENT_PATH", AGENT_PATH);
        env.insert("JAVA_TOOL_OPTIONS", JAVA_TOOL_OPTIONS);
        env.insert("DYNO", "run.1");
        env.insert("HEROKU_METRICS_URL", "https://example.com/metrics");

        assert!(output_from_env(&env).is_empty());
    }

    #[test]
    fn missing_metrics_url() {
        let mut env = Env::new();
        env.insert("HEROKU_METRICS_AGENT_PATH", AGENT_PATH);
        env.insert("JAVA_TOOL_OPTIONS", JAVA_TOOL_OPTIONS);
        env.insert("DYNO", "web.1");

        assert!(output_from_env(&env).is_empty());
    }

    #[test]
    #[should_panic]
    fn missing_agent_path() {
        let mut env = Env::new();
        env.insert("JAVA_TOOL_OPTIONS", JAVA_TOOL_OPTIONS);
        env.insert("DYNO", "web.1");
        env.insert("HEROKU_METRICS_URL", "https://example.com/metrics");

        output_from_env(&env);
    }

    #[test]
    fn explicit_disable() {
        let mut env = Env::new();
        env.insert("HEROKU_METRICS_AGENT_PATH", AGENT_PATH);
        env.insert("JAVA_TOOL_OPTIONS", JAVA_TOOL_OPTIONS);
        env.insert("DYNO", "web.1");
        env.insert("HEROKU_METRICS_URL", "https://example.com/metrics");
        env.insert("DISABLE_HEROKU_METRICS_AGENT", "true");

        assert!(output_from_env(&env).is_empty());
    }

    const AGENT_PATH: &str = "/layers/buildpack/agent/agent.jar";
    const JAVA_TOOL_OPTIONS: &str = "-Xmx1G";
}
