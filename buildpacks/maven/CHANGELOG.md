# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.1.1] - 2023-06-22

- No changes

## [1.0.5] - 2023-06-13

* This buildpack now declares to be compatible with the `*` stack. While the buildpack cannot guarantee it works with any stack conceivable, it should be compatible with some stacks that are not maintained by Heroku. Use of this buildpack on such stacks is unsupported. ([#498](https://github.com/heroku/buildpacks-jvm/pull/498))
* Allow `JAVA_HOME` to be set by user or operator via `<platform>/env`. ([#508](https://github.com/heroku/buildpacks-jvm/pull/508))
* `MAVEN_SETTINGS_PATH`, `MAVEN_ESTTINGS_URL`, `MAVEN_CUSTOM_GOALS`, and `MAVEN_CUSTOM_OPTS` can be set by a previous buildpack. ([#508](https://github.com/heroku/buildpacks-jvm/pull/508))

## [1.0.4] - 2023-05-11

* Upgrade `libcnb` and `libherokubuildpack` to `0.12.0`. ([#463](https://github.com/heroku/buildpacks-jvm/pull/463))
* The buildpack now implements Buildpack API 0.9 instead of 0.8, and so requires `lifecycle` 0.15.x or newer. ([#463](https://github.com/heroku/buildpacks-jvm/pull/463))

## [1.0.3] - 2022-09-28

* Upgrade `libcnb` and `libherokubuildpack` to `0.11.0`. ([#371](https://github.com/heroku/buildpacks-jvm/pull/371))
* Buildpack now implements buildpack API version `0.8` and so requires lifecycle version `0.14.x` or newer. ([#363](https://github.com/heroku/buildpacks-jvm/pull/363))

## [1.0.2] - 2022-07-29

* Updated `libcnb` and `libherokubuildpack` to `0.9.0`. ([#330](https://github.com/heroku/buildpacks-jvm/pull/330))
* Switch to the recommended regional S3 domain instead of the global one. ([#314](https://github.com/heroku/buildpacks-jvm/pull/314))
* Upgrade `libcnb` to `0.8.0` and `libherokubuildpack` to `0.8.0`.

## [1.0.1] - 2022-06-09

* Add support for the `heroku-22` stack. ([#304](https://github.com/heroku/buildpacks-jvm/pull/304))

## [1.0.0] - 2022-03-24

* Re-implement buildpack using [libcnb.rs](https://github.com/heroku/libcnb.rs) ([#273](https://github.com/heroku/buildpacks-jvm/pull/273))
* Source and Javadoc JAR files are no longer considered when determining the default web process. ([#273](https://github.com/heroku/buildpacks-jvm/pull/273))

## [0.2.6] - 2022-03-02

* Switch to BSD 3-Clause License
* Applications that use Spring Boot are now properly detected even if their dependency to Spring Boot is transitive

## [0.2.5] - 2021-08-10

### Fixed
* Ensures `mvnw` is executable

## [0.2.4] - 2021-07-16

### Added
* Loosen stack requirements allowing any linux distro use this buildpack

## [0.2.3] - 2021-05-05

### Added
* Documentation in `README.md`
* `M2_HOME` environment variable is now set for subsequent buildpacks if Maven was installed.
* `MAVEN_OPTS` environment variable will be set for subsequent buildpacks to allow the use of the local
  repository layer without explicit configuration.

### Fixed
* Fixed `licenses` in `buildpack.toml`

## [0.2.2] - 2021-02-23

## [0.2.1] - 2021-02-03

### Added
* Automated post-release PRs
* Now requires (in the CNB sense) `jdk` to pass detection
* Now provides (in the CNB sense) `jvm-application` to subsequent buildpacks

## [0.2.0] - 2021-01-19

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

## [0.1.0] - 2021-01-15

* Initial release

[unreleased]: https://github.com/heroku/buildpacks-jvm/compare/v1.1.1...HEAD
[1.1.1]: https://github.com/heroku/buildpacks-jvm/releases/tag/v1.1.1