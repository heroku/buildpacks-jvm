# Heroku Cloud Native Leiningen Buildpack
[![CI](https://github.com/heroku/buildpacks-jvm/actions/workflows/ci.yml/badge.svg)](https://github.com/heroku/buildpacks-jvm/actions/workflows/ci.yml)

Heroku's official Cloud Native Buildpack for [Leiningen](https://leiningen.org/) usage in [Clojure](https://clojure.org/) applications.

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
