# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.1.1] - 2023-06-21

- Updated `heroku/jvm` to `1.1.1`
- Updated `heroku/maven` to `1.1.1`

## [0.6.11] - 2023-06-13

* Upgraded `heroku/maven` to `1.0.5`

## [0.6.10] - 2023-05-11

* Upgraded `heroku/maven` to `1.0.4`
* Upgraded `heroku/jvm` to `1.0.10`
* The buildpack now implements Buildpack API 0.9 instead of 0.8, and so requires `lifecycle` 0.15.x or newer. ([#491](https://github.com/heroku/buildpacks-jvm/pull/491))

## [0.6.9] - 2023-04-24

* Upgraded `heroku/jvm` to `1.0.9`

## [0.6.8] - 2023-03-31

* Upgraded `heroku/jvm` to `1.0.8`

## [0.6.7] - 2023-03-23

* Upgraded `heroku/jvm` to `1.0.7`

## [0.6.6] - 2023-01-18

* Upgraded `heroku/jvm` to `1.0.6`

## [0.6.5] - 2022-10-20

* Upgraded `heroku/jvm` to `1.0.5`

## [0.6.4] - 2022-09-28

* Upgraded `heroku/maven` to `1.0.3`
* Upgraded `heroku/jvm` to `1.0.4`

* Upgraded `heroku/procfile` to `2.0.0`. ([#374](https://github.com/heroku/buildpacks-jvm/pull/374))
* Buildpack now implements buildpack API version `0.8` and so requires lifecycle version `0.14.x` or newer. ([#363](https://github.com/heroku/buildpacks-jvm/pull/363))

## [0.6.3] - 2022-08-29

* Upgraded `heroku/jvm` to `1.0.3`

## [0.6.2] - 2022-07-29

* Upgraded `heroku/maven` to `1.0.2`

## [0.6.1] - 2022-07-26

* Upgraded `heroku/jvm` to `1.0.2`
* Upgraded `heroku/procfile` to `1.0.2`

## [0.6.0] - 2022-06-09

* Upgraded `heroku/maven` to `1.0.1`
* Upgraded `heroku/jvm` to `1.0.1`
### Breaking
* Remove Gradle support from this meta-buildpack. Gradle support was realized by using a shimmed version of the `heroku/gradle` Heroku buildpack. We decided to strictly separate shimmed buildpacks from proper CNBs. Gradle support will be re-added later, using a native CNB. ([#308](https://github.com/heroku/buildpacks-jvm/pull/308))

## [0.5.0] - 2022-05-17

* Upgraded `heroku/jvm` to `1.0.0`
* Upgraded `heroku/procfile` to `1.0.1`

## [0.3.16] - 2022-03-24

* Upgraded `heroku/jvm` to `0.1.15`
* Upgraded `heroku/maven` to `1.0.0`

## [0.3.15] - 2022-03-02

* Upgraded `heroku/jvm` to `0.1.14`
* Upgraded `heroku/maven` to `0.2.6`

## [0.3.14] - 2022-01-24

* Upgraded `heroku/jvm` to `0.1.12`
* Update github-action to upload buildpackage to Github Releases
* Switch to BSD 3-Clause License

## [0.3.13] - 2021-10-28

* Upgraded `heroku/jvm` to `0.1.11`
* Upgraded `heroku/jvm` to `0.1.10`
* Upgraded `heroku/jvm` to `0.1.9`

## [0.3.12] - 2021-10-19



## [0.3.11] - 2021-09-15

* Upgraded `heroku/jvm` to `0.1.8`

## [0.3.10] - 2021-08-10

* Upgraded `heroku/maven` to `0.2.5`

## [0.3.9] - 2021-07-28

* Upgraded `heroku/jvm` to `0.1.7`

## [0.3.8] - 2021-07-16

* Upgraded `heroku/maven` to `0.2.4`

## [0.3.7] - 2021-05-05

* Upgraded `heroku/maven` to `0.2.3`

## [0.3.6] - 2021-04-29

* Upgraded `heroku/jvm` to `0.1.6`
### Fixed
* Fixed `licenses` in `buildpack.toml`

## [0.3.5] - 2021-03-17

* Upgraded `heroku/jvm` to `0.1.5`

## [0.3.4] - 2021-03-15

* Upgraded `heroku/procfile` to `0.6.2`

## [0.3.3] - 2021-02-23

* Upgraded `heroku/gradle` to `0.0.35`
* Upgraded `heroku/maven` to `0.2.2`
* Upgraded `heroku/jvm` to `0.1.4`

## [0.3.2] - 2021-02-04

* Upgraded `heroku/jvm` to `0.1.3`

## [0.3.1] - 2021-02-04

* Upgraded `heroku/maven` to `0.2.1`

## [0.3.0] - 2021-02-03

### Changed
* Now packages released buildpack images instead of local paths to ensure standalone and bundled
  versions are exactly the same.

## [0.1.3] - 2021-01-22

* Upgraded `heroku/jvm` to `0.1.3`

### Added
* Automated post-release PRs

[unreleased]: https://github.com/heroku/buildpacks-jvm/compare/v1.1.1...HEAD
[1.1.1]: https://github.com/heroku/buildpacks-jvm/compare/v0.6.11...v1.1.1
[0.6.11]: https://github.com/heroku/buildpacks-jvm/compare/v0.6.10...v0.6.11
[0.6.10]: https://github.com/heroku/buildpacks-jvm/compare/v0.6.9...v0.6.10
[0.6.9]: https://github.com/heroku/buildpacks-jvm/compare/v0.6.8...v0.6.9
[0.6.8]: https://github.com/heroku/buildpacks-jvm/compare/v0.6.7...v0.6.8
[0.6.7]: https://github.com/heroku/buildpacks-jvm/compare/v0.6.6...v0.6.7
[0.6.6]: https://github.com/heroku/buildpacks-jvm/compare/v0.6.5...v0.6.6
[0.6.5]: https://github.com/heroku/buildpacks-jvm/compare/v0.6.4...v0.6.5
[0.6.4]: https://github.com/heroku/buildpacks-jvm/compare/v0.6.3...v0.6.4
[0.6.3]: https://github.com/heroku/buildpacks-jvm/compare/v0.6.2...v0.6.3
[0.6.2]: https://github.com/heroku/buildpacks-jvm/compare/v0.6.1...v0.6.2
[0.6.1]: https://github.com/heroku/buildpacks-jvm/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/heroku/buildpacks-jvm/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/heroku/buildpacks-jvm/compare/v0.3.16...v0.5.0
[0.3.16]: https://github.com/heroku/buildpacks-jvm/compare/v0.3.15...v0.3.16
[0.3.15]: https://github.com/heroku/buildpacks-jvm/compare/v0.3.14...v0.3.15
[0.3.14]: https://github.com/heroku/buildpacks-jvm/compare/v0.3.13...v0.3.14
[0.3.13]: https://github.com/heroku/buildpacks-jvm/compare/v0.3.12...v0.3.13
[0.3.12]: https://github.com/heroku/buildpacks-jvm/compare/v0.3.11...v0.3.12
[0.3.11]: https://github.com/heroku/buildpacks-jvm/compare/v0.3.10...v0.3.11
[0.3.10]: https://github.com/heroku/buildpacks-jvm/compare/v0.3.9...v0.3.10
[0.3.9]: https://github.com/heroku/buildpacks-jvm/compare/v0.3.8...v0.3.9
[0.3.8]: https://github.com/heroku/buildpacks-jvm/compare/v0.3.7...v0.3.8
[0.3.7]: https://github.com/heroku/buildpacks-jvm/compare/v0.3.6...v0.3.7
[0.3.6]: https://github.com/heroku/buildpacks-jvm/compare/v0.3.5...v0.3.6
[0.3.5]: https://github.com/heroku/buildpacks-jvm/compare/v0.3.4...v0.3.5
[0.3.4]: https://github.com/heroku/buildpacks-jvm/compare/v0.3.3...v0.3.4
[0.3.3]: https://github.com/heroku/buildpacks-jvm/compare/v0.3.2...v0.3.3
[0.3.2]: https://github.com/heroku/buildpacks-jvm/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/heroku/buildpacks-jvm/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/heroku/buildpacks-jvm/compare/v0.1.3...v0.3.0
[0.1.3]: https://github.com/heroku/buildpacks-jvm/releases/tag/v0.1.3