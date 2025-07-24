# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [7.0.2] - 2025-07-24

### Added

- Platform environment variables are now exposed to Gradle build processes. This allows build-time configuration through environment variables set at the platform level. ([#821](https://github.com/heroku/buildpacks-jvm/pull/821))

## [7.0.1] - 2025-07-16

- No changes.

## [7.0.0] - 2025-06-11

### Changed

- Gradle home layer (referenced by `GRADLE_USER_HOME`) is no longer available in the run image. It was unintentionally included in earlier versions of the buildpack. Applications should not rely on Gradle during runtime. ([#811](https://github.com/heroku/buildpacks-jvm/pull/811))

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

### Added

- The buildpack will add a default process type if a supported framework is detected and the expected build output is found. This mirrors the same feature from the Maven buildpack. ([#726](https://github.com/heroku/buildpacks-jvm/pull/726))
- Support for the Micronaut and Quarkus frameworks. Both previously worked with the buildpack but required some configuration. Unless heavily customized, no build task needs to be specified anymore. ([#726](https://github.com/heroku/buildpacks-jvm/pull/726))

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
[3.1.0]: https://github.com/heroku/buildpacks-jvm/releases/tag/v3.1.0
