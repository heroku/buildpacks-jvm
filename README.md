# Heroku Cloud Native Buildpacks: JVM

[![Cloud Native Buildpacks Registry: heroku/jvm][registry-badge]][registry-url]
[![CI on Github Actions: heroku/jvm][ci-badge]][ci-url]

![Heroku Cloud Native Buildpack: heroku/jvm][cnb-banner]

This repository is the home of [Heroku Cloud Native Buildpacks][heroku-buildpacks]
for JVM applications. These buildpacks build Java, Scala and other JVM language application source code
into application images with minimal configuration.

> [!IMPORTANT]
> This is a collection of [Cloud Native Buildpacks][cnb], and is a component of the [Heroku Cloud Native Buildpacks][heroku-buildpacks] project, which is in preview. If you are instead looking for the Heroku Classic Buildpacks for JVM applications (for use on the Heroku platform), you may find them here: [heroku/jvm](https://github.com/heroku/heroku-buildpack-jvm-common), [heroku/java](https://github.com/heroku/heroku-buildpack-java), [heroku/gradle](https://github.com/heroku/heroku-buildpack-gradle), [heroku/scala](https://github.com/heroku/heroku-buildpack-scala), [heroku/clojure](https://github.com/heroku/heroku-buildpack-clojure).

## Usage

> [!NOTE]
> Before getting started, ensure you have the `pack` CLI installed. Installation instructions are available [here][pack-install].

To build a JVM application codebase into a production image:

```bash
$ cd ~/workdir/sample-jvm-app
$ pack build sample-app --builder heroku/builder:24
```

Then run the image:

```bash
docker run --rm -it -e "PORT=8080" -p 8080:8080 sample-app
```

## Application Requirements

Either a `pom.xml` file (or one of the other POM formats supported by the
[Maven Polyglot plugin](https://github.com/takari/polyglot-maven)) or a `build.sbt` in the root
directory is required for these buildpacks to build your app.

## Configuration

### Procfile

A Procfile is a text file in the root directory of your application that defines process types and explicitly declares
what command should be executed to start your app. Your `Procfile` will look something like this for Spring Boot:

```
web: java -Dserver.port=$PORT $JAVA_OPTS -jar target/demo-0.0.1-SNAPSHOT.jar
```

### OpenJDK

#### OpenJDK Version

By default, the latest OpenJDK long-term support (LTS) release will be installed. You can configure the OpenJDK version your application
needs. The buildpack tries to determine the required version in the following order:

##### `system.properties`

You can specify a Java version by adding a [Java properties file](https://en.wikipedia.org/wiki/.properties) called
`system.properties` to the root directory of your application. The value of the `java.runtime.version` key specifies
the required OpenJDK version:

```
java.runtime.version=21
```

Supported major versions are `8`, `11`, `17`, `21` and `25`. The buildpack will always install the latest release
of the requested major version.

## Included Buildpacks

### Languages

Language buildpacks are meta-buildpacks that aggregate other buildpacks (see below) for convenient use. Use these if you want
to build your application.

| ID             | Name  | Readme                                    | Changelog                                       |
|----------------|-------|-------------------------------------------|-------------------------------------------------|
| `heroku/java`  | Java  | [Readme](meta-buildpacks/java/README.md)  | [Changelog](meta-buildpacks/java/CHANGELOG.md)  |
| `heroku/scala` | Scala | [Readme](meta-buildpacks/scala/README.md) | [Changelog](meta-buildpacks/scala/CHANGELOG.md) |

### Build Tools

| ID              | Name                               | Readme                                | Changelog                                   |
|-----------------|------------------------------------|---------------------------------------|---------------------------------------------|
| `heroku/maven`  | [Maven](https://maven.apache.org/) | [Readme](buildpacks/maven/README.md)  | [Changelog](buildpacks/maven/CHANGELOG.md)  |
| `heroku/gradle` | [Gradle](https://gradle.org/)      | [Readme](buildpacks/gradle/README.md) | [Changelog](buildpacks/gradle/CHANGELOG.md) |
| `heroku/sbt`    | [sbt](https://www.scala-sbt.org/)  | [Readme](buildpacks/sbt/README.md)    | [Changelog](buildpacks/sbt/CHANGELOG.md)    |

### Platforms

| ID           | Name                                 | Readme                             | Changelog                                |
|--------------|--------------------------------------|------------------------------------|------------------------------------------|
| `heroku/jvm` | [OpenJDK](https://openjdk.java.net/) | [Readme](buildpacks/jvm/README.md) | [Changelog](buildpacks/jvm/CHANGELOG.md) |

## Contributing

Issues and pull requests are welcome. See our [contributing guidelines](./CONTRIBUTING.md) if you would like to help.


[ci-badge]: https://github.com/heroku/buildpacks-jvm/actions/workflows/ci.yml/badge.svg

[ci-url]: https://github.com/heroku/buildpacks-jvm/actions/workflows/ci.yml

[cnb]: https://buildpacks.io

[cnb-banner]: https://cloud.githubusercontent.com/assets/871315/20325947/f3544014-ab43-11e6-9c51-8240ce161939.png

[classic-buildpack]: https://github.com/heroku/heroku-buildpack-jvm

[heroku-buildpacks]: https://github.com/heroku/buildpacks

[pack-install]: https://buildpacks.io/docs/for-platform-operators/how-to/integrate-ci/pack/

[registry-badge]: https://img.shields.io/badge/dynamic/json?url=https://registry.buildpacks.io/api/v1/buildpacks/heroku/jvm&label=version&query=$.latest.version&color=DF0A6B&logo=data:image/png;base64,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&labelColor=white

[registry-url]: https://registry.buildpacks.io/buildpacks/heroku/jvm
