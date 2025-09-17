// Required due to: https://github.com/rust-lang/rust/issues/95513
#![allow(unused_crate_dependencies)]

use libcnb::Env;
use libcnb::data::exec_d::ExecDProgramOutputKey;
use libcnb::data::exec_d_program_output_key;
use libcnb::exec_d::write_exec_d_program_output;
use std::collections::HashMap;

fn main() {
    write_exec_d_program_output(output_from_env(&Env::from_current()));
}

fn output_from_env(env: &Env) -> HashMap<ExecDProgramOutputKey, String> {
    let prefix = dyno_type_jvm_opts().join(" ");

    let suffix = env
        .get("JAVA_TOOL_OPTIONS")
        .map(|value| format!(" {}", value.to_string_lossy()))
        .unwrap_or_default();

    HashMap::from([(
        exec_d_program_output_key!("JAVA_TOOL_OPTIONS"),
        format!("{prefix}{suffix}"),
    )])
}

fn dyno_type_jvm_opts() -> Vec<&'static str> {
    match detect_available_memory() {
        // Eco, Basic, Standard-1X
        Some(536_870_912) => vec!["-Xmx300m", "-Xss512k", "-XX:CICompilerCount=2"],
        // Standard-2X, Private-S
        Some(1_073_741_824) => vec!["-Xmx671m", "-XX:CICompilerCount=2"],
        // Rely on JVM ergonomics for other dyno types, but increase the maximum RAM percentage from 25% to 80%.
        // This is to ensure max heap configuration is aligned to the existing Heroku JVM buildpacks.
        _ => vec!["-XX:MaxRAMPercentage=80.0"],
    }
}

fn detect_available_memory() -> Option<usize> {
    [
        "/sys/fs/cgroup/memory.max",
        "/sys/fs/cgroup/memory/memory.limit_in_bytes",
    ]
    .into_iter()
    .find_map(|path| std::fs::read_to_string(path).ok())
    .and_then(|contents| contents.trim().parse().ok())
}
