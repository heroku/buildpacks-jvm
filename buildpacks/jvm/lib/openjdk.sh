#!/usr/bin/env bash

function openjdk:get_configured_version() {
	local -r app_directory="${1:?}"
	local -r default_version="${2}"
	local -r system_properties_path="${app_directory}/system.properties"

	local selected_version=""
	if [[ -f "${system_properties_path}" ]]; then
		selected_version=$(bputils::get_java_properties_value "java.runtime.version" <"${system_properties_path}")
	fi

	echo "${selected_version:-$default_version}"
}

function openjdk::resolve_selector() {
	local -r selector="${1:?}"
	local -r file="${CNB_BUILDPACK_DIR}/${CNB_STACK_ID}.toml"

	if [[ "${selector}" =~ ^(([^-]+)-)?(.+)$ ]]; then
		local -r distribution_selector="${BASH_REMATCH[2]:-"heroku"}"
		local -r version_selector="${BASH_REMATCH[3]}"
	fi

	version=$(yj -t <"${file}" | jq -r ".aliases.versions.\"${version_selector}\" // \"${version_selector}\"")
	distribution=$(yj -t <"${file}" | jq -r ".aliases.vendors.\"${distribution_selector}\" // \"${distribution_selector}\"")

	json=$(yj -t <"${file}" | jq -r ".\"${distribution}\".\"${version}\".versions[0]")
	if [[ "${json}" != "null" ]]; then
		echo "${json}"
		return 0
	else
		json=$(yj -t <"${file}" | jq -r "[.\"${distribution}\"[].versions[]] | map(select(.version == \"${version}\"))[0]")
		if [[ "${json}" != "null" ]]; then
			echo "${json}"
			return 0
		fi
	fi

	echo "null"
}
