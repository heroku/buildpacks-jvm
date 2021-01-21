#!/usr/bin/env bash
set -euo pipefail

buildpack_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
buildpack_toml_path="${buildpack_dir}/buildpack.toml"
target_dir_name="target"
target_dir="${buildpack_dir}/${target_dir_name}"

cnb_shim_tarball_url="https://github.com/heroku/cnb-shim/releases/download/v0.3/cnb-shim-v0.3.tgz"
cnb_shim_tarball_sha256="109cfc01953cb04e69c82eec1c45c7c800bd57d2fd0eef030c37d8fc37a1cb4d"
local_cnb_shim_tarball=$(mktemp)

v2_buildpack_tarball_url="$(yj -t <"${buildpack_toml_path}" | jq -r ".metadata.shim.tarball // empty")"
v2_buildpack_tarball_sha256="$(yj -t <"${buildpack_toml_path}" | jq -r ".metadata.shim.sha256 // empty")"
local_v2_buildpack_tarball=$(mktemp)

curl --retry 3 --location "${cnb_shim_tarball_url}" --output "${local_cnb_shim_tarball}"
curl --retry 3 --location "${v2_buildpack_tarball_url}" --output "${local_v2_buildpack_tarball}"

if ! echo "${cnb_shim_tarball_sha256} ${local_cnb_shim_tarball}" | sha256sum --check --status; then
	echo "Checksum verification of cnb_shim failed!"
	exit 1
fi

if ! echo "${v2_buildpack_tarball_sha256} ${local_v2_buildpack_tarball}" | sha256sum --check --status; then
	echo "Checksum verification of V2 buildpack tarball failed!"
	exit 1
fi

mkdir "${target_dir}"
rsync -a -L "${buildpack_dir}/" "${target_dir}" --exclude "${target_dir_name}"

mkdir -p "${target_dir}/target"
tar -xzmf "${local_cnb_shim_tarball}" -C "${target_dir}"
# The shim expects the V2 buildpack to be in the target directory. Don't get confused with all the nesting. :)
tar -xzmf "${local_v2_buildpack_tarball}" -C "${target_dir}/target"
