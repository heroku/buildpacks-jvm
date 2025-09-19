# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [7.0.4] - 2025-09-19

### Changed

- Updated `heroku/gradle` to `7.0.4`.
- Updated `heroku/jvm` to `7.0.4`.
- Updated `heroku/maven` to `7.0.4`.

## [7.0.3] - 2025-09-17

### Changed

- Updated `heroku/gradle` to `7.0.3`.
- Updated `heroku/jvm` to `7.0.3`.
- Updated `heroku/maven` to `7.0.3`.

## [7.0.2] - 2025-07-24

### Changed

- Updated `heroku/gradle` to `7.0.2`.
- Updated `heroku/jvm` to `7.0.2`.
- Updated `heroku/maven` to `7.0.2`.

## [7.0.1] - 2025-07-16

### Changed

- Updated `heroku/gradle` to `7.0.1`.
- Updated `heroku/jvm` to `7.0.1`.
- Updated `heroku/maven` to `7.0.1`.

## [7.0.0] - 2025-06-11

### Changed

- Updated `heroku/gradle` to `7.0.0`.
- Updated `heroku/jvm` to `7.0.0`.
- Updated `heroku/maven` to `7.0.0`.

## [6.2.1] - 2025-04-28

### Changed

- Updated `heroku/gradle` to `6.2.1`.
- Updated `heroku/jvm` to `6.2.1`.
- Updated `heroku/maven` to `6.2.1`.

## [6.2.0] - 2025-04-22

### Changed

- Updated `heroku/gradle` to `6.2.0`.
- Updated `heroku/jvm` to `6.2.0`.
- Updated `heroku/maven` to `6.2.0`.

## [6.1.2] - 2025-04-03

### Changed

- Updated `heroku/gradle` to `6.1.2`.
- Updated `heroku/jvm` to `6.1.2`.
- Updated `heroku/maven` to `6.1.2`.

## [6.1.1] - 2025-03-19

### Changed

- Updated `heroku/gradle` to `6.1.1`.
- Updated `heroku/jvm` to `6.1.1`.
- Updated `heroku/maven` to `6.1.1`.

## [6.1.0] - 2025-02-28

### Changed

- Updated `heroku/gradle` to `6.1.0`.
- Updated `heroku/jvm` to `6.1.0`.
- Updated `heroku/maven` to `6.1.0`.

## [6.0.4] - 2024-12-05

### Changed

- Updated `heroku/gradle` to `6.0.4`.
- Updated `heroku/jvm` to `6.0.4`.
- Updated `heroku/maven` to `6.0.4`.

## [6.0.3] - 2024-09-26

### Changed

- Updated `heroku/gradle` to `6.0.3`.
- Updated `heroku/jvm` to `6.0.3`.
- Updated `heroku/maven` to `6.0.3`.

## [6.0.2] - 2024-09-25

### Changed

- Updated `heroku/gradle` to `6.0.2`.
- Updated `heroku/jvm` to `6.0.2`.
- Updated `heroku/maven` to `6.0.2`.

## [6.0.1] - 2024-07-19

### Changed

- Updated `heroku/gradle` to `6.0.1`.
- Updated `heroku/jvm` to `6.0.1`.
- Updated `heroku/maven` to `6.0.1`.

## [6.0.0] - 2024-05-28

### Changed

- Updated `heroku/gradle` to `6.0.0`.
- Updated `heroku/jvm` to `6.0.0`.
- Updated `heroku/maven` to `6.0.0`.

## [5.0.1] - 2024-05-23

### Changed

- Updated `heroku/gradle` to `5.0.1`.
- Updated `heroku/jvm` to `5.0.1`.
- Updated `heroku/maven` to `5.0.1`.

## [5.0.0] - 2024-05-23

### Changed

