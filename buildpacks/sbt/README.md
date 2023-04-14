# Heroku Cloud Native sbt Buildpack
[![CI](https://github.com/heroku/buildpacks-jvm/actions/workflows/ci.yml/badge.svg)](https://github.com/heroku/buildpacks-jvm/actions/workflows/ci.yml)

Heroku's official Cloud Native Buildpack for [sbt](https://www.scala-sbt.org/) usage in [Scala](https://www.scala-lang.org/) applications.

## How it works

The buildpack will detect if your application requires `sbt` if any one of the following file patterns match:
- `project/build.properties`
- `project/*.scala`
- `*.sbt`
- `.sbt/*.scala`

### Step 1: Download sbt

An [sbt wrapper script](https://github.com/dwijnand/sbt-extras) is written into its own layer that will contain the sbt
build tooling. This wrapper script then executes to download the sbt version specified in your `project/build.properties` file.

### Step 2: Run sbt

By default, the [sbt](https://www.scala-sbt.org/index.html) command used to build the application is `sbt compile stage`.
Applications that require customizations to this process to build successfully should refer to the [Customizing](#customizing)
section of this document.

## Customizing

This buildpack exposes the following configurable settings. The order they appear in the table indicates their precedence.

> For example, if your application contains:
> * a `system.properties` file with the `sbt.project` property set
> * and the environment variable `SBT_PROJECT` is set
>
> Then the value from `system.properties` will be used.

### Configure a sbt subproject build

When this setting is configured, the default build tasks will be prepended with the supplied project name. E.g.; the default
`compile` and `stage` tasks would become `{subproject}/compile` and `{subproject}/stage`.

| From               | Path                | Name          |
|--------------------|---------------------|---------------|
| Java property file | `system.properties` | `sbt.project` |
| Environment        |                     | `SBT_PROJECT` |

### Specify tasks to execute before building

This setting will prepend the supplied tasks to the list of tasks to run during the build. These must be supplied as
a string of space-separated task names. E.g.; a value of `task1 task2` would cause the build step to be invoked with
`sbt task1 task2 compile stage`.

| From               | Path                | Name            |
|--------------------|---------------------|-----------------|
| Java property file | `system.properties` | `sbt.pre-tasks` |
| Environment        |                     | `SBT_PRE_TASKS` |

### Specify which build tasks to use

This setting will override the default build tasks of `compile` and `stage`. These must be supplied as
a string of space-separated task names. E.g.; a value of `mybuild` would cause the build step to be invoked with
`sbt mybuild`.

| From               | Path                | Name        |
|--------------------|---------------------|-------------|
| Java property file | `system.properties` | `sbt.tasks` |
| Environment        |                     | `SBT_TASKS` |

### Cleaning the project before build

This setting will prepend a `clean` task to before all other tasks to run during the build. This must be supplied as a
value of either `true` or `false`. E.g.; setting this value to `true` would cause the build step to be invoked with
`sbt clean compile stage`.

| From               | Path                | Name        |
|--------------------|---------------------|-------------|
| Java property file | `system.properties` | `sbt.clean` |
| Environment        |                     | `SBT_CLEAN` |

### Making sbt available at launch

By default, the `sbt` executable as well as its caches are only available during the build process.  If you need
`sbt` to launch your application you can configure this setting with a value of `true`.

| From               | Path                | Name                      |
|--------------------|---------------------|---------------------------|
| Java property file | `system.properties` | `sbt.available-at-launch` |
| Environment        |                     | `SBT_AVAILABLE_AT_LAUNCH` |

### Adding custom sbt options

If the `SBT_OPTS` environment variable is defined when sbt starts, its content are passed as command line arguments to
the JVM running sbt.

If a file named `.sbtopts` exists, its content is appended to `SBT_OPTS`.

When passing options to the underlying sbt JVM, you must prefix them with `-J`. Thus, setting stack size for the compile
process would look like this:

```sh
heroku config:set SBT_OPTS="-J-Xss4m"
```

| From         | Path                | Name       |
|--------------|---------------------|------------|
| Environment  |                     | `SBT_OPTS` |
| Options file | `.sbtopts`          |            |


## Build Plan

### Requires

* `jdk`: To compile Java sources a JDK is required. It can be provided by the `heroku/jvm` ([Source](/buildpacks/jvm),
[Readme](/buildpacks/jvm/README.md)) buildpack.
* `jvm-application`: This is not a strict requirement of the buildpack. Requiring `jvm-application` ensures that this
buildpack can be used even when no other buildpack requires `jvm-application`.

### Provides

* `jvm-application`: Allows other buildpacks to depend on a compiled JVM application.

## License
See [LICENSE](../../LICENSE) file.
