# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [6.0.3] - 2024-09-26

### Added

- Support for OpenJDK 23. ([#728](https://github.com/heroku/buildpacks-jvm/pull/728))

## [6.0.2] - 2024-09-25

- No changes.

## [6.0.1] - 2024-07-19

### Added

- Support for new OpenJDK versions: `1.8.0_422`, `11.0.24`, `17.0.12`, `21.0.4`, `22.0.2`. ([#706](https://github.com/heroku/buildpacks-jvm/pull/706))

### Changed

- Default version for **OpenJDK 8** is now `1.8.0_422`. ([#706](https://github.com/heroku/buildpacks-jvm/pull/706))
- Default version for **OpenJDK 11** is now `11.0.24`. ([#706](https://github.com/heroku/buildpacks-jvm/pull/706))
- Default version for **OpenJDK 17** is now `17.0.12`. ([#706](https://github.com/heroku/buildpacks-jvm/pull/706))
- Default version for **OpenJDK 21** is now `21.0.4`. ([#706](https://github.com/heroku/buildpacks-jvm/pull/706))
- Default version for **OpenJDK 22** is now `22.0.2`. ([#706](https://github.com/heroku/buildpacks-jvm/pull/706))

## [6.0.0] - 2024-05-28

### Added

- Checksum validation of downloaded OpenJDK distribution files. ([#680](https://github.com/heroku/buildpacks-jvm/pull/680))
- A warning will now be shown when the OpenJDK version is not explicitly configured for an application. ([#681](https://github.com/heroku/buildpacks-jvm/pull/681))

### Changed

- This buildpack now installs the latest long-term support release (currently 21) of OpenJDK if no version is explicitly configured. Previously, OpenJDK 8 was installed as the default. ([#681](https://github.com/heroku/buildpacks-jvm/pull/681))
- Some error messages have changed so they longer suggest to open a Heroku support ticket. Instead, users are now provided with a link to create an issue on GitHub. ([#674](https://github.com/heroku/buildpacks-jvm/pull/674))

### Removed

- The buildpack no longer installs [Heroku's Java Metrics Agent](https://github.com/heroku/heroku-java-metrics-agent). It is only useful on the Heroku platform and can be installed by a dedicated buildpack in the future. ([#679](https://github.com/heroku/buildpacks-jvm/pull/679))

## [5.0.1] - 2024-05-23

### Changed

- Fall back to `std::env::consts::ARCH` when `CNB_TARGET_ARCHITECTURE` is missing. This is a workaround for certain lifecycle and platform API combinations. ([#672](https://github.com/heroku/buildpacks-jvm/pull/672))

## [5.0.0] - 2024-05-23

### Added

- Support for the `arm64` architecture. ([#668](https://github.com/heroku/buildpacks-jvm/pull/668))

### Changed

- Buildpack API version changed from `0.9` to `0.10`, and so requires `lifecycle` `0.17.x` or newer. ([#662](https://github.com/heroku/buildpacks-jvm/pull/662))
- This buildpack is no longer Heroku stack specific and can be used with most `amd64` or `arm64` Linux based CNB build and run images. ([#662](https://github.com/heroku/buildpacks-jvm/pull/662))
- Default OpenJDK distribution is now always Azul® Zulu®. ([#662](https://github.com/heroku/buildpacks-jvm/pull/662))
- Some error messages relating to OpenJDK installation changed. ([#665](https://github.com/heroku/buildpacks-jvm/pull/665))
- OpenJDK is now downloaded from `heroku-buildpacks-jvm.s3.us-east-1.amazonaws.com`. Users that use allow-listing for internet access during builds might need to add this new URL to their allow-list. ([#665](https://github.com/heroku/buildpacks-jvm/pull/665))

### Removed

- Support for Heroku's own OpenJDK build distribution. Users that explicitly request that distribution by prefixing their OpenJDK version with `heroku-` need to either remove the prefix or replace it with `zulu-`. Azul Zulu is a drop-in replacement for the Heroku OpenJDK distribution. ([#662](https://github.com/heroku/buildpacks-jvm/pull/662))

## [4.1.1] - 2024-05-01

### Changed

- Default version for **OpenJDK 8** is now `1.8.0_412`. ([#657](https://github.com/heroku/buildpacks-jvm/pull/657))
- Default version for **OpenJDK 11** is now `11.0.23`. ([#657](https://github.com/heroku/buildpacks-jvm/pull/657))
- Default version for **OpenJDK 17** is now `17.0.11`.([#657](https://github.com/heroku/buildpacks-jvm/pull/657))
- Default version for **OpenJDK 21** is now `21.0.3`. ([#657](https://github.com/heroku/buildpacks-jvm/pull/657))

## [4.1.0] - 2024-01-23

### Changed

- For containers that do not correspond to `Eco`, `Basic`, `Standard-1X`, `Standard-2X` or `Private-S` Heroku dyno types, only `-XX:MaxRAMPercentage=80.0` will now be passed to the JVM via `JAVA_TOOL_OPTIONS`. ([#631](https://github.com/heroku/buildpacks-jvm/pull/631))
- Default version for **OpenJDK 8** is now `1.8.0_402`. ([#630](https://github.com/heroku/buildpacks-jvm/pull/630))
- Default version for **OpenJDK 11** is now `11.0.22`. ([#630](https://github.com/heroku/buildpacks-jvm/pull/630))
- Default version for **OpenJDK 17** is now `17.0.10`.([#630](https://github.com/heroku/buildpacks-jvm/pull/630))
- Default version for **OpenJDK 21** is now `21.0.2`. ([#630](https://github.com/heroku/buildpacks-jvm/pull/630))

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

### Changed

- Default version for **OpenJDK 8** is now `1.8.0_392`. ([#599](https://github.com/heroku/buildpacks-jvm/pull/599))
- Default version for **OpenJDK 11** is now `11.0.21`. ([#599](https://github.com/heroku/buildpacks-jvm/pull/599))
- Default version for **OpenJDK 17** is now `17.0.9`. ([#599](https://github.com/heroku/buildpacks-jvm/pull/599))
- Default version for **OpenJDK 21** is now `21.0.1`. ([#599](https://github.com/heroku/buildpacks-jvm/pull/599))

## [3.2.0] - 2023-09-20

### Added

- Support for Java 21. ([#585](https://github.com/heroku/buildpacks-jvm/pull/585))

## [3.1.0] - 2023-09-18

### Changed

- Default version for **OpenJDK 11** is now `11.0.20.1`. ([#582](https://github.com/heroku/buildpacks-jvm/pull/582))
- Default version for **OpenJDK 17** is now `17.0.8.1`. ([#582](https://github.com/heroku/buildpacks-jvm/pull/582))

## [3.0.0] - 2023-08-09

- No changes.

## [2.0.0] - 2023-07-31

### Changed

- This buildpack now requires that an OpenJDK version is specified in `system.properties` when the buildpack is used standalone (no other buildpack adds `jdk` to the build plan `require`s). Users that use this buildpack in a standalone fashion can add a `system.properties` file to their application with the following contents to restore the old behavior of installing the most recent OpenJDK 8 release: `java.runtime.version=8`. ([#546](https://github.com/heroku/buildpacks-jvm/pull/546))

## [1.1.2] - 2023-07-24

### Changed

- Default version for **OpenJDK 8** is now `1.8.0_382`. ([#543](https://github.com/heroku/buildpacks-jvm/pull/543))
- Default version for **OpenJDK 11** is now `11.0.20`. ([#543](https://github.com/heroku/buildpacks-jvm/pull/543))
- Default version for **OpenJDK 17** is now `17.0.8`. ([#543](https://github.com/heroku/buildpacks-jvm/pull/543))
- Default version for **OpenJDK 20** is now `20.0.2`. ([#543](https://github.com/heroku/buildpacks-jvm/pull/543))

## [1.1.1] - 2023-06-22

### Removed

- Removed support for the `heroku-18` stack due to the stack being EOL and no longer maintained. ([#498](https://github.com/heroku/buildpacks-jvm/pull/498))
- Removed support for the `io.buildpacks.stacks.bionic` stack from `buildpack.toml`. Since the stack id is used for URL construction, this stack never properly worked. ([#498](https://github.com/heroku/buildpacks-jvm/pull/498))

## [1.0.10] - 2023-05-10

### Changed

- Upgrade `libcnb` and `libherokubuildpack` to `0.12.0`. ([#463](https://github.com/heroku/buildpacks-jvm/pull/463))
- The buildpack now implements Buildpack API 0.9 instead of 0.8, and so requires `lifecycle` 0.15.x or newer. ([#463](https://github.com/heroku/buildpacks-jvm/pull/463))

## [1.0.9] - 2023-04-24

### Changed

- Default version for **OpenJDK 8** is now `1.8.0_372`. ([#459](https://github.com/heroku/buildpacks-jvm/pull/459))
- Default version for **OpenJDK 11** is now `11.0.19`. ([#459](https://github.com/heroku/buildpacks-jvm/pull/459))
- Default version for **OpenJDK 17** is now `17.0.7`. ([#459](https://github.com/heroku/buildpacks-jvm/pull/459))
- Default version for **OpenJDK 20** is now `20.0.1`. ([#459](https://github.com/heroku/buildpacks-jvm/pull/459))

## [1.0.8] - 2023-03-31

### Added

- Add support for `SKIP_HEROKU_JVM_METRICS_AGENT_INSTALLATION` environment variable. When set to `true`, the installation of the [Heroku JVM metrics agent](https://github.com/heroku/heroku-java-metrics-agent) will be skipped. ([#444](https://github.com/heroku/buildpacks-jvm/pull/444))

### Changed

- Update [Heroku JVM metrics agent](https://github.com/heroku/heroku-java-metrics-agent) to the most recent version, `4.0.1`. This is a backwards compatible version bump. ([#445](https://github.com/heroku/buildpacks-jvm/pull/445))

## [1.0.7] - 2023-03-23

### Added

- Support for Java 20. ([#437](https://github.com/heroku/buildpacks-jvm/pull/437))

## [1.0.6] - 2023-01-18

### Changed

- Default version for **OpenJDK 8** is now `1.8.0_362`. ([#413](https://github.com/heroku/buildpacks-jvm/pull/413))
- Default version for **OpenJDK 11** is now `11.0.18`. ([#413](https://github.com/heroku/buildpacks-jvm/pull/413))
- Default version for **OpenJDK 13** is now `13.0.14`. ([#413](https://github.com/heroku/buildpacks-jvm/pull/413))
- Default version for **OpenJDK 15** is now `15.0.10`. ([#413](https://github.com/heroku/buildpacks-jvm/pull/413))
- Default version for **OpenJDK 17** is now `17.0.6`. ([#413](https://github.com/heroku/buildpacks-jvm/pull/413))
- Default version for **OpenJDK 19** is now `19.0.2`. ([#413](https://github.com/heroku/buildpacks-jvm/pull/413))

## [1.0.5] - 2022-10-20

### Changed

- Default version for **OpenJDK 8** is now `1.8.0_352`. ([#387](https://github.com/heroku/buildpacks-jvm/pull/387))
- Default version for **OpenJDK 11** is now `11.0.17`. ([#387](https://github.com/heroku/buildpacks-jvm/pull/387))
- Default version for **OpenJDK 13** is now `13.0.13`. ([#387](https://github.com/heroku/buildpacks-jvm/pull/387))
- Default version for **OpenJDK 15** is now `15.0.9`. ([#387](https://github.com/heroku/buildpacks-jvm/pull/387))
- Default version for **OpenJDK 17** is now `17.0.5`. ([#387](https://github.com/heroku/buildpacks-jvm/pull/387))
- Default version for **OpenJDK 19** is now `19.0.1`. ([#387](https://github.com/heroku/buildpacks-jvm/pull/387))
- Upgrade `libcnb` and `libherokubuildpack` to `0.11.1`. ([#384](https://github.com/heroku/buildpacks-jvm/pull/384) and [#386](https://github.com/heroku/buildpacks-jvm/pull/386))

## [1.0.4] - 2022-09-28

### Changed

- Upgrade `libcnb` and `libherokubuildpack` to `0.11.0`. ([#371](https://github.com/heroku/buildpacks-jvm/pull/371))
- Buildpack now implements buildpack API version `0.8` and so requires lifecycle version `0.14.x` or newer. ([#363](https://github.com/heroku/buildpacks-jvm/pull/363))
- Default version for **OpenJDK 18** is now `18.0.2.1`. ([#372](https://github.com/heroku/buildpacks-jvm/pull/372))

### Added

- Support for Java 19. ([#372](https://github.com/heroku/buildpacks-jvm/pull/372))

## [1.0.3] - 2022-08-29

### Changed

- Default version for **OpenJDK 8** is now `1.8.0_345`
- Default version for **OpenJDK 11** is now `11.0.16.1`
- Default version for **OpenJDK 17** is now `17.0.4.1`

## [1.0.2] - 2022-07-26

### Changed

- Default version for **OpenJDK 7** is now `1.7.0_352`
- Default version for **OpenJDK 8** is now `1.8.0_342`
- Default version for **OpenJDK 11** is now `11.0.16`
- Default version for **OpenJDK 13** is now `13.0.12`
- Default version for **OpenJDK 15** is now `15.0.8`
- Default version for **OpenJDK 17** is now `17.0.4`
- Default version for **OpenJDK 18** is now `18.0.2`
- Updated `libcnb` and `libherokubuildpack` to `0.9.0`. ([#330](https://github.com/heroku/buildpacks-jvm/pull/330))
- Switch to the recommended regional S3 domain instead of the global one. ([#314](https://github.com/heroku/buildpacks-jvm/pull/314))
- Upgrade `libcnb` to `0.8.0` and `libherokubuildpack` to `0.8.0`.

## [1.0.1] - 2022-06-09

### Added

- Add support for the `heroku-22` stack. ([#304](https://github.com/heroku/buildpacks-jvm/pull/304))

### Changed

- [Azul Zulu Builds of OpenJDK](https://www.azul.com/downloads/?package=jdk#download-openjdk) is now the default OpenJDK distribution. This change does not affect the `heroku-18` and `heroku-20` stack. ([#304](https://github.com/heroku/buildpacks-jvm/pull/304))

## [1.0.0] - 2022-05-17

### Added

- Add checksum validation when installing the Heroku JVM Metrics Agent. ([#272](https://github.com/heroku/buildpacks-jvm/pull/272))
- Support for selecting a major version when using Azul Zulu as the OpenJDK distribution. Users no longer have to pick a specific version when using Azul Zulu. To select, for example, the latest OpenJDK 11 release from Azul Zulu, use `java.runtime.version=zulu-11` in your `system.properties` file. ([#272](https://github.com/heroku/buildpacks-jvm/pull/272))

### Removed

- Remove support for GPG signed OpenJDK binaries. This feature wasn't used and will be replaced by a unified solution across Heroku buildpacks. ([#272](https://github.com/heroku/buildpacks-jvm/pull/272))
- Remove support for the `JDK_BASE_URL` environment variable. It was deprecated in Jan 2021 and was slated for removal Oct 2021. ([#272](https://github.com/heroku/buildpacks-jvm/pull/272))
- Remove support for the `JVM_BUILDPACK_ASSETS_BASE_URL` environment variable. ([#272](https://github.com/heroku/buildpacks-jvm/pull/272))
- Remove legacy debugging scripts: `with_jmap`, `with_jmap_and_jstack`, `with_jmap_and_jstack_java`, `with_jmap_java`, `with_jstack` and `with_jstack_java`. ([#272](https://github.com/heroku/buildpacks-jvm/pull/272))
- Remove explicit setting of `-XX:+UseContainerSupport` as it's nowadays the default for all supported Java versions. ([#272](https://github.com/heroku/buildpacks-jvm/pull/272))
- No longer installs `jq` and `yj` command-line tools during the buildpack bootstrap, improving overall build times. ([#272](https://github.com/heroku/buildpacks-jvm/pull/272))

### Changed

- Default version for **OpenJDK 7** is now `1.7.0_342`
- Default version for **OpenJDK 8** is now `1.8.0_332`
- Default version for **OpenJDK 11** is now `11.0.15`
- Default version for **OpenJDK 13** is now `13.0.11`
- Default version for **OpenJDK 15** is now `15.0.7`
- Default version for **OpenJDK 17** is now `17.0.3`
- Default version for **OpenJDK 18** is now `18.0.1`
- Improved compatibility when reading Java properties files (`system.properties`). ([#272](https://github.com/heroku/buildpacks-jvm/pull/272))
- Improved compatibility when rewriting the `DATABASE_URL` environment variable by using proper URL parsing. ([#272](https://github.com/heroku/buildpacks-jvm/pull/272))
- Improved error messages when unexpected IO errors occur during the build. ([#272](https://github.com/heroku/buildpacks-jvm/pull/272))
- Re-implement buildpack using [libcnb.rs](https://github.com/heroku/libcnb.rs) ([#272](https://github.com/heroku/buildpacks-jvm/pull/272))

### Fixed

- Fixed caching behaviour when a JDK overlay was used. Updated overlays will now be always applied to a clean version of OpenJDK. ([#272](https://github.com/heroku/buildpacks-jvm/pull/272))

## [0.1.15] - 2022-03-24

### Added

- Support for Java 18

## [0.1.14] - 2022-03-02

### Changed

- Default version for **OpenJDK 11** is now `11.0.14.1`

### Fixed

- JDK overlays (using the `.jdk-overlay` directory) are now properly applied

## [0.1.12] - 2022-01-24

- Switch to BSD 3-Clause License
- Default version for **OpenJDK 7** is now `1.7.0_332`
- Default version for **OpenJDK 8** is now `1.8.0_322`
- Default version for **OpenJDK 11** is now `11.0.14`
- Default version for **OpenJDK 13** is now `13.0.10`
- Default version for **OpenJDK 15** is now `15.0.6`
- Default version for **OpenJDK 17** is now `17.0.2`

## [0.1.11] - 2021-10-28

### Changed

- Default version for **OpenJDK 7** is now `1.7.0_322`
- Default version for **OpenJDK 17** is now `17.0.1`

## [0.1.10] - 2021-10-27

- No changes.

## [0.1.9] - 2021-10-19

### Changed

- Default version for **OpenJDK 8** is now `1.8.0_312`
- Default version for **OpenJDK 11** is now `11.0.13`
- Default version for **OpenJDK 13** is now `13.0.9`
- Default version for **OpenJDK 15** is now `15.0.5`

## [0.1.8] - 2021-09-15

### Added

- Support for Java 17

### Fixed

- Updated GPG public key

## [0.1.7] - 2021-07-28

### Changed

- Default version for **OpenJDK 7** is now `1.7.0_312`
- Default version for **OpenJDK 8** is now `1.8.0_302`
- Default version for **OpenJDK 11** is now `11.0.12`
- Default version for **OpenJDK 13** is now `13.0.8`
- Default version for **OpenJDK 15** is now `15.0.4`
- Default version for **OpenJDK 16** is now `16.0.2`

## [0.1.6] - 2021-04-29

### Changed

- Default version for **OpenJDK 7** is now `1.7.0_302`
- Default version for **OpenJDK 8** is now `1.8.0_292`
- Default version for **OpenJDK 11** is now `11.0.11`
- Default version for **OpenJDK 13** is now `13.0.7`
- Default version for **OpenJDK 15** is now `15.0.3`
- Default version for **OpenJDK 16** is now `16.0.1`

### Fixed

- Fixed `licenses` in `buildpack.toml`

## [0.1.5] - 2021-03-17

### Added

- Support for Java 16

## [0.1.4] - 2021-02-23

- No changes.

## [0.1.3] - 2021-02-04

### Changed

- Status headers are now bold

### Fixed

- `JAVA_HOME` will now be correctly set when using older versions of `pack`

## [0.1.2] - 2021-01-22

### Changed

- Default version for **OpenJDK 7** is now `1.7.0_292`
- Default version for **OpenJDK 8** is now `1.8.0_282`
- Default version for **OpenJDK 11** is now `11.0.10`
- Default version for **OpenJDK 13** is now `13.0.6`
- Default version for **OpenJDK 15** is now `15.0.2`

## [0.1.1] - 2021-01-19

### Added

- Automated post-release PRs

## [0.1.0] - 2021-01-14

### Added

- Initial release

[unreleased]: https://github.com/heroku/buildpacks-jvm/compare/v6.0.3...HEAD
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
