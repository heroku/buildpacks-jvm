log::cnb::error() {
	# Send all of our output to stderr
	exec 1>&2

	# If arguments are given, redirect them to stdin this allows the function to be invoked with a string argument,
	# or with stdin, e.g. via <<-EOF
	(($#)) && exec <<<"$@"

	echo
	echo -n " !     ERROR: $(cat -)" | log::cnb::indent no_first_line_indent
	echo
}

log::cnb::warning() {
	# Send all of our output to stderr
	exec 1>&2

	# If arguments are given, redirect them to stdin this allows the function to be invoked with a string argument,
	# or with stdin, e.g. via <<-EOF
	(($#)) && exec <<<"$@"

	echo
	echo " !     WARNING: $(cat -)" | log::cnb::indent no_first_line_indent
	echo
}

log::cnb::warning_inline() {
	echo " !     WARNING: $*" | log::cnb::indent no_first_line_indent
}

log::cnb::status() {
	echo "-----> $*"
}

log::cnb::status_pending() {
	echo -n "-----> $*..."
}

log::cnb::status_done() {
	echo " done"
}

log::cnb::notice() {
	echo
	echo "NOTICE: $(cat -)" | log::cnb::indent
	echo
}

log::cnb::notice_inline() {
	echo "NOTICE: $*" | log::cnb::indent
}

log::cnb::indent() {
	# sed -l basically makes sed replace and buffer through stdin to stdout so you get updates while the command runs
	# and dont wait for the end e.g. npm install | log::cnb::indent

	# If any value (e.g. a non-empty string, or true, or false) is given for the first argument, this will act as a flag
	# indicating we shouldn't indent the first line; we use :+ to tell SED accordingly if that parameter is set,
	# otherwise null string for no range selector prefix (it selects from line 2 onwards and then every 1st line,
	# meaning all lines)
	#
	# If the first argument is an empty string, it's the same as no argument (useful if a second argument is passed)
	# the second argument is the prefix to use for indenting; defaults to seven space characters, but can be set to
	# e.g. " !     " to decorate each line of an error message
	local c="${1:+"2,999"} s/^/${2-"       "}/"

	case $(uname) in
	Darwin) sed -l "$c" ;; # mac/bsd sed: -l buffers on line boundaries
	*) sed -u "$c" ;; # unix/gnu sed: -u unbuffered (arbitrary) chunks of data
	esac
}
