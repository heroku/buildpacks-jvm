#!/usr/bin/env bash

log::cnb::header() {
	echo
	echo -e "\033[1;35m[$*]\033[0m"
}

log::cnb::error() {
	echo
	echo 1>&2 -e "\033[1;31m[ERROR: ${1:?}]\033[0m"
	echo 1>&2 -e "\033[31m$(cat -)\033[0m"
	echo
}

log::cnb::warning() {
	echo
	echo -e "\033[1;33m[WARNING: ${1:?}]\033[0m"
	echo -e "\033[33m$(cat -)\033[0m"
	echo
}

log::cnb::debug() {
	if [[ -n ${HEROKU_BUILDPACK_DEBUG:-} ]]; then
		echo "[DEBUG] $*"
	fi
}

log::cnb::info() {
	echo "[INFO] $*"
}
