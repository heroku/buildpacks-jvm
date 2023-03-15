use indoc::formatdoc;
use libherokubuildpack::log::log_warning;

pub(crate) fn log_unused_maven_wrapper_warning(version: &str) {
    log_warning(
        "Unused Maven wrapper",
        formatdoc! {"
            Your application contains Maven wrapper, but a Maven version was also specified in system.properties.
            We will install that specified version ({version}) and ignore the Maven wrapper.
            We recommend that you use Maven wrapper instead of requesting a specific Maven version in system.properties.
        ", version = version },
    );
}

pub(crate) fn log_default_maven_version_warning(version: &str) {
    log_warning(
        "Using default version",
        formatdoc! {"
            Your application does not explicitly specify which Maven version should be used to build your application.
            The current default version {version} will be used. This default version will change from time to time.
            Depending on your build configuration, a different Maven version might cause your build to fail.

            We recommend that you use Maven wrapper to ensure that the same Maven version is always used to build your
            application. Alternatively, you can set 'maven.version' in 'system.properties' to a supported Maven version.
        ", version = version },
    );
}
