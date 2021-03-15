# Heroku Cloud Native Maven Buildpack
[![CircleCI](https://circleci.com/gh/heroku/buildpacks-jvm/tree/main.svg?style=shield)](https://circleci.com/gh/heroku/buildpacks-jvm/tree/main)
[![Registry](https://img.shields.io/badge/dynamic/json?url=https://registry.buildpacks.io/api/v1/buildpacks/heroku/maven&label=version&query=$.latest.version&color=DF0A6B&logo=data:image/png;base64,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&labelColor=white)](https://registry.buildpacks.io/buildpacks/heroku/maven)

Heroku's official Cloud Native Buildpack for [Apache Maven](https://maven.apache.org/).

This buildpack is designed to work in conjunction with other Heroku buildpacks and cannot be used independently. If you
want to build a Java application, use the `heroku/java` buildpack ([Source](/meta-buildpacks/java),
[Readme](/meta-buildpacks/java/README.md)) which includes this Maven buildpack.

## How it works
### Step 1: Download Maven
If the application does not contain Maven Wrapper, the buildpack will download Maven and install it in its own layer.

Users can specify the Maven version for their application by adding (or extending) a
[Java properties file](https://en.wikipedia.org/wiki/.properties) called `system.properties` in the root directory of
the application.

The `maven.version` key determines the Maven version that is installed. Currently, supported versions are `3.2.5`,
`3.3.9`, `3.5.3`, and `3.6.2`. The default is `3.6.2`.

### Step 2: Resolve settings.xml
A Maven `settings.xml` file defines values that configure Maven execution in various ways. Most commonly, it is used to
define a local repository location, alternate remote repository servers, and authentication information for private
repositories.

When a file named `settings.xml` is present in the root directory of the application, the buildpack will automatically
use it to configure Maven at build time. The environment variable [MAVEN\_SETTINGS\_PATH](#MAVEN_SETTINGS_PATH) can be
used customize the file location.

In addition, the [MAVEN\_SETTINGS\_URL](#MAVEN_SETTINGS_URL) environment variable can be used to instruct the buildpack
to download a `settings.xml` file from a remote host via HTTPS.

### Step 3: Run Maven build
By default, the Maven command used to build the application is `mvn clean install -DskipTests`. Users can customize
this with the [MAVEN\_CUSTOM\_GOALS](#MAVEN_CUSTOM_GOALS) and [MAVEN\_CUSTOM\_OPTS](#MAVEN_CUSTOM_OPTS) environment
variables.

In addition, some extra configuration is used to ensure Maven stores the local repository in a dedicated layer.

### Step 4: Generate target/mvn-dependency-list.log
This buildpack will create a `target/mvn-dependency-list.log` in the application directory that can be used to later
determine which dependencies (including transitive ones) have been installed during the build.

### Step 5: launch.toml
For applications that use Spring Boot or Wildfly Swarm, this buildpack will generate a `launch.toml` with a `web` process
type to launch the application.


## Reference
### Detect
Requires either `pom.xml`, `pom.atom`, `pom.clj`, `pom.groovy`, `pom.rb`, `pom.scala`, `pom.yaml`, or `pom.yml` at the
root of the application source.

### Build Plan
#### Requires
##### `jdk`
To compile Java sources a JDK is required. It can be provided by the `heroku/jvm` ([Source](/buildpacks/jvm),
[Readme](/buildpacks/jvm/README.md)) buildpack.

##### `jvm-application`
This is not a strict requirement of the buildpack. Requiring `jvm-application` ensures that this Maven buildpack can be
used even when no other buildpack requires `jvm-application.`

#### Provides
##### `jvm-application`
Allows other buildpacks to depend on a compiled JVM application.

### Environment Variables
#### `MAVEN_SETTINGS_PATH`
If you do not want the `settings.xml` file in the root directory or if you intend to frequently change between different
setting configurations, you may prefer to put a settings file in a custom location. The buildpack provides this
capability with the `MAVEN_SETTINGS_PATH` environment variable.
#### `MAVEN_SETTINGS_URL`
When the `MAVEN_SETTINGS_URL` config variable is defined, the buildpack will download the file at the given location
and use it to configure Maven.
#### `MAVEN_CUSTOM_OPTS`
Allows overriding Maven options used during the build process. The default options are `-DskipTests`.
#### `MAVEN_CUSTOM_GOALS`
Allows overriding the Maven goals used during the build process. The default goals are `clean install`.
#### `MAVEN_JAVA_OPTS`
Allows overriding the Java options for the Maven process during build. The default Java options are `-Xmx1024m`.
#### `HEROKU_BUILDPACK_DEBUG`
If set, the buildpack will emit debug log messages.

## License
Licensed under the MIT License. See [LICENSE](../../LICENSE) file.
