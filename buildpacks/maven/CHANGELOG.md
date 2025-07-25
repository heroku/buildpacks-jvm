# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [7.0.2] - 2025-07-24

- No changes.

## [7.0.1] - 2025-07-16

- No changes.

## [7.0.0] - 2025-06-11

- No changes.

## [6.2.1] - 2025-04-28

- No changes.

## [6.2.0] - 2025-04-22

### Changed

- Buildpack output changed to a new format. ([#745](https://github.com/heroku/buildpacks-jvm/pull/745))

## [6.1.2] - 2025-04-03

### Changed

- Updated libcnb to 0.28.1, which includes tracing improvements/fixes. ([#794](https://github.com/heroku/buildpacks-jvm/pull/794))

## [6.1.1] - 2025-03-19

- No changes.

## [6.1.0] - 2025-02-28

### Changed

- Enabled `libcnb`'s `trace` feature. ([#779](https://github.com/heroku/buildpacks-jvm/pull/779))

## [6.0.4] - 2024-12-05

- No changes.

## [6.0.3] - 2024-09-26

- No changes.

## [6.0.2] - 2024-09-25

- No changes.

## [6.0.1] - 2024-07-19

- No changes.

## [6.0.0] - 2024-05-28

### Changed

- Some error messages have changed so they longer suggest to open a Heroku support ticket. Instead, users are now provided with a link to create an issue on GitHub. ([#674](https://github.com/heroku/buildpacks-jvm/pull/674))

## [5.0.1] - 2024-05-23

- No changes.

## [5.0.0] - 2024-05-23

### Added

- Support for the `arm64` architecture. ([#668](https://github.com/heroku/buildpacks-jvm/pull/668))

### Changed

- Buildpack API version changed from `0.9` to `0.10`, and so requires `lifecycle` `0.17.x` or newer. ([#662](https://github.com/heroku/buildpacks-jvm/pull/662))

## [4.1.1] - 2024-05-01

- No changes.

## [4.1.0] - 2024-01-23

- No changes.

## [4.0.2] - 2023-12-05

- No changes.

## [4.0.1] - 2023-12-04

- No changes.

## [4.0.0] - 2023-10-25

- No changes.

## [3.2.2] - 2023-10-24

### Changed

- Updated buildpack display name, description and keywords. ([#603](https://github.com/heroku/buildpacks-jvm/pull/603))

## [3.2.1] - 2023-10-19

- No changes.

## [3.2.0] - 2023-09-20

- No changes.

## [3.1.0] - 2023-09-18

- No changes.

## [3.0.0] - 2023-08-09

### Removed

- Remove support for installing Maven `3.2.5`, `3.3.9`, `3.5.4` and `3.6.2` via `system.properties`. These versions of Maven contain security vulnerabilities and should not be used. Users that cannot upgrade to a secure version of Maven can install the Maven Wrapper for the required Maven version to their application (i.e. `mvn wrapper:wrapper -Dmaven=3.6.2`). ([#556](https://github.com/heroku/buildpacks-jvm/pull/556))
 
### Changed

- Default version for Maven is now `3.9.4`. ([#556](https://github.com/heroku/buildpacks-jvm/pull/556))

## [2.0.0] - 2023-07-31

- No changes.

## [1.1.2] - 2023-07-24

- No changes.

## [1.1.1] - 2023-06-22

- No changes

## [1.0.5] - 2023-06-13

### Changed

- This buildpack now declares to be compatible with the `*` stack. While the buildpack cannot guarantee it works with any stack conceivable, it should be compatible with some stacks that are not maintained by Heroku. Use of this buildpack on such stacks is unsupported. ([#498](https://github.com/heroku/buildpacks-jvm/pull/498))
- Allow `JAVA_HOME` to be set by user or operator via `<platform>/env`. ([#508](https://github.com/heroku/buildpacks-jvm/pull/508))
- `MAVEN_SETTINGS_PATH`, `MAVEN_ESTTINGS_URL`, `MAVEN_CUSTOM_GOALS`, and `MAVEN_CUSTOM_OPTS` can be set by a previous buildpack. ([#508](https://github.com/heroku/buildpacks-jvm/pull/508))

## [1.0.4] - 2023-05-11

### Changed

- Upgrade `libcnb` and `libherokubuildpack` to `0.12.0`. ([#463](https://github.com/heroku/buildpacks-jvm/pull/463))
- The buildpack now implements Buildpack API 0.9 instead of 0.8, and so requires `lifecycle` 0.15.x or newer. ([#463](https://github.com/heroku/buildpacks-jvm/pull/463))

## [1.0.3] - 2022-09-28

### Changed

- Upgrade `libcnb` and `libherokubuildpack` to `0.11.0`. ([#371](https://github.com/heroku/buildpacks-jvm/pull/371))
- Buildpack now implements buildpack API version `0.8` and so requires lifecycle version `0.14.x` or newer. ([#363](https://github.com/heroku/buildpacks-jvm/pull/363))

## [1.0.2] - 2022-07-29

### Changed

- Updated `libcnb` and `libherokubuildpack` to `0.9.0`. ([#330](https://github.com/heroku/buildpacks-jvm/pull/330))
- Switch to the recommended regional S3 domain instead of the global one. ([#314](https://github.com/heroku/buildpacks-jvm/pull/314))
- Upgrade `libcnb` to `0.8.0` and `libherokubuildpack` to `0.8.0`.

## [1.0.1] - 2022-06-09

### Added

- Add support for the `heroku-22` stack. ([#304](https://github.com/heroku/buildpacks-jvm/pull/304))

## [1.0.0] - 2022-03-24

### Changed

- Re-implement buildpack using [libcnb.rs](https://github.com/heroku/libcnb.rs) ([#273](https://github.com/heroku/buildpacks-jvm/pull/273))
- Source and Javadoc JAR files are no longer considered when determining the default web process. ([#273](https://github.com/heroku/buildpacks-jvm/pull/273))

## [0.2.6] - 2022-03-02

### Changed

- Switch to BSD 3-Clause License
- Applications that use Spring Boot are now properly detected even if their dependency to Spring Boot is transitive

## [0.2.5] - 2021-08-10

### Fixed

- Ensures `mvnw` is executable

## [0.2.4] - 2021-07-16

### Added

- Loosen stack requirements allowing any linux distro use this buildpack

## [0.2.3] - 2021-05-05

### Added

- Documentation in `README.md`
- `M2_HOME` environment variable is now set for subsequent buildpacks if Maven was installed.
- `MAVEN_OPTS` environment variable will be set for subsequent buildpacks to allow the use of the local
  repository layer without explicit configuration.

### Fixed

- Fixed `licenses` in `buildpack.toml`

## [0.2.2] - 2021-02-23

- No changes.

## [0.2.1] - 2021-02-03

### Added

- Automated post-release PRs
- Now requires (in the CNB sense) `jdk` to pass detection
- Now provides (in the CNB sense) `jvm-application` to subsequent buildpacks

## [0.2.0] - 2021-01-19

### Added

- Debug logging, can be enabled by setting `HEROKU_BUILDPACK_DEBUG` environment variable

### Changed

- Code refactoring
- Logging style now adheres to Heroku's CNB logging style
- Maven options that are implementation details are no longer logged by default
- Maven options that are required for proper operation of this buildpack can no longer be overridden by
  `MAVEN_CUSTOM_OPTS` or `MAVEN_CUSTOM_GOALS`

### Fixed

- Caching of Maven dependencies
- Exit code of `bin/detect` when detection failed without an error

## [0.1.1] - 2021-01-15

### Added

- Initial release

[unreleased]: https://github.com/heroku/buildpacks-jvm/compare/v7.0.2...HEAD
[7.0.2]: https://github.com/heroku/buildpacks-jvm/compare/v7.0.1...v7.0.2
[7.0.1]: https://github.com/heroku/buildpacks-jvm/compare/v7.0.0...v7.0.1
[7.0.0]: https://github.com/heroku/buildpacks-jvm/compare/v6.2.1...v7.0.0
[6.2.1]: https://github.com/heroku/buildpacks-jvm/compare/v6.2.0...v6.2.1
[6.2.0]: https://github.com/heroku/buildpacks-jvm/compare/v6.1.2...v6.2.0
[6.1.2]: https://github.com/heroku/buildpacks-jvm/compare/v6.1.1...v6.1.2
[6.1.1]: https://github.com/heroku/buildpacks-jvm/compare/v6.1.0...v6.1.1
[6.1.0]: https://github.com/heroku/buildpacks-jvm/compare/v6.0.4...v6.1.0
[6.0.4]: https://github.com/heroku/buildpacks-jvm/compare/v6.0.3...v6.0.4
[6.0.3]: https://github.com/heroku/buildpacks-jvm/compare/v6.0.2...v6.0.3
[6.0.2]: https://github.com/heroku/buildpacks-jvm/compare/v6.0.1...v6.0.2
[6.0.1]: https://github.com/heroku/buildpacks-jvm/compare/v6.0.0...v6.0.1
[6.0.0]: https://github.com/heroku/buildpacks-jvm/compare/v5.0.1...v6.0.0
[5.0.1]: https://github.com/heroku/buildpacks-jvm/compare/v5.0.0...v5.0.1
[5.0.0]: https://github.com/heroku/buildpacks-jvm/compare/v4.1.1...v5.0.0
[4.1.1]: https://github.com/heroku/buildpacks-jvm/compare/v4.1.0...v4.1.1
[4.1.0]: https://github.com/heroku/buildpacks-jvm/compare/v4.0.2...v4.1.0
[4.0.2]: https://github.com/heroku/buildpacks-jvm/compare/v4.0.1...v4.0.2
[4.0.1]: https://github.com/heroku/buildpacks-jvm/compare/v4.0.0...v4.0.1
[4.0.0]: https://github.com/heroku/buildpacks-jvm/compare/v3.2.2...v4.0.0
[3.2.2]: https://github.com/heroku/buildpacks-jvm/compare/v3.2.1...v3.2.2
[3.2.1]: https://github.com/heroku/buildpacks-jvm/compare/v3.2.0...v3.2.1
[3.2.0]: https://github.com/heroku/buildpacks-jvm/compare/v3.1.0...v3.2.0
[3.1.0]: https://github.com/heroku/buildpacks-jvm/compare/v3.0.0...v3.1.0
[3.0.0]: https://github.com/heroku/buildpacks-jvm/compare/v2.0.0...v3.0.0
[2.0.0]: https://github.com/heroku/buildpacks-jvm/compare/v1.1.2...v2.0.0
[1.1.2]: https://github.com/heroku/buildpacks-jvm/compare/v1.1.1...v1.1.2
[1.1.1]: https://github.com/heroku/buildpacks-jvm/releases/tag/v1.1.1
