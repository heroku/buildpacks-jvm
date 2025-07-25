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

### Removed

- Support for sbt `<1.0`. This buildpack supported old and deprecated sbt versions on a best-effort basis before. Artifacts required by those older versions recently started to be unavailable upstream which caused us to drop support for those versions. If you're affected, please migrate your project to the latest stable sbt version `1.10.0`. ([#669](https://github.com/heroku/buildpacks-jvm/pull/669))

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

- No changes.

## [2.0.0] - 2023-07-31

- No changes.

## [1.1.2] - 2023-07-24

- No changes.

## [1.1.1] - 2023-06-22

### Changed

- This buildpack now declares to be compatible with the `*` stack. While the buildpack cannot guarantee it works with any stack conceivable, it should be compatible with some stacks that are not maintained by Heroku. Use of this buildpack on such stacks is unsupported. ([#498](https://github.com/heroku/buildpacks-jvm/pull/498))

## [1.0.0] - 2023-05-11

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
