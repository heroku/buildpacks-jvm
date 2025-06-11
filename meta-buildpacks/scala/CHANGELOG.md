# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [7.0.0] - 2025-06-11

### Changed

- Updated `heroku/jvm` to `7.0.0`.
- Updated `heroku/sbt` to `7.0.0`.

## [6.2.1] - 2025-04-28

### Changed

- Updated `heroku/jvm` to `6.2.1`.
- Updated `heroku/sbt` to `6.2.1`.

## [6.2.0] - 2025-04-22

### Changed

- Updated `heroku/jvm` to `6.2.0`.
- Updated `heroku/sbt` to `6.2.0`.

## [6.1.2] - 2025-04-03

### Changed

- Updated `heroku/jvm` to `6.1.2`.
- Updated `heroku/sbt` to `6.1.2`.

## [6.1.1] - 2025-03-19

### Changed

- Updated `heroku/jvm` to `6.1.1`.
- Updated `heroku/sbt` to `6.1.1`.

## [6.1.0] - 2025-02-28

### Changed

- Updated `heroku/jvm` to `6.1.0`.
- Updated `heroku/sbt` to `6.1.0`.

## [6.0.4] - 2024-12-05

### Changed

- Updated `heroku/jvm` to `6.0.4`.
- Updated `heroku/sbt` to `6.0.4`.

## [6.0.3] - 2024-09-26

### Changed

- Updated `heroku/jvm` to `6.0.3`.
- Updated `heroku/sbt` to `6.0.3`.

## [6.0.2] - 2024-09-25

### Changed

- Updated `heroku/jvm` to `6.0.2`.
- Updated `heroku/sbt` to `6.0.2`.

## [6.0.1] - 2024-07-19

### Changed

- Updated `heroku/jvm` to `6.0.1`.
- Updated `heroku/sbt` to `6.0.1`.

## [6.0.0] - 2024-05-28

### Changed

- Updated `heroku/jvm` to `6.0.0`.
- Updated `heroku/sbt` to `6.0.0`.

## [5.0.1] - 2024-05-23

### Changed

- Updated `heroku/jvm` to `5.0.1`.
- Updated `heroku/sbt` to `5.0.1`.

## [5.0.0] - 2024-05-23

### Changed

- Buildpack API version changed from `0.9` to `0.10`, and so requires `lifecycle` `0.17.x` or newer. ([#662](https://github.com/heroku/buildpacks-jvm/pull/662))
- Updated `heroku/jvm` to `5.0.0`.
- Updated `heroku/sbt` to `5.0.0`.

## [4.1.1] - 2024-05-01

### Changed

- Updated `heroku/jvm` to `4.1.1`.
- Updated `heroku/sbt` to `4.1.1`.

## [4.1.0] - 2024-01-23

### Changed

- Updated `heroku/jvm` to `4.1.0`.
- Updated `heroku/sbt` to `4.1.0`.

## [4.0.2] - 2023-12-05

### Changed

- Updated `heroku/jvm` to `4.0.2`.
- Updated `heroku/sbt` to `4.0.2`.

## [4.0.1] - 2023-12-04

### Changed

- Updated `heroku/jvm` to `4.0.1`.
- Updated `heroku/sbt` to `4.0.1`.

## [4.0.0] - 2023-10-25

### Removed

- Removed `heroku/procfile`, since it's being added directly to the Heroku builder images instead. If you override the Heroku builder images' default buildpack detection order (or use this buildpack with a non-Heroku builder image), you will need to append `heroku/procfile` to your buildpacks list. ([#608](https://github.com/heroku/buildpacks-jvm/pull/608))

### Changed

- Updated `heroku/jvm` to `4.0.0`.
- Updated `heroku/sbt` to `4.0.0`.

## [3.2.2] - 2023-10-24

### Changed

- Updated buildpack display name, description and keywords. ([#603](https://github.com/heroku/buildpacks-jvm/pull/603))
- Updated `heroku/jvm` to `3.2.2`.
- Updated `heroku/sbt` to `3.2.2`.

## [3.2.1] - 2023-10-19

### Changed

- Updated `heroku/jvm` to `3.2.1`.
- Updated `heroku/sbt` to `3.2.1`.

## [3.2.0] - 2023-09-20

### Changed

- Updated `heroku/jvm` to `3.2.0`.
- Updated `heroku/sbt` to `3.2.0`.

## [3.1.0] - 2023-09-18

### Changed

- Updated `heroku/procfile` to `2.0.1`. ([#568](https://github.com/heroku/buildpacks-jvm/pull/568))
- Updated `heroku/jvm` to `3.1.0`.
- Updated `heroku/sbt` to `3.1.0`.

## [3.0.0] - 2023-08-09

### Changed

- Updated `heroku/jvm` to `3.0.0`.
- Updated `heroku/sbt` to `3.0.0`.

## [2.0.0] - 2023-07-31

### Changed

- Updated `heroku/jvm` to `2.0.0`.
- Updated `heroku/sbt` to `2.0.0`.

## [1.1.2] - 2023-07-24

### Changed

- Updated `heroku/jvm` to `1.1.2`.
- Updated `heroku/sbt` to `1.1.2`.

## [1.1.1] - 2023-06-22

### Changed

- Updated `heroku/jvm` to `1.1.1`
- Updated `heroku/sbt` to `1.1.1`

## [1.0.0] - 2023-05-11

### Added

- Initial release

[unreleased]: https://github.com/heroku/buildpacks-jvm/compare/v7.0.0...HEAD
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
