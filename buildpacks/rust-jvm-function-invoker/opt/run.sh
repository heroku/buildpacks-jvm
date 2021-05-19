#!/usr/bin/env bash

set -euo pipefail

runtime_layer_jar_path="${1}"
function_bundle_layer_dir="${2}"

additional_java_args=()
if [[ -n "${DEBUG_PORT:-""}" ]]; then
	java_version=$(java -version 2>&1 | grep -i version | awk '{gsub(/"/, "", $3); print $3}')

	if [[ "${java_version}" == 1.8* ]]; then
		additional_java_args+=("-agentlib:jdwp=transport=dt_socket,server=y,suspend=n,address=${DEBUG_PORT}")
	else
		additional_java_args+=("-agentlib:jdwp=transport=dt_socket,server=y,suspend=n,address=*:${DEBUG_PORT}")
	fi
fi

exec java "${additional_java_args[@]}" \
	-jar "${runtime_layer_jar_path}" serve "${function_bundle_layer_dir}" -h 0.0.0.0 -p "${PORT:-8080}"
