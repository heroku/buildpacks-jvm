#!/usr/bin/env bash

export DEFAULT_MAVEN_VERSION="3.6.2"
export BUILDPACK_STDLIB_URL="https://lang-common.s3.amazonaws.com/buildpack-stdlib/v7/stdlib.sh"

install_maven() {
	local installDir=$1
	local buildDir=$2
	mavenHome=$installDir/.maven

	definedMavenVersion=$(detect_maven_version "$buildDir")

	mavenVersion=${definedMavenVersion:-$DEFAULT_MAVEN_VERSION}
	mcount "mvn.version.${mavenVersion}"

	status_pending "Installing Maven ${mavenVersion}"
	local mavenUrl="https://lang-jvm.s3.amazonaws.com/maven-${mavenVersion}.tar.gz"
	if is_supported_maven_version "${mavenVersion}" "${mavenUrl}"; then
		download_maven "${mavenUrl}" "${installDir}" "${mavenHome}"
		status_done
	else
		error_return "Error, you have defined an unsupported Maven version in the system.properties file.
The default supported version is ${DEFAULT_MAVEN_VERSION}"
		return 1
	fi
}

download_maven() {
	local mavenUrl=$1
	local installDir=$2
	local mavenHome=$3
	rm -rf "$mavenHome"
	curl --retry 3 --silent --max-time 60 --location "${mavenUrl}" | tar xzm -C "$installDir"
	chmod +x "$mavenHome/bin/mvn"
}

is_supported_maven_version() {
	local mavenVersion=${1}
	local mavenUrl=${2:?}
	if [ "$mavenVersion" = "$DEFAULT_MAVEN_VERSION" ]; then
		return 0
	elif curl -I --retry 3 --fail --silent --max-time 5 --location "${mavenUrl}" >/dev/null; then
		return 0
	else
		return 1
	fi
}

detect_maven_version() {
	local baseDir=${1}
	if [ -f "${baseDir}/system.properties" ]; then
		mavenVersion=$(get_app_system_value "${baseDir}/system.properties" "maven.version")
		if [ -n "$mavenVersion" ]; then
			echo "$mavenVersion"
		else
			echo ""
		fi
	else
		echo ""
	fi
}

get_app_system_value() {
	local file=${1?"No file specified"}
	local key=${2?"No key specified"}

	# escape for regex
	local escaped_key
	# shellcheck disable=SC2001
	escaped_key="$(echo "$key" | sed "s/\./\\\./g")"

	[ -f "$file" ] &&
		grep -E "^${escaped_key}[[:space:]=]+" "$file" |
		sed -E -e "s/$escaped_key([\ \t]*=[\ \t]*|[\ \t]+)([A-Za-z0-9\.-]*).*/\2/g"
}

cache_copy() {
	rel_dir=$1
	from_dir=$2
	to_dir=$3
	rm -rf "${to_dir:?}/${rel_dir:?}"
	if [ -d "$from_dir/$rel_dir" ]; then
		mkdir -p "$to_dir/$rel_dir"
		cp -pr "$from_dir/$rel_dir"/. "$to_dir/$rel_dir"
	fi
}