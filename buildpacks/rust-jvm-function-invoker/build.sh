#!/usr/bin/env bash
set -euo pipefail

buildpack_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
pushd "$buildpack_dir"
cargo make pack --profile "$CARGO_MAKE_PROFILE"
popd
