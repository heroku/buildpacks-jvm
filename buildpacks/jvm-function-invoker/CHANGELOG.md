# Changelog
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.5] 2021/04/21
### Changed
* Updated function runtime to `0.1.3-ea`

## [0.2.4] 2021/04/08
### Fixed
* Fixed `licenses` in `buildpack.toml`
* Updated function runtime to `0.1.1-ea`

## [0.2.3] 2021/02/23

## [0.2.2] 2021/02/04
### Added
* Support for the `PORT` environment variable at runtime for setting the HTTP port

### Fixed
* When using an older version of `pack`, the function layer might be incorrectly restored, causing errors
  "directory not empty" during function detection. A workaround has been added.

## [0.2.1] 2021/02/03
### Changed
* Now requires (in the CNB sense) `jvm-application` to pass detection.
* Will now fail detection if there is no `function.toml` present.

### Removed
* The Java function runtime binary integrity is now checked after download (temporarily removed).
* Java function runtime is now cached between builds (temporarily removed).

## [0.2.0] 2021/02/01
### Changed
* Function runtime binary URL no longer has to be specified with the `JVM_INVOKER_JAR_URL` environment variable.
* Functions are now detected during build. This means the build will now fail if more or less than one valid
  Salesforce Java function is detected in the project.

### Added
* The Java function runtime binary integrity is now checked after download.
* Java function runtime is now cached between builds.

## [0.1.0] 2021/01/21
### Added
* Initial release.
