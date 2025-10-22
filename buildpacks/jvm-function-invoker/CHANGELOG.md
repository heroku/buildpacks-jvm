# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [7.0.6] - 2025-10-22

- No changes.

## [7.0.5] - 2025-09-29

- No changes.

## [7.0.4] - 2025-09-19

- No changes.

## [7.0.3] - 2025-09-17

- No changes.

## [7.0.2] - 2025-07-24

- No changes.

## [7.0.1] - 2025-07-16

- No changes.

## [7.0.0] - 2025-06-11

- No changes.

## [6.2.1] - 2025-04-28

- No changes.

## [6.2.0] - 2025-04-22

- No changes.

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

- No changes.

## [5.0.1] - 2024-05-23

- No changes.

## [5.0.0] - 2024-05-23

### Changed

- Updated function runtime to 1.1.7. ([#664](https://github.com/heroku/buildpacks-jvm/pull/664))
- Buildpack API version changed from `0.9` to `0.10`, and so requires `lifecycle` `0.17.x` or newer. ([#662](https://github.com/heroku/buildpacks-jvm/pull/662))

## [4.1.1] - 2024-05-01

- No changes.

## [4.1.0] - 2024-01-23

- No changes.

## [4.0.2] - 2023-12-05

### Changed

- Updated function runtime to 1.1.6 ([#622](https://github.com/heroku/buildpacks-jvm/pull/622))

## [4.0.1] - 2023-12-04

### Changed

- Updated function runtime to 1.1.5 ([#619](https://github.com/heroku/buildpacks-jvm/pull/619))

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

- No changes.

## [2.0.0] - 2023-07-31

- No changes.

## [1.1.2] - 2023-07-24

- No changes.

## [1.1.1] - 2023-06-22

### Changed

- This buildpack now declares to be compatible with the `*` stack. While the buildpack cannot guarantee it works with any stack conceivable, it should be compatible with some stacks that are not maintained by Heroku. Use of this buildpack on such stacks is unsupported. ([#498](https://github.com/heroku/buildpacks-jvm/pull/498))

## [0.6.8] - 2023-05-11

### Changed

- Upgrade `libcnb` and `libherokubuildpack` to `0.12.0`. ([#463](https://github.com/heroku/buildpacks-jvm/pull/463))
- The buildpack now implements Buildpack API 0.9 instead of 0.8, and so requires `lifecycle` 0.15.x or newer. ([#463](https://github.com/heroku/buildpacks-jvm/pull/463))

## [0.6.7] - 2023-01-19

### Changed

- Update `sf-fx-runtime-java` from `1.1.2` to `1.1.3`.

## [0.6.6] - 2022-11-30

### Changed

- Update `sf-fx-runtime-java` from `1.1.1` to `1.1.2`. ([#398](https://github.com/heroku/buildpacks-jvm/pull/398))

## [0.6.5] - 2022-10-20

### Changed

- Updated function runtime to `1.1.1`. ([#388](https://github.com/heroku/buildpacks-jvm/pull/388))
- Upgrade `libcnb` and `libherokubuildpack` to `0.11.1`. ([#384](https://github.com/heroku/buildpacks-jvm/pull/384) and [#386](https://github.com/heroku/buildpacks-jvm/pull/386))

## [0.6.4] - 2022-09-28

### Changed

- Upgrade `libcnb` and `libherokubuildpack` to `0.11.0`. ([#371](https://github.com/heroku/buildpacks-jvm/pull/371))
- Buildpack now implements buildpack API version `0.8` and so requires lifecycle version `0.14.x` or newer. ([#363](https://github.com/heroku/buildpacks-jvm/pull/363))
- Updated function runtime to `1.1.0`

## [0.6.3] - 2022-06-29

### Changed

- Upgrade `libcnb` to `0.8.0` and `libherokubuildpack` to `0.8.0`.
- Updated function runtime to `1.0.7`

## [0.6.2] - 2022-06-09

### Changed

- Upgrade `libcnb` to `0.6.0` and `libherokubuildpack` to `0.6.0`.
- Add support for the `heroku-22` stack. ([#304](https://github.com/heroku/buildpacks-jvm/pull/304))

## [0.6.1] - 2022-02-08

### Changed

- Upgrade `libcnb` to `0.5.0` and `libherokubuildpack` to `0.5.0`.
- Updated function runtime to `1.0.6`

## [0.6.0] - 2022-01-05

### Changed

- Switch to BSD 3-Clause License
- Upgrade to `libcnb` version `0.4.0`
- Updated function runtime to `1.0.5`

## [0.5.5] - 2021-10-19

- No changes.

## [0.5.4] - 2021-09-30

### Changed

- Updated function runtime to `1.0.3`

## [0.5.3] - 2021-09-29

### Changed

- Updated function runtime to `1.0.2`
- Update buildpack API version from `0.4` to `0.5`
- Update `libcnb` and `libherokubuildpack` to new major versions

## [0.5.2] - 2021-08-31

### Changed

- Rewrite to use libcnb 0.2.0 features

## [0.5.1] - 2021-08-25

### Changed

- Updated function runtime to `1.0.1`

## [0.5.0] - 2021-08-10

### Changed

- Changed implementation to Rust (relanded with upgrade to libcnb `0.1.3`)

## [0.4.0] - 2021-07-28

### Changed

- Revert all changes from `0.3.0`
- Updated function runtime to `1.0.0` again

## [0.3.0] - 2021-07-15

### Changed

- Changed implementation to Rust
- Updated function runtime to `1.0.0`

## [0.2.11] - 2021-05-21

### Changed

- Updated function runtime to `0.2.4`

## [0.2.10] - 2021-05-18

### Changed

- Updated function runtime to `0.2.3`

## [0.2.9] - 2021-05-17

### Changed

- Updated function runtime to `0.2.2`

## [0.2.8] - 2021-05-17

### Changed

- `SF_FX_REMOTE_DEBUG` was renamed to `DEBUG_PORT` and also species the port on with the JDWP agent will listen on.
- Updated function runtime to `0.2.1`
- Update `bin/detect` to check for `type=function`.

## [0.2.7] - 2021-05-05

### Changed

- Updated function runtime to `0.2.0`

### Added

- Support for the `SF_FX_REMOTE_DEBUG` runtime environment variable. If set, the invoker will listen for incoming JDWP
  connections on port `5005`.

### Changed

- Detection now checks for `project.toml` in addition to `function.toml` to determine if an app is a function.

## [0.2.6] - 2021-04-29

### Changed

- Updated function runtime to `0.1.4-ea`

## [0.2.5] - 2021-04-21

### Changed

- Updated function runtime to `0.1.3-ea`

## [0.2.4] - 2021-04-08

### Fixed

- Fixed `licenses` in `buildpack.toml`
- Updated function runtime to `0.1.1-ea`

## [0.2.3] - 2021-02-23

- No changes.

## [0.2.2] - 2021-02-04

### Added

- Support for the `PORT` environment variable at runtime for setting the HTTP port

### Fixed

- When using an older version of `pack`, the function layer might be incorrectly restored, causing errors
  "directory not empty" during function detection. A workaround has been added.

## [0.2.1] - 2021-02-03

### Changed

- Now requires (in the CNB sense) `jvm-application` to pass detection.
- Will now fail detection if there is no `function.toml` present.

### Removed

- The Java function runtime binary integrity is now checked after download (temporarily removed).
- Java function runtime is now cached between builds (temporarily removed).

## [0.2.0] - 2021-02-01

### Changed

- Function runtime binary URL no longer has to be specified with the `JVM_INVOKER_JAR_URL` environment variable.
- Functions are now detected during build. This means the build will now fail if more or less than one valid
  Salesforce Java function is detected in the project.

### Added

- The Java function runtime binary integrity is now checked after download.
- Java function runtime is now cached between builds.

## [0.1.0] - 2021-01-21

### Added

- Initial release.

[unreleased]: https://github.com/heroku/buildpacks-jvm/compare/v7.0.6...HEAD
[7.0.6]: https://github.com/heroku/buildpacks-jvm/compare/v7.0.5...v7.0.6
[7.0.5]: https://github.com/heroku/buildpacks-jvm/compare/v7.0.4...v7.0.5
[7.0.4]: https://github.com/heroku/buildpacks-jvm/compare/v7.0.3...v7.0.4
[7.0.3]: https://github.com/heroku/buildpacks-jvm/compare/v7.0.2...v7.0.3
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
