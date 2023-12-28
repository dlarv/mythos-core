#!/bin/bash 
function execute() {
	RESULT="$(source data/mythos-dirs.sh -v "$@")"
	echo "$RESULT"
}
function run_regex_tests() {
	local valid
	valid="$1"
	shift
	while [ $# -gt 0 ]; do
		if [ "$(execute "$1")" != "$valid" ]; then
			echo -en "$FG_RED"'Failed'"$COLOR_RESET"
			echo -e "\tArg = $1\tResult = $RESULT"
			return
		fi
		shift
	done
	echo -e "$FG_GREEN"'Passed'"$COLOR_RESET"
}
function test_regex() {
	echo "TESTING REGEX PATTERNS"
	dir="$MYTHOS_ALIAS_DIR"
	echo -n "Testing alias dir: $dir........" 
	run_regex_tests "$dir" 'a' 'alias' 'aliases'

	dir="$MYTHOS_BIN_DIR"
	echo -n "Testing bin dir: $dir........" 
	run_regex_tests "$dir" 'b' 'bin' 

	dir="$MYTHOS_LIB_DIR"
	echo -n "Testing lib dir: $dir........" 
	run_regex_tests "$dir" 'l' 'lib' 'library' 

	dir="$MYTHOS_LOCAL_CONFIG_DIR"
	echo -n "Testing local config dir: $dir........" 
	run_regex_tests "$dir" 'lc' 'lconf' 'lconfig' 'localc' 'localconf' 'localconfig' 'l_c' 'l_conf' 'l_config' 'local_c' 'local_conf' 'local_config'

	dir="$MYTHOS_LOCAL_DATA_DIR"
	echo -n "Testing local data dir: $dir........" 
	run_regex_tests "$dir" 'ld' 'ldata' 'locald' 'localdata' 'l_d' 'l_data' 'local_d' 'local_data' 

	dir="$MYTHOS_CONFIG_DIR"
	echo -n "Testing config dir: $dir........" 
	run_regex_tests "$dir" 'C' 'CONF' 'CONFIG' 'Conf'

	dir="$MYTHOS_DATA_DIR"
	echo -n "Testing data dir: $dir........" 
	run_regex_tests "$dir" 'D' 'DATA' 'Data'
}
function test_subdir() {
	echo "TESTING SUBDIR CATCHING"
	echo -n "Checking for exising subdir......."
	execute "config" "arachne" >/dev/null
	if [ "$RESULT" == "$MYTHOS_LOCAL_CONFIG_DIR/arachne" ]; then
		echo -e "$FG_GREEN"'Passed'"$COLOR_RESET"
	else 
		echo -en "$FG_RED"'Failed'"$COLOR_RESET"
		echo -e "\t$MYTHOS_LOCAL_CONFIG_DIR/arachne\tResult = $RESULT"
	fi

	echo -n "Checking for non-exising subdir......."
	execute "config" "thisdirdne" >/dev/null
	if [ "$RESULT" == "$MYTHOS_LOCAL_CONFIG_DIR" ]; then
		echo -e "$FG_GREEN"'Passed'"$COLOR_RESET"
	else 
		echo -en "$FG_RED"'Failed'"$COLOR_RESET"
		echo -e "\t$MYTHOS_LOCAL_CONFIG_DIR\tResult = $RESULT"
	fi

	echo -n "Checking for global dir......."
	execute "data" "charon" >/dev/null
	if [ "$RESULT" == "$MYTHOS_DATA_DIR/charon" ]; then
		echo -e "$FG_GREEN"'Passed'"$COLOR_RESET"
	else 
		echo -en "$FG_RED"'Failed'"$COLOR_RESET"
		echo -e "\t$MYTHOS_DATA_DIR/charon\tResult = $RESULT"
	fi

}

test_regex
test_subdir
