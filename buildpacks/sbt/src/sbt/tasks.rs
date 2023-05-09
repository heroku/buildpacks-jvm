use crate::configuration::SbtBuildpackConfiguration;

pub(crate) fn from_config(build_config: &SbtBuildpackConfiguration) -> Vec<String> {
    let mut tasks: Vec<String> = Vec::new();

    if let Some(true) = &build_config.sbt_clean {
        tasks.push(String::from("clean"));
    }

    if let Some(sbt_pre_tasks) = &build_config.sbt_pre_tasks {
        sbt_pre_tasks
            .iter()
            .for_each(|task| tasks.push(task.to_string()));
    }

    if let Some(sbt_tasks) = &build_config.sbt_tasks {
        sbt_tasks
            .iter()
            .for_each(|task| tasks.push(task.to_string()));
    } else {
        let default_tasks = vec![String::from("compile"), String::from("stage")];
        for default_task in &default_tasks {
            tasks.push(match &build_config.sbt_project {
                Some(project) => format!("{project}/{default_task}"),
                None => default_task.to_string(),
            });
        }
    }

    tasks
}

#[cfg(test)]
mod test {
    use super::from_config;
    use crate::configuration::SbtBuildpackConfiguration;

    #[test]
    fn from_config_with_no_configured_options() {
        let config = SbtBuildpackConfiguration {
            sbt_project: None,
            sbt_pre_tasks: None,
            sbt_tasks: None,
            sbt_clean: None,
            sbt_available_at_launch: None,
        };

        assert_eq!(from_config(&config), vec!["compile", "stage"]);
    }

    #[test]
    fn from_config_with_all_configured_options() {
        let config = SbtBuildpackConfiguration {
            sbt_project: Some("projectName".to_string()),
            sbt_pre_tasks: Some(vec!["preTask".to_string()]),
            sbt_tasks: Some(vec!["task".to_string()]),
            sbt_clean: Some(true),
            sbt_available_at_launch: None,
        };

        assert_eq!(from_config(&config), vec!["clean", "preTask", "task"]);
    }

    #[test]
    fn from_config_with_clean_set_to_true() {
        let config = SbtBuildpackConfiguration {
            sbt_project: None,
            sbt_pre_tasks: None,
            sbt_tasks: None,
            sbt_clean: Some(true),
            sbt_available_at_launch: None,
        };

        assert_eq!(from_config(&config), vec!["clean", "compile", "stage"]);
    }

    #[test]
    fn from_config_with_clean_set_to_false() {
        let config = SbtBuildpackConfiguration {
            sbt_project: None,
            sbt_pre_tasks: None,
            sbt_tasks: None,
            sbt_clean: Some(false),
            sbt_available_at_launch: None,
        };
        assert_eq!(from_config(&config), vec!["compile", "stage"]);
    }

    #[test]
    fn from_config_with_project_set() {
        let config = SbtBuildpackConfiguration {
            sbt_project: Some("projectName".to_string()),
            sbt_pre_tasks: None,
            sbt_tasks: None,
            sbt_clean: None,
            sbt_available_at_launch: None,
        };
        assert_eq!(
            from_config(&config),
            vec!["projectName/compile", "projectName/stage"]
        );
    }

    #[test]
    fn from_config_with_project_and_pre_tasks_set() {
        let config = SbtBuildpackConfiguration {
            sbt_project: Some("projectName".to_string()),
            sbt_pre_tasks: Some(vec!["preTask".to_string()]),
            sbt_tasks: None,
            sbt_clean: None,
            sbt_available_at_launch: None,
        };
        assert_eq!(
            from_config(&config),
            vec!["preTask", "projectName/compile", "projectName/stage"]
        );
    }

    #[test]
    fn from_config_with_project_and_clean_set() {
        let config = SbtBuildpackConfiguration {
            sbt_project: Some("projectName".to_string()),
            sbt_pre_tasks: None,
            sbt_tasks: None,
            sbt_clean: Some(true),
            sbt_available_at_launch: None,
        };
        assert_eq!(
            from_config(&config),
            vec!["clean", "projectName/compile", "projectName/stage"]
        );
    }
}
