# Changelog
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
* Add documentation to `README.md`

## [0.2.2] 2021/02/23

## [0.2.1] 2021/02/03
### Added
* Automated post-release PRs
* Now requires (in the CNB sense) `jdk` to pass detection
* Now provides (in the CNB sense) `jvm-application` to subsequent buildpacks

## [0.2.0]
### Added
* Debug logging, can be enabled by setting `HEROKU_BUILDPACK_DEBUG` environment variable

### Changed
* Code refactoring
* Logging style now adheres to Heroku's CNB logging style
* Maven options that are implementation details are no longer logged by default
* Maven options that are required for proper operation of this buildpack can no longer be overridden by
  `MAVEN_CUSTOM_OPTS` or `MAVEN_CUSTOM_GOALS`

### Fixed
* Caching of Maven dependencies
* Exit code of `bin/detect` when detection failed without an error

## [0.1.0]
* Initial release
