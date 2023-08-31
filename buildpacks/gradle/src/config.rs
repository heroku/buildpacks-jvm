use libcnb::build::BuildContext;
use libcnb::generic::GenericPlatform;
use libcnb::{Buildpack, Platform};

pub(crate) struct GradleBuildpackConfig {
    pub(crate) gradle_task: Option<String>,
}

impl<T: Buildpack<Platform = GenericPlatform>> From<&BuildContext<T>> for GradleBuildpackConfig {
    fn from(context: &BuildContext<T>) -> Self {
        GradleBuildpackConfig {
            gradle_task: context
                .platform
                .env()
                .get("GRADLE_TASK")
                .map(|s| s.to_string_lossy().to_string()),
        }
    }
}
