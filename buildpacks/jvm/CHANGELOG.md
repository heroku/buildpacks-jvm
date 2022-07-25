# Changelog
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

* Default version for **OpenJDK 11** is now `11.0.16`
* Default version for **OpenJDK 13** is now `13.0.12`
* Default version for **OpenJDK 15** is now `15.0.8`
* Default version for **OpenJDK 17** is now `17.0.4`
* Default version for **OpenJDK 18** is now `18.0.2`
* Updated `libcnb` and `libherokubuildpack` to `0.9.0`. ([#330](https://github.com/heroku/buildpacks-nodejs/pull/330))
* Switch to the recommended regional S3 domain instead of the global one. ([#314](https://github.com/heroku/buildpacks-jvm/pull/314))
* Upgrade `libcnb` to `0.8.0` and `libherokubuildpack` to `0.8.0`.

## [1.0.1] 2022/06/09

* Add support for the `heroku-22` stack. ([#304](https://github.com/heroku/buildpacks-jvm/pull/304))
* [Azul Zulu Builds of OpenJDK](https://www.azul.com/downloads/?package=jdk#download-openjdk) is now the default OpenJDK distribution. This change does not affect the `heroku-18` and `heroku-20` stack. ([#304](https://github.com/heroku/buildpacks-jvm/pull/304))

## [1.0.0] 2022/05/17

* Re-implement buildpack using [libcnb.rs](https://github.com/heroku/libcnb.rs) ([#272](https://github.com/heroku/buildpacks-jvm/pull/272))
* Remove support for GPG signed OpenJDK binaries. This feature wasn't used and will be replaced by a unified solution across Heroku buildpacks. ([#272](https://github.com/heroku/buildpacks-jvm/pull/272))
* Remove support for the `JDK_BASE_URL` environment variable. It was deprecated in Jan 2021 and was slated for removal Oct 2021. ([#272](https://github.com/heroku/buildpacks-jvm/pull/272))
* Remove support for the `JVM_BUILDPACK_ASSETS_BASE_URL` environment variable. ([#272](https://github.com/heroku/buildpacks-jvm/pull/272))
* Remove legacy debugging scripts: `with_jmap`, `with_jmap_and_jstack`, `with_jmap_and_jstack_java`, `with_jmap_java`, `with_jstack` and `with_jstack_java`. ([#272](https://github.com/heroku/buildpacks-jvm/pull/272))
* Remove explicit setting of `-XX:+UseContainerSupport` as it's nowadays the default for all supported Java versions. ([#272](https://github.com/heroku/buildpacks-jvm/pull/272))
* Fixed caching behaviour when a JDK overlay was used. Updated overlays will now be always applied to a clean version of OpenJDK. ([#272](https://github.com/heroku/buildpacks-jvm/pull/272))
* Improved compatibility when reading Java properties files (`system.properties`). ([#272](https://github.com/heroku/buildpacks-jvm/pull/272))
* Support for selecting a major version when using Azul Zulu as the OpenJDK distribution. Users no longer have to pick a specific version when using Azul Zulu. To select, for example, the latest OpenJDK 11 release from Azul Zulu, use `java.runtime.version=zulu-11` in your `system.properties` file. ([#272](https://github.com/heroku/buildpacks-jvm/pull/272))
* Add checksum validation when installing the Heroku JVM Metrics Agent. ([#272](https://github.com/heroku/buildpacks-jvm/pull/272))
* No longer installs `jq` and `yj` command-line tools during the buildpack bootstrap, improving overall build times. ([#272](https://github.com/heroku/buildpacks-jvm/pull/272))
* Improved compatibility when rewriting the `DATABASE_URL` environment variable by using proper URL parsing. ([#272](https://github.com/heroku/buildpacks-jvm/pull/272))
* Improved error messages when unexpected IO errors occur during the build. ([#272](https://github.com/heroku/buildpacks-jvm/pull/272))
* Default version for **OpenJDK 7** is now `1.7.0_342`
* Default version for **OpenJDK 8** is now `1.8.0_332`
* Default version for **OpenJDK 11** is now `11.0.15`
* Default version for **OpenJDK 13** is now `13.0.11`
* Default version for **OpenJDK 15** is now `15.0.7`
* Default version for **OpenJDK 17** is now `17.0.3`
* Default version for **OpenJDK 18** is now `18.0.1`


## [0.1.15] 2022/03/24

### Added
* Support for Java 18

## [0.1.14] 2022/03/02

### Changed

* Default version for **OpenJDK 11** is now `11.0.14.1`

### Fixed

* JDK overlays (using the `.jdk-overlay` directory) are now properly applied

## [0.1.12] 2022/01/24
* Switch to BSD 3-Clause License
* Default version for **OpenJDK 7** is now `1.7.0_332`
* Default version for **OpenJDK 8** is now `1.8.0_322`
* Default version for **OpenJDK 11** is now `11.0.14`
* Default version for **OpenJDK 13** is now `13.0.10`
* Default version for **OpenJDK 15** is now `15.0.6`
* Default version for **OpenJDK 17** is now `17.0.2`

## [0.1.11] 2021/10/28
### Changed
* Default version for **OpenJDK 7** is now `1.7.0_322`
* Default version for **OpenJDK 17** is now `17.0.1`

## [0.1.10] 2021/10/27

## [0.1.9] 2021/10/19
### Changed
* Default version for **OpenJDK 8** is now `1.8.0_312`
* Default version for **OpenJDK 11** is now `11.0.13`
* Default version for **OpenJDK 13** is now `13.0.9`
* Default version for **OpenJDK 15** is now `15.0.5`

## [0.1.8] 2021/09/15
### Added
* Support for Java 17

### Fixed
* Updated GPG public key

## [0.1.7] 2021/07/28
### Changed
* Default version for **OpenJDK 7** is now `1.7.0_312`
* Default version for **OpenJDK 8** is now `1.8.0_302`
* Default version for **OpenJDK 11** is now `11.0.12`
* Default version for **OpenJDK 13** is now `13.0.8`
* Default version for **OpenJDK 15** is now `15.0.4`
* Default version for **OpenJDK 16** is now `16.0.2`

## [0.1.6] 2021/04/29
### Changed
* Default version for **OpenJDK 7** is now `1.7.0_302`
* Default version for **OpenJDK 8** is now `1.8.0_292`
* Default version for **OpenJDK 11** is now `11.0.11`
* Default version for **OpenJDK 13** is now `13.0.7`
* Default version for **OpenJDK 15** is now `15.0.3`
* Default version for **OpenJDK 16** is now `16.0.1`

### Fixed
* Fixed `licenses` in `buildpack.toml`

## [0.1.5] 2021/03/17
### Added
* Support for Java 16

## [0.1.4] 2021/02/23

## [0.1.3] 2021/02/04
### Changed
* Status headers are now bold

### Fixed
* `JAVA_HOME` will now be correctly set when using older versions of `pack`

## [0.1.2] 2021/01/22
### Changed
* Default version for **OpenJDK 7** is now `1.7.0_292`
* Default version for **OpenJDK 8** is now `1.8.0_282`
* Default version for **OpenJDK 11** is now `11.0.10`
* Default version for **OpenJDK 13** is now `13.0.6`
* Default version for **OpenJDK 15** is now `15.0.2`

## [0.1.1] 2021/01/19
### Added
* Automated post-release PRs

## [0.1.0]
* Initial release
