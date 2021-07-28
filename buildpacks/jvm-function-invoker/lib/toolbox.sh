#!/usr/bin/env bash

install_toolbox() {
	local toolbox_dir=$1
	local jqUrl="https://github.com/stedolan/jq/releases/download/jq-1.5/jq-linux64"
	local jqSha="c6b3a7d7d3e7b70c6f51b706a3b90bd01833846c54d32ca32f0027f00226ff6d"
	local yjUrl="https://github.com/sclevine/yj/releases/download/v2.0/yj-linux"
	local yjSha="db2b94b7fbf0941b6af9d30c1e7d43e41be62edad59d711b5c760ad5b13f7d6c"

	mkdir -p "${toolbox_dir}/bin"

	if [[ ! -f "${toolbox_dir}/bin/jq" ]]; then
		local jqBin="${toolbox_dir}/bin/jq"
		curl -o "$jqBin" -Ls "$jqUrl" && chmod +x "$jqBin"

		local actualSha
		actualSha="$(shasum -a 256 "${jqBin}" | awk '{ print $1 }')"

		if [ "$actualSha" != "$jqSha" ]; then
			echo "Invalid jq sha: $actualSha"
			exit 1
		fi
	fi

	if [[ ! -f "${toolbox_dir}/bin/yj" ]]; then
		local yjBin="${toolbox_dir}/bin/yj"
		curl -o "$yjBin" -Ls "$yjUrl" && chmod +x "$yjBin"

		local actualSha
		actualSha="$(shasum -a 256 "$yjBin" | awk '{ print $1 }')"

		if [ "$actualSha" != "$yjSha" ]; then
			echo "Invalid yj sha: $actualSha"
			exit 1
		fi
	fi
}
