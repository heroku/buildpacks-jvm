#!/usr/bin/env bash
set -euo pipefail

buildpack_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"

pushd "${buildpack_dir}"

cargo libcnb package --release

mv target target-cargo
mkdir target
cp -r target-cargo/buildpack/release/heroku_jvm/* target/
cp package.toml target/

popd
