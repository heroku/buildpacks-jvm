# Heroku Cloud Native Java Buildpack
[![CircleCI](https://circleci.com/gh/heroku/buildpacks-jvm/tree/main.svg?style=shield)](https://circleci.com/gh/heroku/buildpacks-jvm/tree/main)
[![Registry](https://img.shields.io/badge/dynamic/json?url=https://registry.buildpacks.io/api/v1/buildpacks/heroku/java&label=version&query=$.latest.version&color=DF0A6B&logo=data:image/png;base64,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&labelColor=white)](https://registry.buildpacks.io/buildpacks/heroku/java)

Heroku's official Cloud Native Buildpack for Java applications.

## Table of Contents
* [How it works](#how-it-works)
    + [How it works: Maven](#how-it-works-maven)
    + [How it works: Gradle](#how-it-works-gradle)
* [Configuration](#configuration)
    + [Procfile](#procfile)
    + [OpenJDK](#openjdk)
        - [OpenJDK Version](#openjdk-version)
            * [`system.properties`](#systemproperties)
            * [Environment variable: `BP_JVM_VERSION`](#environment-variable-bp_jvm_version)
        - [Customizing the JDK](#customizing-the-jdk)
            * [Adding custom certificates](#adding-custom-certificates)
        - [Customizing runtime JVM flags](#customizing-runtime-jvm-flags)
    + [Maven](#maven)
        - [Maven Version](#maven-version)
        - [Customizing Maven execution](#customizing-maven-execution)
            * [Environment variable: `MAVEN_CUSTOM_GOALS`](#environment-variable-maven_custom_goals)
            * [Environment variable: `MAVEN_CUSTOM_OPTS`](#environment-variable-maven_custom_opts)
            * [Environment variable: `MAVEN_JAVA_OPTS`](#environment-variable-maven_java_opts)
        - [Using a custom Maven settings file](#using-a-custom-maven-settings-file)
            * [Environment variable: `MAVEN_SETTINGS_PATH`](#environment-variable-maven_settings_path)
            * [Environment variable: `MAVEN_SETTINGS_URL`](#environment-variable-maven_settings_url)
    + [Gradle](#gradle)
        - [Gradle Version](#gradle-version)
        - [Customizing Gradle execution](#customizing-gradle-execution)
            * [Environment variable: `GRADLE_TASK`](#environment-variable-gradle_task)
* [License](#license)

## How it works

This buildpack will install OpenJDK 8 ([Configuration: OpenJDK Version](#openjdk-version)) and builds your app with
[Maven](https://maven.apache.org/) or [Gradle](https://gradle.org/). It requires either:

- A `pom.xml` file, or one of the other POM formats supported by the
  [Maven Polyglot plugin](https://github.com/takari/polyglot-maven) in the root directory of your app. See
  [How it works: Maven](#how-it-works-maven) for Maven specifics.
- A `build.gradle` file and [Gradle wrapper](https://docs.gradle.org/current/userguide/gradle_wrapper.html) in the root
  directory of your app. See [How it works: Gradle](#how-it-works-gradle) for Gradle specifics.

The buildpack will try to figure out the required goals/tasks to run based on the framework used for your application.
It will also add a process type based on the framework. If required, those features can be configured:

- [Configuration: Customizing Maven execution](#customizing-maven-execution)
- [Configuration: Customizing Gradle execution](#customizing-gradle-execution)
- [Configuration: Procfile](#procfile)

### How it works: Maven
The buildpack will use the [Maven wrapper](https://github.com/takari/maven-wrapper) in your app's root directory to
execute the build defined by the POM and download your dependencies. If the application does not use Maven wrapper,
it will install Maven and use that instead.

The local Maven repository (`.m2` folder) will be cached between builds for dependency resolution. However, neither
the `mvn` executable (if installed) nor the `.m2` folder will be available in your container at runtime.

### How it works: Gradle
The buildpack will use the [Gradle wrapper](https://docs.gradle.org/current/userguide/gradle_wrapper.html) in your app's
root directory to execute the build defined by your `build.gradle` and download your dependencies. Gradle's
[dependency cache](https://docs.gradle.org/current/userguide/dependency_resolution.html#sec:dependency_cache) is
persisted between builds to speed up subsequent builds. However, the dependency cache will not be available in your
container at runtime.

## Configuration
### Procfile
A Procfile is a text file in the root directory of your application that defines process types and explicitly declares
what command should be executed to start your app. Your `Procfile` will look something like this for Spring Boot:

```
web: java -Dserver.port=$PORT $JAVA_OPTS -jar target/demo-0.0.1-SNAPSHOT.jar
```

### OpenJDK
#### OpenJDK Version
By default, the latest OpenJDK 8 release will be installed. You can configure the OpenJDK version your application
needs. The buildpack tries to determine the required version in the following order:

##### `system.properties`
You can specify a Java version by adding a [Java properties file](https://en.wikipedia.org/wiki/.properties) called
`system.properties` to the root directory of your application. The value of the `java.runtime.version` key specifies
the required OpenJDK version:

```
java.runtime.version=15
```

Supported major versions are `1.7`, `1.8`, `11`, `13`, and `15`. The buildpack will always install the latest release
of the requested major version.

##### Environment variable: `BP_JVM_VERSION`
You can use the same major version strings as in the `system.properties` file. In addition, it is allowed to append `.*`
to a major version (i.e. `11.*` for OpenJDK 11). This ensures compatibility with the
[Spring Boot Plugin](https://docs.spring.io/spring-boot/docs/2.4.1/maven-plugin/reference/htmlsingle/#goals-build-image).

#### Customizing the JDK
There are some cases where files need to be bundled with the JDK in order to expose functionality in the runtime JVM.
For example, the inclusion of a custom certificate authority (CA) store is common. To handle such cases, this buildpack
will copy files designated by the app in a `.jdk-overlay` folder into the JDK's directory structure.

##### Adding custom certificates
You may also need to add custom certificates to the JDK’s cacerts. You may start with the keystore in your local JDK or
[download the base Heroku keystore](https://heroku-cacerts.herokuapp.com/heroku_cacerts). Add the custom certificate
with:

```shell
keytool -import -keystore cacerts -file custom.cer
```

You may be prompted for a password. The default password is `changeit`. You may then include the keystore in the slug by
placing it in the `.jdk-overlay/jre/lib/security/` directory of your app’s repository
(or `.jdk-overlay/lib/security/` for Java 11 and higher).

#### Customizing runtime JVM flags
To set JVM flags during runtime, the `JAVA_TOOL_OPTIONS` environment variable can be used. It is directly supported by
Java and [intended to augment a command line in environments where the command-line cannot be accessed or modified](http://docs.oracle.com/javase/7/docs/platform/jvmti/jvmti.html#tooloptions).
Since it is automatically picked up by Java, you do not need to include it in your Procfile command.

### Maven
#### Maven Version
We encourage users to use [Maven wrapper](https://github.com/takari/maven-wrapper) to set the required Maven
version for their applications.

If you cannot or do not want to use Maven wrapper, you can specify the Maven version for
your application by adding (or extending) a [Java properties file](https://en.wikipedia.org/wiki/.properties) called
`system.properties` in the root directory of your application.

The `maven.version` key determines the Maven version that is installed. Currently, supported versions are `3.2.5`,
`3.3.9`, `3.5.3`, and `3.6.2`. The default is `3.6.2`.

```
maven.version=3.5.3
```

#### Customizing Maven execution
There is a set of environment variables that can be used to customize the Maven execution.

##### Environment variable: `MAVEN_CUSTOM_GOALS`
Allows overriding the Maven goals used during the build process. The default goals are `clean install`.

##### Environment variable: `MAVEN_CUSTOM_OPTS`
Allows overriding Maven options used during the build process. The default options are `-DskipTests`.

##### Environment variable: `MAVEN_JAVA_OPTS`
Allows overriding the Java options for the Maven process during build. The default Java options are `-Xmx1024m`.
See [Customizing runtime JVM flags](#customizing-runtime-jvm-flags) on how to set Java options during runtime.

#### Using a custom Maven settings file
A Maven `settings.xml` file defines values that configure Maven execution in various ways. Most commonly, it is used to
define a local repository location, alternate remote repository servers, and authentication information for private
repositories.

When a file named `settings.xml` is present in the root directory of the application, the buildpack will automatically
use it to configure Maven at build time.

##### Environment variable: `MAVEN_SETTINGS_PATH`
If you do not want the `settings.xml` file in the root directory or if you intend to frequently change between different
setting configurations, you may prefer to put a settings file in a custom location. The buildpack provides this
capability with the `MAVEN_SETTINGS_PATH` environment variable.

##### Environment variable: `MAVEN_SETTINGS_URL`
When the `MAVEN_SETTINGS_URL` config variable is defined, the buildpack will download the file at the given location
and use it to configure Maven.

### Gradle
#### Gradle Version
The buildpack will not install Gradle itself and requires that your application uses
[Gradle wrapper](https://docs.gradle.org/current/userguide/gradle_wrapper.html) to configure the Gradle version.

#### Customizing Gradle execution
There is a set of environment variables that can be used to customize the Gradle execution.

##### Environment variable: `GRADLE_TASK`
This buildpack tries to determine the correct Gradle task to run based on the framework you use. In some cases, you
might not use a framework or use one that is not directly supported. In such cases, the `stage` task will be executed
by default. To use another task, set the `GRADLE_TASK` environment variable to the task you want to execute for building
your application.

## License
Licensed under the MIT License. See [LICENSE](../../LICENSE) file.
