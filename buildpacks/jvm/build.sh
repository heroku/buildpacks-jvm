#!/usr/bin/env bash
set -euo pipefail

# Copies the whole buildpack to the target directory while following symlinks.
# Resolving symlinks to regular files is the main purpose of this "build" script.

buildpack_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
target_dir_name="target"
target_dir="${buildpack_dir}/${target_dir_name}"

mkdir "${target_dir}"
rsync -a -L "${buildpack_dir}/" "${target_dir}" --exclude "${target_dir_name}"

dependencies_dir="${target_dir}/dependencies"
mkdir -p "${dependencies_dir}"

dependency_count=$(yj -t <"${buildpack_dir}/buildpack.toml" | jq -r ".metadata.build.dependencies | length")
for ((index = 0; index < "${dependency_count}"; index++)); do
	name="$(yj -t <"${buildpack_dir}/buildpack.toml" | jq -r ".metadata.build.dependencies[${index}].name")"
	uri="$(yj -t <"${buildpack_dir}/buildpack.toml" | jq -r ".metadata.build.dependencies[${index}].uri")"
	sha256="$(yj -t <"${buildpack_dir}/buildpack.toml" | jq -r ".metadata.build.dependencies[${index}].sha256")"
	executable="$(yj -t <"${buildpack_dir}/buildpack.toml" | jq -r ".metadata.build.dependencies[${index}].executable // false")"

	wget "${uri}" -O "${dependencies_dir}/${name}"

	if ! echo "${sha256} ${dependencies_dir}/${name}" | sha256sum --check --status; then
		echo "Checksum verification of ${name} failed!"
		exit 1
	fi

	if [[ "${executable}" == "true" ]]; then
		chmod +x "${dependencies_dir}/${name}"
	fi
done
