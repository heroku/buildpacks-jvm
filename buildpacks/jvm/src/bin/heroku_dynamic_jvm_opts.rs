use libcnb::data::exec_d::ExecDProgramOutputKey;
use libcnb::data::exec_d_program_output_key;
use libcnb::exec_d::write_exec_d_program_output;
use libcnb::Env;
use std::collections::HashMap;

fn main() {
    write_exec_d_program_output(output_from_env(Env::from_current()));
}

fn output_from_env(env: Env) -> HashMap<ExecDProgramOutputKey, String> {
    let prefix = dyno_type_jvm_opts().join(" ");

    let suffix = env
        .get("JAVA_TOOL_OPTIONS")
        .map(|value| format!(" {}", value.to_string_lossy().to_string()))
        .unwrap_or_default();

    HashMap::from([(
        exec_d_program_output_key!("JAVA_TOOL_OPTIONS"),
        format!("{}{}", prefix, suffix),
    )])
}

fn dyno_type_jvm_opts() -> Vec<&'static str> {
    match detect_available_memory() {
        // Free, Hobby, Standard-1X
        Some(536_870_912) => vec!["-Xmx300m", "-Xss512k", "-XX:CICompilerCount=2"],
        // Standard-2X, Private-S
        Some(1_073_741_824) => vec!["-Xmx671m", "-XX:CICompilerCount=2"],
        // Performance-M, Private-M
        Some(2_684_354_560) => vec!["-Xmx2g"],
        // Performance-L, Private-L
        Some(15_032_385_536) => vec!["-Xmx12g"],
        _ => vec![],
    }
}

fn detect_available_memory() -> Option<usize> {
    [
        "/sys/fs/cgroup/memory.max",
        "/sys/fs/cgroup/memory/memory.limit_in_bytes",
    ]
    .iter()
    .find_map(|path| std::fs::read_to_string(path).ok())
    .and_then(|contents| contents.trim().parse().ok())
}
