# Changelog
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Breaking
* This buildpack no longer adds a `requires` entry for itself in the build plan. This means it cannot be used in a
  standalone fashion anymore. Another buildpack has to explicitly `require` `jdk` in its detect phase. The change has
  been made to improve composability in cases where this buildpack is marked as optional in meta-buildpacks or stack
  images.

### Added
* Debug logging, can be enabled by setting `HEROKU_BUILDPACK_DEBUG` environment variable.
* Checksum checking of downloaded OpenJDK distributions.
* Support for installing Zulu builds of OpenJDK without specifying a concrete version. (e.g. it's now possible to
  specify `zulu-11` to install the latest Zulu build of OpenJDK 11).

### Changed
* Logging style now adheres to Heroku's CNB logging style.
* Available OpenJDK versions are now recorded in a metadata-file. This means that new OpenJDK versions will not be
  available without a new version of this buildpack.
* "toolbox" dependencies are no longer downloaded during build and are now bundled with the buildpack. This slightly
  improves build times.

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
