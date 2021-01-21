# Heroku Cloud Native JVM Buildpacks
[![CircleCI](https://circleci.com/gh/heroku/buildpacks-jvm/tree/main.svg?style=shield)](https://circleci.com/gh/heroku/buildpacks-jvm/tree/main)

Heroku's official [Cloud Native Buildpacks](https://buildpacks.io) for the JVM ecosystem.

## Included Buildpacks
### Languages
Language buildpacks are meta-buildpacks that aggregate other buildpacks for convenient use. Use these if you want
to build your application.

- `heroku/java` (Java, [Readme](meta-buildpacks/java/README.md), [Changelog](meta-buildpacks/java/CHANGELOG.md))

### Platforms
- `heroku/jvm` (OpenJDK, [Readme](buildpacks/jvm/README.md), [Changelog](buildpacks/jvm/CHANGELOG.md))

### Build Tools
- `heroku/maven` (Maven, [Readme](buildpacks/maven/README.md), [Changelog](buildpacks/maven/CHANGELOG.md))

## External Buildpacks
In addition to the buildpacks in this repository, some buildpacks live in a dedicated repository.

- `heroku/gradle` ([GitHub](https://github.com/heroku/heroku-buildpack-gradle), V2 buildpack, [shimmed](https://github.com/heroku/cnb-shim))
- `heroku/scala` ([GitHub](https://github.com/heroku/heroku-buildpack-scala), V2 buildpack, [shimmed](https://github.com/heroku/cnb-shim))
- `heroku/clojure` ([GitHub](https://github.com/heroku/heroku-buildpack-clojure), V2 buildpack, [shimmed](https://github.com/heroku/cnb-shim))

## Classic Heroku Buildpacks

If you're looking for the repositories of the classic JVM Heroku buildpacks than can be used on the Heroku platform,
use the links below for your convenience.

- [heroku/java](https://github.com/heroku/heroku-buildpack-java)
- [heroku/gradle](https://github.com/heroku/heroku-buildpack-gradle)
- [heroku/scala](https://github.com/heroku/heroku-buildpack-scala)
- [heroku/clojure](https://github.com/heroku/heroku-buildpack-clojure)

## Building
Many of the buildpacks in this repository require a separate build step before they can be used. By convention, build
scripts must be located in a file named `build.sh` in the buildpack root directory.

### Build script conventions
`build.sh` scripts:
- **MUST NOT** depend on a specific working directory and can be called from anywhere
- **MUST** write the finished buildpack to `target/` within the buildpack directory

### Dependencies
- [Bash](https://www.gnu.org/software/bash/) >= `5.0`
- [yj](https://github.com/sclevine/yj) >= `5.0.0` in `$PATH`
- [jq](https://github.com/stedolan/jq) >= `1.6` in `$PATH`

## License
Licensed under the MIT License. See [LICENSE](./LICENSE) file.
