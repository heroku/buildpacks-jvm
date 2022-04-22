#!/usr/bin/env bash
set -euo pipefail
shopt -s dotglob

buildpack_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"

pushd "${buildpack_dir}"

cargo libcnb package --release

mkdir target
cp -r ../../target/buildpack/release/heroku_maven/* target/
cp package.toml target/

popd