- Buildpack API version changed from `0.9` to `0.10`, and so requires `lifecycle` `0.17.x` or newer. ([#662](https://github.com/heroku/buildpacks-jvm/pull/662))
- Updated `heroku/gradle` to `5.0.0`.
- Updated `heroku/jvm` to `5.0.0`.
- Updated `heroku/maven` to `5.0.0`.

## [4.1.1] - 2024-05-01

### Changed

- Updated `heroku/gradle` to `4.1.1`.
- Updated `heroku/jvm` to `4.1.1`.
- Updated `heroku/maven` to `4.1.1`.

## [4.1.0] - 2024-01-23

### Changed

- Updated `heroku/gradle` to `4.1.0`.
- Updated `heroku/jvm` to `4.1.0`.
- Updated `heroku/maven` to `4.1.0`.

## [4.0.2] - 2023-12-05

### Changed

- Updated `heroku/gradle` to `4.0.2`.
- Updated `heroku/jvm` to `4.0.2`.
- Updated `heroku/maven` to `4.0.2`.

## [4.0.1] - 2023-12-04

### Changed

- Updated `heroku/gradle` to `4.0.1`.
- Updated `heroku/jvm` to `4.0.1`.
- Updated `heroku/maven` to `4.0.1`.

## [4.0.0] - 2023-10-25

### Removed

- Removed `heroku/procfile`, since it's being added directly to the Heroku builder images instead. If you override the Heroku builder images' default buildpack detection order (or use this buildpack with a non-Heroku builder image), you will need to append `heroku/procfile` to your buildpacks list. ([#608](https://github.com/heroku/buildpacks-jvm/pull/608))

### Changed

- Updated `heroku/gradle` to `4.0.0`.
- Updated `heroku/jvm` to `4.0.0`.
- Updated `heroku/maven` to `4.0.0`.

## [3.2.2] - 2023-10-24

### Changed

- Updated buildpack display name, description and keywords. ([#603](https://github.com/heroku/buildpacks-jvm/pull/603))
- Updated `heroku/gradle` to `3.2.2`.
- Updated `heroku/jvm` to `3.2.2`.
- Updated `heroku/maven` to `3.2.2`.

## [3.2.1] - 2023-10-19

### Changed

- Updated `heroku/gradle` to `3.2.1`.
- Updated `heroku/jvm` to `3.2.1`.
- Updated `heroku/maven` to `3.2.1`.

## [3.2.0] - 2023-09-20

### Changed

- Updated `heroku/gradle` to `3.2.0`.
- Updated `heroku/jvm` to `3.2.0`.
- Updated `heroku/maven` to `3.2.0`.

## [3.1.0] - 2023-09-18

### Changed

- Updated `heroku/procfile` to `2.0.1`. ([#568](https://github.com/heroku/buildpacks-jvm/pull/568))
- Updated `heroku/gradle` to `3.1.0`.
- Updated `heroku/jvm` to `3.1.0`.
- Updated `heroku/maven` to `3.1.0`.

## [3.0.0] - 2023-08-09

### Changed

- Updated `heroku/jvm` to `3.0.0`.
- Updated `heroku/maven` to `3.0.0`.

## [2.0.0] - 2023-07-31

### Changed

- Updated `heroku/jvm` to `2.0.0`.
- Updated `heroku/maven` to `2.0.0`.

## [1.1.2] - 2023-07-24

### Changed

- Updated `heroku/jvm` to `1.1.2`.
- Updated `heroku/maven` to `1.1.2`.

## [1.1.1] - 2023-06-22

### Changed

- Updated `heroku/jvm` to `1.1.1`
- Updated `heroku/maven` to `1.1.1`

## [0.6.11] - 2023-06-13

### Changed

- Upgraded `heroku/maven` to `1.0.5`

## [0.6.10] - 2023-05-11

### Changed

- Upgraded `heroku/maven` to `1.0.4`
- Upgraded `heroku/jvm` to `1.0.10`
- The buildpack now implements Buildpack API 0.9 instead of 0.8, and so requires `lifecycle` 0.15.x or newer. ([#491](https://github.com/heroku/buildpacks-jvm/pull/491))

## [0.6.9] - 2023-04-24

### Changed

- Upgraded `heroku/jvm` to `1.0.9`

## [0.6.8] - 2023-03-31

### Changed

- Upgraded `heroku/jvm` to `1.0.8`

## [0.6.7] - 2023-03-23

### Changed

- Upgraded `heroku/jvm` to `1.0.7`

## [0.6.6] - 2023-01-18

### Changed

- Upgraded `heroku/jvm` to `1.0.6`

## [0.6.5] - 2022-10-20

### Changed

- Upgraded `heroku/jvm` to `1.0.5`

## [0.6.4] - 2022-09-28

### Changed

- Upgraded `heroku/maven` to `1.0.3`
- Upgraded `heroku/jvm` to `1.0.4`
- Upgraded `heroku/procfile` to `2.0.0`. ([#374](https://github.com/heroku/buildpacks-jvm/pull/374))
- Buildpack now implements buildpack API version `0.8` and so requires lifecycle version `0.14.x` or newer. ([#363](https://github.com/heroku/buildpacks-jvm/pull/363))

## [0.6.3] - 2022-08-29

### Changed

- Upgraded `heroku/jvm` to `1.0.3`

## [0.6.2] - 2022-07-29

### Changed

- Upgraded `heroku/maven` to `1.0.2`

## [0.6.1] - 2022-07-26

### Changed

- Upgraded `heroku/jvm` to `1.0.2`
- Upgraded `heroku/procfile` to `1.0.2`

## [0.6.0] - 2022-06-09

### Changed

- Upgraded `heroku/maven` to `1.0.1`
- Upgraded `heroku/jvm` to `1.0.1`

### Removed

- Remove Gradle support from this meta-buildpack. Gradle support was realized by using a shimmed version of the `heroku/gradle` Heroku buildpack. We decided to strictly separate shimmed buildpacks from proper CNBs. Gradle support will be re-added later, using a native CNB. ([#308](https://github.com/heroku/buildpacks-jvm/pull/308))

## [0.5.0] - 2022-05-17

### Changed

- Upgraded `heroku/jvm` to `1.0.0`
- Upgraded `heroku/procfile` to `1.0.1`

## [0.3.16] - 2022-03-24

### Changed

- Upgraded `heroku/jvm` to `0.1.15`
- Upgraded `heroku/maven` to `1.0.0`

## [0.3.15] - 2022-03-02

### Changed

- Upgraded `heroku/jvm` to `0.1.14`
- Upgraded `heroku/maven` to `0.2.6`

## [0.3.14] - 2022-01-24

### Changed

- Upgraded `heroku/jvm` to `0.1.12`
- Update github-action to upload buildpackage to Github Releases
- Switch to BSD 3-Clause License

## [0.3.13] - 2021-10-28

### Changed

- Upgraded `heroku/jvm` to `0.1.11`
- Upgraded `heroku/jvm` to `0.1.10`
- Upgraded `heroku/jvm` to `0.1.9`

## [0.3.12] - 2021-10-19

- No changes.

## [0.3.11] - 2021-09-15

### Changed

- Upgraded `heroku/jvm` to `0.1.8`

## [0.3.10] - 2021-08-10

### Changed

- Upgraded `heroku/maven` to `0.2.5`

## [0.3.9] - 2021-07-28

### Changed

- Upgraded `heroku/jvm` to `0.1.7`

## [0.3.8] - 2021-07-16

### Changed

- Upgraded `heroku/maven` to `0.2.4`

## [0.3.7] - 2021-05-05

### Changed

- Upgraded `heroku/maven` to `0.2.3`

## [0.3.6] - 2021-04-29

### Changed

- Upgraded `heroku/jvm` to `0.1.6`

### Fixed

### Changed

- Fixed `licenses` in `buildpack.toml`

## [0.3.5] - 2021-03-17

### Changed

- Upgraded `heroku/jvm` to `0.1.5`

## [0.3.4] - 2021-03-15

### Changed

- Upgraded `heroku/procfile` to `0.6.2`

## [0.3.3] - 2021-02-23

### Changed

- Upgraded `heroku/gradle` to `0.0.35`
- Upgraded `heroku/maven` to `0.2.2`
- Upgraded `heroku/jvm` to `0.1.4`

## [0.3.2] - 2021-02-04

### Changed

- Upgraded `heroku/jvm` to `0.1.3`

## [0.3.1] - 2021-02-04

### Changed

- Upgraded `heroku/maven` to `0.2.1`

## [0.3.0] - 2021-02-03

### Changed

- Now packages released buildpack images instead of local paths to ensure standalone and bundled
  versions are exactly the same.

## [0.1.3] - 2021-01-22

### Changed

- Upgraded `heroku/jvm` to `0.1.3`

### Added

- Automated post-release PRs

## [0.1.2] - 2012-01-19

### Changes

- Upgrade `heroku/maven` to `0.2.0`

## [0.1.1] - 2012-01-13

### Added

- Initial release

[unreleased]: https://github.com/heroku/buildpacks-jvm/compare/v7.0.4...HEAD
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
