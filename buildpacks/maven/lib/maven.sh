function maven::default_version() {
	echo "3.6.2"
}

function maven::tarball_url_for_version() {
	local -r maven_version="${1:?}"

	declare -A maven_tarball_urls
	maven_tarball_urls[3.6.2]="https://lang-jvm.s3.amazonaws.com/maven-3.6.2.tar.gz"
	maven_tarball_urls[3.5.4]="https://lang-jvm.s3.amazonaws.com/maven-3.5.4.tar.gz"
	maven_tarball_urls[3.3.9]="https://lang-jvm.s3.amazonaws.com/maven-3.3.9.tar.gz"
	maven_tarball_urls[3.2.5]="https://lang-jvm.s3.amazonaws.com/maven-3.2.5.tar.gz"

	echo "${maven_tarball_urls["${maven_version}"]:-}"
}

function maven::get_configured_version() {
	local -r app_directory="${1:?}"
	local -r default_version="${2}"
	local -r system_properties_path="${app_directory}/system.properties"

	local selected_version=""
	if [[ -f "${system_properties_path}" ]]; then
		selected_version=$(bputils::get_java_properties_value "maven.version" <"${system_properties_path}")
	fi

	echo "${selected_version:-$default_version}"
}

function maven::is_version_configured() {
	local -r app_directory="${1:?}"

	[[ $(maven::get_configured_version "${app_directory}" "") != "" ]]
}

function maven::app_contains_wrapper() {
	local -r app_directory="${1:?}"

	[[ -f "${app_directory}/mvnw" && -f "${app_directory}/.mvn/wrapper/maven-wrapper.properties" ]]
}

function maven::should_use_wrapper_for_app() {
	local -r app_directory="${1:?}"

	maven::app_contains_wrapper "${app_directory}" && ! maven::is_version_configured "${app_directory}"
}
