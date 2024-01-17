use crate::gradle_command::init::gradle_init_script_args;
use crate::gradle_command::{run_gradle_command, GradleCommandError};
use libcnb::Env;
use std::path::{Path, PathBuf};
use std::process::Command;

pub(crate) fn tasks(
    current_dir: &Path,
    env: &Env,
    gradle_init_scripts: &[PathBuf],
) -> Result<Tasks, GradleCommandError<nom::error::Error<String>>> {
    run_gradle_command(
        Command::new(current_dir.join("gradlew"))
            .current_dir(current_dir)
            .envs(env)
            .args(gradle_init_script_args(gradle_init_scripts))
            .args(["--quiet", "tasks"]),
        |stdout, _stderr| {
            parser::parse(stdout)
                .map(|groups| Tasks { groups })
                .map_err(|error| nom::error::Error {
                    input: error.input.to_string(),
                    code: error.code,
                })
        },
    )
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct Tasks {
    pub(crate) groups: Vec<TaskGroup>,
}

impl Tasks {
    fn names(&self) -> Vec<String> {
        self.groups
            .iter()
            .flat_map(|task_group| &task_group.tasks)
            .map(|task| task.name.clone())
            .collect()
    }

    pub(crate) fn has_task(&self, s: &str) -> bool {
        self.names().iter().any(|task_name| task_name == s)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct TaskGroup {
    pub(crate) heading: String,
    pub(crate) tasks: Vec<Task>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct Task {
    pub(crate) name: String,
    pub(crate) description: String,
}

mod parser {
    use super::Task;
    use super::TaskGroup;
    use nom::bytes::complete::{is_not, tag};
    use nom::character::complete::{char, line_ending, not_line_ending};
    use nom::combinator::{map, recognize, verify};
    use nom::multi::{count, many0, many1, many_till};
    use nom::sequence::{terminated, tuple};
    use nom::{Finish, IResult};

    pub(crate) fn parse(input: &str) -> Result<Vec<TaskGroup>, nom::error::Error<&str>> {
        many1(map(many_till(any_line, task_group), |(_, out)| out))(input)
            .finish()
            .map(|(_remaining, parsed)| parsed)
    }

    fn task_group(input: &str) -> IResult<&str, TaskGroup> {
        map(
            tuple((task_group_heading, many0(task_line))),
            |(heading, lines)| TaskGroup {
                heading,
                tasks: lines,
            },
        )(input)
    }

    fn task_group_heading(input: &str) -> IResult<&str, String> {
        let (input, line) = verify(any_line, |line: &str| line.ends_with("tasks"))(input)?;
        let (input, _) = terminated(count(char('-'), line.len()), line_ending)(input)?;

        Ok((input, line))
    }

    fn task_line(input: &str) -> IResult<&str, Task> {
        map(
            tuple((
                map(terminated(task_name, tag(" - ")), String::from),
                map(terminated(not_line_ending, line_ending), String::from),
            )),
            |(name, description)| Task { name, description },
        )(input)
    }

    fn any_line(input: &str) -> IResult<&str, String> {
        map(terminated(not_line_ending, line_ending), String::from)(input)
    }

    fn task_name(input: &str) -> IResult<&str, &str> {
        // https://github.com/gradle/gradle/blob/95410a3dff9c63c660f897297f54ebaad3581f5a/subprojects/core/src/main/java/org/gradle/util/internal/NameValidator.java#L26-L27
        recognize(is_not("/\\:<>\"?*| "))(input)
    }

    #[cfg(test)]
    mod test {
        use super::{Task, TaskGroup};
        use indoc::indoc;

        #[test]
        fn test() {
            let input = indoc! {"

                ------------------------------------------------------------
                Tasks runnable from root project 'demo'
                ------------------------------------------------------------

                Application tasks
                -----------------
                bootRun - Runs this project as a Spring Boot application.

                Build tasks
                -----------
                assemble - Assembles the outputs of this project.
                bootBuildImage - Builds an OCI image of the application using the output of the bootJar task
                bootJar - Assembles an executable jar archive containing the main classes and their dependencies.
                build - Assembles and tests this project.
                buildDependents - Assembles and tests this project and all projects that depend on it.
                buildNeeded - Assembles and tests this project and all projects it depends on.
                classes - Assembles main classes.
                clean - Deletes the build directory.
                jar - Assembles a jar archive containing the main classes.
                resolveMainClassName - Resolves the name of the application's main class.
                testClasses - Assembles test classes.

                Build Setup tasks
                -----------------
                init - Initializes a new Gradle build.
                wrapper - Generates Gradle wrapper files.

                Documentation tasks
                -------------------
                javadoc - Generates Javadoc API documentation for the main source code.

                Help tasks
                ----------
                buildEnvironment - Displays all buildscript dependencies declared in root project 'demo'.
                dependencies - Displays all dependencies declared in root project 'demo'.
                dependencyInsight - Displays the insight into a specific dependency in root project 'demo'.
                dependencyManagement - Displays the dependency management declared in root project 'demo'.
                help - Displays a help message.
                javaToolchains - Displays the detected java toolchains.
                kotlinDslAccessorsReport - Prints the Kotlin code for accessing the currently available project extensions and conventions.
                outgoingVariants - Displays the outgoing variants of root project 'demo'.
                projects - Displays the sub-projects of root project 'demo'.
                properties - Displays the properties of root project 'demo'.
                resolvableConfigurations - Displays the configurations that can be resolved in root project 'demo'.
                tasks - Displays the tasks runnable from root project 'demo'.

                Verification tasks
                ------------------
                check - Runs all checks.
                test - Runs the test suite.

                Weird tasks
                -----------
                bärchen - Contains an umlaut.
                1337 - Only consists of numbers.
                one-two - Contains a dash.

                Rules
                -----
                Pattern: clean<TaskName>: Cleans the output files of a task.
                Pattern: build<ConfigurationName>: Assembles the artifacts of a configuration.

                To see all tasks and more detail, run gradlew tasks --all

                To see more detail about a task, run gradlew help --task <task>
            "};

            let result = super::parse(input).unwrap();

            assert_eq!(
                result,
                vec![
                    TaskGroup { heading: String::from("Application tasks"), tasks: vec![Task { name: String::from("bootRun"), description: String::from("Runs this project as a Spring Boot application.") }] },
                    TaskGroup { heading: String::from("Build tasks"), tasks: vec![Task { name: String::from("assemble"), description: String::from("Assembles the outputs of this project.") }, Task { name: String::from("bootBuildImage"), description: String::from("Builds an OCI image of the application using the output of the bootJar task") }, Task { name: String::from("bootJar"), description: String::from("Assembles an executable jar archive containing the main classes and their dependencies.") }, Task { name: String::from("build"), description: String::from("Assembles and tests this project.") }, Task { name: String::from("buildDependents"), description: String::from("Assembles and tests this project and all projects that depend on it.") }, Task { name: String::from("buildNeeded"), description: String::from("Assembles and tests this project and all projects it depends on.") }, Task { name: String::from("classes"), description: String::from("Assembles main classes.") }, Task { name: String::from("clean"), description: String::from("Deletes the build directory.") }, Task { name: String::from("jar"), description: String::from("Assembles a jar archive containing the main classes.") }, Task { name: String::from("resolveMainClassName"), description: String::from("Resolves the name of the application's main class.") }, Task { name: String::from("testClasses"), description: String::from("Assembles test classes.") }] },
                    TaskGroup { heading: String::from("Build Setup tasks"), tasks: vec![Task { name: String::from("init"), description: String::from("Initializes a new Gradle build.") }, Task { name: String::from("wrapper"), description: String::from("Generates Gradle wrapper files.") }] },
                    TaskGroup { heading: String::from("Documentation tasks"), tasks: vec![Task { name: String::from("javadoc"), description: String::from("Generates Javadoc API documentation for the main source code.") }] },
                    TaskGroup { heading: String::from("Help tasks"), tasks: vec![Task { name: String::from("buildEnvironment"), description: String::from("Displays all buildscript dependencies declared in root project 'demo'.") }, Task { name: String::from("dependencies"), description: String::from("Displays all dependencies declared in root project 'demo'.") }, Task { name: String::from("dependencyInsight"), description: String::from("Displays the insight into a specific dependency in root project 'demo'.") }, Task { name: String::from("dependencyManagement"), description: String::from("Displays the dependency management declared in root project 'demo'.") }, Task { name: String::from("help"), description: String::from("Displays a help message.") }, Task { name: String::from("javaToolchains"), description: String::from("Displays the detected java toolchains.") }, Task { name: String::from("kotlinDslAccessorsReport"), description: String::from("Prints the Kotlin code for accessing the currently available project extensions and conventions.") }, Task { name: String::from("outgoingVariants"), description: String::from("Displays the outgoing variants of root project 'demo'.") }, Task { name: String::from("projects"), description: String::from("Displays the sub-projects of root project 'demo'.") }, Task { name: String::from("properties"), description: String::from("Displays the properties of root project 'demo'.") }, Task { name: String::from("resolvableConfigurations"), description: String::from("Displays the configurations that can be resolved in root project 'demo'.") }, Task { name: String::from("tasks"), description: String::from("Displays the tasks runnable from root project 'demo'.") }] },
                    TaskGroup { heading: String::from("Verification tasks"), tasks: vec![Task { name: String::from("check"), description: String::from("Runs all checks.") }, Task { name: String::from("test"), description: String::from("Runs the test suite.") }] },
                    TaskGroup { heading: String::from("Weird tasks"), tasks: vec![Task { name: String::from("bärchen"), description: String::from("Contains an umlaut.") }, Task { name: String::from("1337"), description: String::from("Only consists of numbers.") },  Task { name: String::from("one-two"), description: String::from("Contains a dash.") }] }
                ]
            );
        }
    }
}
