use libcnb::{
    data::build_plan::{BuildPlan, Require},
    detect::{cnb_runtime_detect, DetectOutcome, GenericDetectContext},
};

fn main() {
    cnb_runtime_detect(detect)
}

fn detect(ctx: GenericDetectContext) -> anyhow::Result<DetectOutcome> {
    let mut buildplan = BuildPlan::new();

    // We check for a function.toml/project.toml to be able to distinguish between regular JVM applications and a function.
    // Just from the application alone, they're indistinguishable by design.
    let outcome = if ctx.app_dir().join("function.toml").exists()
        || ctx.app_dir().join("project.toml").exists()
    {
        buildplan.requires.push(Require::new("jdk"));
        buildplan.requires.push(Require::new("jvm-application"));

        DetectOutcome::Pass(buildplan)
    } else {
        DetectOutcome::Fail
    };

    Ok(outcome)
}
