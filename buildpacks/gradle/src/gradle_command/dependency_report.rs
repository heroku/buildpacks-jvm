use crate::gradle_command::init::gradle_init_script_args;
use crate::gradle_command::GradleCommandError;
use libcnb::Env;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

pub(crate) fn dependency_report(
    app_dir: &Path,
    env: &Env,
    gradle_init_scripts: &[PathBuf],
) -> Result<GradleDependencyReport, GradleCommandError<()>> {
    let output = Command::new(app_dir.join("gradlew"))
        .current_dir(app_dir)
        .envs(env)
        .args(gradle_init_script_args(gradle_init_scripts))
        .args(["--quiet", "dependencies"])
        .output()
        .map_err(GradleCommandError::Io)?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if output.status.success() {
        parser::dependency_report(&stdout)
            .map_err(|_| GradleCommandError::Parse(()))
            .map(|(_, dependency_report)| dependency_report)
    } else {
        Err(GradleCommandError::UnexpectedExitStatus {
            status: output.status,
            stdout: stdout.into_owned(),
            stderr: stderr.into_owned(),
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct GradleDependencyReport {
    pub(crate) entries: BTreeMap<String, Vec<Dependency>>,
}

impl GradleDependencyReport {
    pub(crate) fn contains_dependency(
        &self,
        configuration_name: &str,
        group_id: &str,
        artifact_id: &str,
    ) -> bool {
        self.flattened_dependencies(configuration_name)
            .unwrap_or_default()
            .into_iter()
            .any(|dependency| {
                dependency.group_id == group_id && dependency.artifact_id == artifact_id
            })
    }

    fn flattened_dependencies(&self, configuration_name: &str) -> Option<Vec<Dependency>> {
        self.entries.get(configuration_name).map(|dependencies| {
            let mut acc = vec![];

            for dependency in dependencies {
                acc.append(&mut dependency.flatten());
            }

            acc
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct Dependency {
    pub(crate) group_id: String,
    pub(crate) artifact_id: String,
    pub(crate) package_version: Option<String>,
    pub(crate) resolved_package_version: Option<String>,
    pub(crate) suffix: Option<Suffix>,
    pub(crate) dependencies: Vec<Dependency>,
}

impl Dependency {
    fn flatten(&self) -> Vec<Dependency> {
        let mut acc = vec![];
        acc.push(Dependency {
            group_id: self.group_id.clone(),
            artifact_id: self.artifact_id.clone(),
            package_version: self.package_version.clone(),
            resolved_package_version: self.resolved_package_version.clone(),
            suffix: self.suffix,
            dependencies: vec![],
        });

        for transitive_dependency in &self.dependencies {
            acc.append(&mut transitive_dependency.flatten());
        }

        acc
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum Suffix {
    DependencyConstraint,
    DependenciesOmitted,
    NotResolved,
}

#[derive(Debug)]
enum ParseError {}

mod parser {
    use super::{Dependency, GradleDependencyReport, Suffix};
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{alphanumeric1, char, line_ending, newline, not_line_ending};
    use nom::combinator::{map, opt, recognize};
    use nom::multi::{count, many0, many1, many_till};
    use nom::sequence::{delimited, preceded, terminated, tuple};
    use nom::IResult;

    pub(crate) fn dependency_report(report: &str) -> IResult<&str, GradleDependencyReport> {
        let configuration_name_and_dependencies = tuple((
            configuration_line,
            alt((
                map(tag("No dependencies"), |_| Vec::new()),
                many1(dependency_tree(1)),
            )),
        ));

        map(
            many0(map(
                many_till(any_line, configuration_name_and_dependencies),
                |(_, parsed_configuration)| parsed_configuration,
            )),
            |entries| GradleDependencyReport {
                entries: entries.into_iter().collect(),
            },
        )(report)
    }

    fn configuration_line(input: &str) -> IResult<&str, String> {
        map(
            terminated(alphanumeric1, terminated(not_line_ending, line_ending)),
            String::from,
        )(input)
    }

    fn any_line(input: &str) -> IResult<&str, String> {
        map(terminated(not_line_ending, line_ending), String::from)(input)
    }

    fn dependency_tree(start_at_depth: usize) -> impl FnMut(&str) -> IResult<&str, Dependency> {
        move |input: &str| {
            preceded(
                count(tree_depth_indicator, start_at_depth),
                map(
                    tuple((
                        terminated(
                            tuple((
                                group_or_artifact_id,
                                preceded(char(':'), group_or_artifact_id),
                                opt(preceded(char(':'), package_version)),
                                opt(preceded(tag(" -> "), package_version)),
                                opt(preceded(char(' '), dependency_suffix)),
                            )),
                            newline,
                        ),
                        many0(dependency_tree(start_at_depth + 1)),
                    )),
                    |(
                        (group_id, artifact_id, package_version, resolved_package_version, suffix),
                        children,
                    )| Dependency {
                        group_id,
                        artifact_id,
                        package_version,
                        resolved_package_version,
                        suffix,
                        dependencies: children,
                    },
                ),
            )(input)
        }
    }

    fn dependency_suffix(input: &str) -> IResult<&str, Suffix> {
        delimited(
            char('('),
            alt((
                map(char('c'), |_| Suffix::DependencyConstraint),
                map(char('*'), |_| Suffix::DependenciesOmitted),
                map(char('n'), |_| Suffix::NotResolved),
            )),
            char(')'),
        )(input)
    }

    fn tree_depth_indicator(input: &str) -> IResult<&str, &str> {
        alt((tag("+--- "), tag("|    "), tag("     "), tag("\\--- ")))(input)
    }

    fn group_or_artifact_id(input: &str) -> IResult<&str, String> {
        map(
            recognize(many1(alt((alphanumeric1, tag("_"), tag("-"), tag("."))))),
            String::from,
        )(input)
    }

    fn package_version(input: &str) -> IResult<&str, String> {
        map(
            recognize(many1(alt((alphanumeric1, tag("_"), tag("-"), tag("."))))),
            String::from,
        )(input)
    }

    #[cfg(test)]
    mod test {
        use super::Suffix::NotResolved;
        use super::{Dependency, GradleDependencyReport};
        use indoc::indoc;
        use std::collections::BTreeMap;

        #[test]
        #[allow(clippy::too_many_lines)]
        fn test() {
            let result = super::dependency_report(indoc! {r"
            ------------------------------------------------------------
            Project ':app'
            ------------------------------------------------------------
            
            annotationProcessor - Annotation processors and their dependencies for source set 'main'.
            No dependencies
            
            apiElements - API elements for main. (n)
            No dependencies
            
            archives - Configuration for archive artifacts. (n)
            No dependencies
            
            compileClasspath - Compile classpath for source set 'main'.
            \--- com.google.guava:guava:30.1.1-jre
                 +--- com.google.guava:failureaccess:1.0.1
                 +--- com.google.guava:listenablefuture:9999.0-empty-to-avoid-conflict-with-guava
                 +--- com.google.code.findbugs:jsr305:3.0.2
                 +--- org.checkerframework:checker-qual:3.8.0
                 +--- com.google.errorprone:error_prone_annotations:2.5.1
                 \--- com.google.j2objc:j2objc-annotations:1.3
            
            compileOnly - Compile only dependencies for source set 'main'. (n)
            No dependencies
            
            default - Configuration for default artifacts. (n)
            No dependencies
            
            implementation - Implementation only dependencies for source set 'main'. (n)
            \--- com.google.guava:guava:30.1.1-jre (n)
            
            mainSourceElements - List of source directories contained in the Main SourceSet. (n)
            No dependencies
            
            runtimeClasspath - Runtime classpath of source set 'main'.
            \--- com.google.guava:guava:30.1.1-jre
                 +--- com.google.guava:failureaccess:1.0.1
                 +--- com.google.guava:listenablefuture:9999.0-empty-to-avoid-conflict-with-guava
                 +--- com.google.code.findbugs:jsr305:3.0.2
                 +--- org.checkerframework:checker-qual:3.8.0
                 +--- com.google.errorprone:error_prone_annotations:2.5.1
                 \--- com.google.j2objc:j2objc-annotations:1.3
            
            runtimeElements - Elements of runtime for main. (n)
            No dependencies
            
            runtimeOnly - Runtime only dependencies for source set 'main'. (n)
            No dependencies
            
            testAnnotationProcessor - Annotation processors and their dependencies for source set 'test'.
            No dependencies
            
            testCompileClasspath - Compile classpath for source set 'test'.
            +--- com.google.guava:guava:30.1.1-jre
            |    +--- com.google.guava:failureaccess:1.0.1
            |    +--- com.google.guava:listenablefuture:9999.0-empty-to-avoid-conflict-with-guava
            |    +--- com.google.code.findbugs:jsr305:3.0.2
            |    +--- org.checkerframework:checker-qual:3.8.0
            |    +--- com.google.errorprone:error_prone_annotations:2.5.1
            |    \--- com.google.j2objc:j2objc-annotations:1.3
            \--- junit:junit:4.13.2
                 \--- org.hamcrest:hamcrest-core:1.3
            
            testCompileOnly - Compile only dependencies for source set 'test'. (n)
            No dependencies
            
            testImplementation - Implementation only dependencies for source set 'test'. (n)
            \--- junit:junit:4.13.2 (n)
            
            testResultsElementsForTest - Directory containing binary results of running tests for the test Test Suite's test target. (n)
            No dependencies
            
            testRuntimeClasspath - Runtime classpath of source set 'test'.
            +--- com.google.guava:guava:30.1.1-jre
            |    +--- com.google.guava:failureaccess:1.0.1
            |    +--- com.google.guava:listenablefuture:9999.0-empty-to-avoid-conflict-with-guava
            |    +--- com.google.code.findbugs:jsr305:3.0.2
            |    +--- org.checkerframework:checker-qual:3.8.0
            |    +--- com.google.errorprone:error_prone_annotations:2.5.1
            |    \--- com.google.j2objc:j2objc-annotations:1.3
            \--- junit:junit:4.13.2
                 \--- org.hamcrest:hamcrest-core:1.3
            
            testRuntimeOnly - Runtime only dependencies for source set 'test'. (n)
            No dependencies
            
            (n) - Not resolved (configuration is not meant to be resolved)
            
            A web-based, searchable dependency report is available by adding the --scan option.
            "});

            assert_eq!(
                result.unwrap().1,
                GradleDependencyReport {
                    entries: BTreeMap::from([
                        (String::from("annotationProcessor"), vec![]),
                        (String::from("apiElements"), vec![]),
                        (String::from("archives"), vec![]),
                        (
                            String::from("compileClasspath"),
                            vec![Dependency {
                                group_id: String::from("com.google.guava"),
                                artifact_id: String::from("guava"),
                                package_version: Some(String::from("30.1.1-jre")),
                                resolved_package_version: None,
                                suffix: None,
                                dependencies: vec![
                                    Dependency {
                                        group_id: String::from("com.google.guava"),
                                        artifact_id: String::from("failureaccess"),
                                        package_version: Some(String::from("1.0.1")),
                                        resolved_package_version: None,
                                        suffix: None,
                                        dependencies: vec![]
                                    },
                                    Dependency {
                                        group_id: String::from("com.google.guava"),
                                        artifact_id: String::from("listenablefuture"),
                                        package_version: Some(String::from(
                                            "9999.0-empty-to-avoid-conflict-with-guava"
                                        )),
                                        resolved_package_version: None,
                                        suffix: None,
                                        dependencies: vec![]
                                    },
                                    Dependency {
                                        group_id: String::from("com.google.code.findbugs"),
                                        artifact_id: String::from("jsr305"),
                                        package_version: Some(String::from("3.0.2")),
                                        resolved_package_version: None,
                                        suffix: None,
                                        dependencies: vec![]
                                    },
                                    Dependency {
                                        group_id: String::from("org.checkerframework"),
                                        artifact_id: String::from("checker-qual"),
                                        package_version: Some(String::from("3.8.0")),
                                        resolved_package_version: None,
                                        suffix: None,
                                        dependencies: vec![]
                                    },
                                    Dependency {
                                        group_id: String::from("com.google.errorprone"),
                                        artifact_id: String::from("error_prone_annotations"),
                                        package_version: Some(String::from("2.5.1")),
                                        resolved_package_version: None,
                                        suffix: None,
                                        dependencies: vec![]
                                    },
                                    Dependency {
                                        group_id: String::from("com.google.j2objc"),
                                        artifact_id: String::from("j2objc-annotations"),
                                        package_version: Some(String::from("1.3")),
                                        resolved_package_version: None,
                                        suffix: None,
                                        dependencies: vec![]
                                    }
                                ]
                            }]
                        ),
                        (String::from("compileOnly"), vec![]),
                        (String::from("default"), vec![]),
                        (
                            String::from("implementation"),
                            vec![Dependency {
                                group_id: String::from("com.google.guava"),
                                artifact_id: String::from("guava"),
                                package_version: Some(String::from("30.1.1-jre")),
                                resolved_package_version: None,
                                suffix: Some(NotResolved),
                                dependencies: vec![]
                            }]
                        ),
                        (String::from("mainSourceElements"), vec![]),
                        (
                            String::from("runtimeClasspath"),
                            vec![Dependency {
                                group_id: String::from("com.google.guava"),
                                artifact_id: String::from("guava"),
                                package_version: Some(String::from("30.1.1-jre")),
                                resolved_package_version: None,
                                suffix: None,
                                dependencies: vec![
                                    Dependency {
                                        group_id: String::from("com.google.guava"),
                                        artifact_id: String::from("failureaccess"),
                                        package_version: Some(String::from("1.0.1")),
                                        resolved_package_version: None,
                                        suffix: None,
                                        dependencies: vec![]
                                    },
                                    Dependency {
                                        group_id: String::from("com.google.guava"),
                                        artifact_id: String::from("listenablefuture"),
                                        package_version: Some(String::from(
                                            "9999.0-empty-to-avoid-conflict-with-guava"
                                        )),
                                        resolved_package_version: None,
                                        suffix: None,
                                        dependencies: vec![]
                                    },
                                    Dependency {
                                        group_id: String::from("com.google.code.findbugs"),
                                        artifact_id: String::from("jsr305"),
                                        package_version: Some(String::from("3.0.2")),
                                        resolved_package_version: None,
                                        suffix: None,
                                        dependencies: vec![]
                                    },
                                    Dependency {
                                        group_id: String::from("org.checkerframework"),
                                        artifact_id: String::from("checker-qual"),
                                        package_version: Some(String::from("3.8.0")),
                                        resolved_package_version: None,
                                        suffix: None,
                                        dependencies: vec![]
                                    },
                                    Dependency {
                                        group_id: String::from("com.google.errorprone"),
                                        artifact_id: String::from("error_prone_annotations"),
                                        package_version: Some(String::from("2.5.1")),
                                        resolved_package_version: None,
                                        suffix: None,
                                        dependencies: vec![]
                                    },
                                    Dependency {
                                        group_id: String::from("com.google.j2objc"),
                                        artifact_id: String::from("j2objc-annotations"),
                                        package_version: Some(String::from("1.3")),
                                        resolved_package_version: None,
                                        suffix: None,
                                        dependencies: vec![]
                                    }
                                ]
                            }]
                        ),
                        (String::from("runtimeElements"), vec![]),
                        (String::from("runtimeOnly"), vec![]),
                        (String::from("testAnnotationProcessor"), vec![]),
                        (
                            String::from("testCompileClasspath"),
                            vec![
                                Dependency {
                                    group_id: String::from("com.google.guava"),
                                    artifact_id: String::from("guava"),
                                    package_version: Some(String::from("30.1.1-jre")),
                                    resolved_package_version: None,
                                    suffix: None,
                                    dependencies: vec![
                                        Dependency {
                                            group_id: String::from("com.google.guava"),
                                            artifact_id: String::from("failureaccess"),
                                            package_version: Some(String::from("1.0.1")),
                                            resolved_package_version: None,
                                            suffix: None,
                                            dependencies: vec![]
                                        },
                                        Dependency {
                                            group_id: String::from("com.google.guava"),
                                            artifact_id: String::from("listenablefuture"),
                                            package_version: Some(String::from(
                                                "9999.0-empty-to-avoid-conflict-with-guava"
                                            )),
                                            resolved_package_version: None,
                                            suffix: None,
                                            dependencies: vec![]
                                        },
                                        Dependency {
                                            group_id: String::from("com.google.code.findbugs"),
                                            artifact_id: String::from("jsr305"),
                                            package_version: Some(String::from("3.0.2")),
                                            resolved_package_version: None,
                                            suffix: None,
                                            dependencies: vec![]
                                        },
                                        Dependency {
                                            group_id: String::from("org.checkerframework"),
                                            artifact_id: String::from("checker-qual"),
                                            package_version: Some(String::from("3.8.0")),
                                            resolved_package_version: None,
                                            suffix: None,
                                            dependencies: vec![]
                                        },
                                        Dependency {
                                            group_id: String::from("com.google.errorprone"),
                                            artifact_id: String::from("error_prone_annotations"),
                                            package_version: Some(String::from("2.5.1")),
                                            resolved_package_version: None,
                                            suffix: None,
                                            dependencies: vec![]
                                        },
                                        Dependency {
                                            group_id: String::from("com.google.j2objc"),
                                            artifact_id: String::from("j2objc-annotations"),
                                            package_version: Some(String::from("1.3")),
                                            resolved_package_version: None,
                                            suffix: None,
                                            dependencies: vec![]
                                        }
                                    ]
                                },
                                Dependency {
                                    group_id: String::from("junit"),
                                    artifact_id: String::from("junit"),
                                    package_version: Some(String::from("4.13.2")),
                                    resolved_package_version: None,
                                    suffix: None,
                                    dependencies: vec![Dependency {
                                        group_id: String::from("org.hamcrest"),
                                        artifact_id: String::from("hamcrest-core"),
                                        package_version: Some(String::from("1.3")),
                                        resolved_package_version: None,
                                        suffix: None,
                                        dependencies: vec![]
                                    }]
                                }
                            ]
                        ),
                        (String::from("testCompileOnly"), vec![]),
                        (
                            String::from("testImplementation"),
                            vec![Dependency {
                                group_id: String::from("junit"),
                                artifact_id: String::from("junit"),
                                package_version: Some(String::from("4.13.2")),
                                resolved_package_version: None,
                                suffix: Some(NotResolved),
                                dependencies: vec![]
                            }]
                        ),
                        (String::from("testResultsElementsForTest"), vec![]),
                        (
                            String::from("testRuntimeClasspath"),
                            vec![
                                Dependency {
                                    group_id: String::from("com.google.guava"),
                                    artifact_id: String::from("guava"),
                                    package_version: Some(String::from("30.1.1-jre")),
                                    resolved_package_version: None,
                                    suffix: None,
                                    dependencies: vec![
                                        Dependency {
                                            group_id: String::from("com.google.guava"),
                                            artifact_id: String::from("failureaccess"),
                                            package_version: Some(String::from("1.0.1")),
                                            resolved_package_version: None,
                                            suffix: None,
                                            dependencies: vec![]
                                        },
                                        Dependency {
                                            group_id: String::from("com.google.guava"),
                                            artifact_id: String::from("listenablefuture"),
                                            package_version: Some(String::from(
                                                "9999.0-empty-to-avoid-conflict-with-guava"
                                            )),
                                            resolved_package_version: None,
                                            suffix: None,
                                            dependencies: vec![]
                                        },
                                        Dependency {
                                            group_id: String::from("com.google.code.findbugs"),
                                            artifact_id: String::from("jsr305"),
                                            package_version: Some(String::from("3.0.2")),
                                            resolved_package_version: None,
                                            suffix: None,
                                            dependencies: vec![]
                                        },
                                        Dependency {
                                            group_id: String::from("org.checkerframework"),
                                            artifact_id: String::from("checker-qual"),
                                            package_version: Some(String::from("3.8.0")),
                                            resolved_package_version: None,
                                            suffix: None,
                                            dependencies: vec![]
                                        },
                                        Dependency {
                                            group_id: String::from("com.google.errorprone"),
                                            artifact_id: String::from("error_prone_annotations"),
                                            package_version: Some(String::from("2.5.1")),
                                            resolved_package_version: None,
                                            suffix: None,
                                            dependencies: vec![]
                                        },
                                        Dependency {
                                            group_id: String::from("com.google.j2objc"),
                                            artifact_id: String::from("j2objc-annotations"),
                                            package_version: Some(String::from("1.3")),
                                            resolved_package_version: None,
                                            suffix: None,
                                            dependencies: vec![]
                                        }
                                    ]
                                },
                                Dependency {
                                    group_id: String::from("junit"),
                                    artifact_id: String::from("junit"),
                                    package_version: Some(String::from("4.13.2")),
                                    resolved_package_version: None,
                                    suffix: None,
                                    dependencies: vec![Dependency {
                                        group_id: String::from("org.hamcrest"),
                                        artifact_id: String::from("hamcrest-core"),
                                        package_version: Some(String::from("1.3")),
                                        resolved_package_version: None,
                                        suffix: None,
                                        dependencies: vec![]
                                    }]
                                }
                            ]
                        ),
                        (String::from("testRuntimeOnly"), vec![]),
                    ])
                }
            );
        }
    }
}
