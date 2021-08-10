# Changelog
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.0] 2021/08/10
* Changed implementation to Rust (relanded with upgrade to libcnb `0.1.3`)

## [0.4.0] 2021/07/28
* Revert all changes from `0.3.0`
* Updated function runtime to `1.0.0` again

## [0.3.0] 2021/07/15
* Changed implementation to Rust
* Updated function runtime to `1.0.0`

## [0.2.11] 2021/05/21
* Updated function runtime to `0.2.4`

## [0.2.10] 2021/05/18
* Updated function runtime to `0.2.3`

## [0.2.9] 2021/05/17
* Updated function runtime to `0.2.2`

## [0.2.8] 2021/05/17
### Changed
* `SF_FX_REMOTE_DEBUG` was renamed to `DEBUG_PORT` and also species the port on with the JDWP agent will listen on.
* Updated function runtime to `0.2.1`
* Update `bin/detect` to check for `type=function`.

## [0.2.7] 2021/05/05
### Changed
* Updated function runtime to `0.2.0`

### Added
* Support for the `SF_FX_REMOTE_DEBUG` runtime environment variable. If set, the invoker will listen for incoming JDWP
  connections on port `5005`.

### Changed
* Detection now checks for `project.toml` in addition to `function.toml` to determine if an app is a function.

## [0.2.6] 2021/04/29
### Changed
* Updated function runtime to `0.1.4-ea`

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
