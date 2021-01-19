# Changelog
## main

## 0.2.0
* Code refactoring
* Logging style now adheres to Heroku's CNB logging style
* Maven options that are implementation details are no longer logged by default
* Maven options that are required for proper operation of this buildpack can no longer be overridden by
  `MAVEN_CUSTOM_OPTS` or `MAVEN_CUSTOM_GOALS`
* Added debug logging, can be enabled by setting `HEROKU_BUILDPACK_DEBUG` environment variable
* Caching of Maven dependencies has been fixed
* Fixed exit code of `bin/detect` when detection failed without an error

## 0.1.1
* Initial release
