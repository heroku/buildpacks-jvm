# Changelog
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

* Add support for the `heroku-22` stack. ([#304](https://github.com/heroku/buildpacks-jvm/pull/304))

## [1.0.0] 2022/03/24

* Re-implement buildpack using [libcnb.rs](https://github.com/Malax/libcnb.rs) ([#273](https://github.com/heroku/buildpacks-jvm/pull/273))
* Source and Javadoc JAR files are no longer considered when determining the default web process. ([#273](https://github.com/heroku/buildpacks-jvm/pull/273))

## [0.2.6] 2022/03/02

* Switch to BSD 3-Clause License
* Applications that use Spring Boot are now properly detected even if their dependency to Spring Boot is transitive

## [0.2.5] 2021/08/10
### Fixed
* Ensures `mvnw` is executable

## [0.2.4] 2021/07/16
### Added
* Loosen stack requirements allowing any linux distro use this buildpack

## [0.2.3] 2021/05/05
### Added
* Documentation in `README.md`
* `M2_HOME` environment variable is now set for subsequent buildpacks if Maven was installed.
* `MAVEN_OPTS` environment variable will be set for subsequent buildpacks to allow the use of the local
  repository layer without explicit configuration.

### Fixed
* Fixed `licenses` in `buildpack.toml`

## [0.2.2] 2021/02/23

## [0.2.1] 2021/02/03
### Added
* Automated post-release PRs
* Now requires (in the CNB sense) `jdk` to pass detection
* Now provides (in the CNB sense) `jvm-application` to subsequent buildpacks

## [0.2.0]
### Added
* Debug logging, can be enabled by setting `HEROKU_BUILDPACK_DEBUG` environment variable

### Changed
* Code refactoring
* Logging style now adheres to Heroku's CNB logging style
* Maven options that are implementation details are no longer logged by default
* Maven options that are required for proper operation of this buildpack can no longer be overridden by
  `MAVEN_CUSTOM_OPTS` or `MAVEN_CUSTOM_GOALS`

### Fixed
* Caching of Maven dependencies
* Exit code of `bin/detect` when detection failed without an error

## [0.1.0]
* Initial release
