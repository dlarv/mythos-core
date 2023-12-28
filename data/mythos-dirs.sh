#!/bin/bash
function print_err() {
	>&2 echo "$*"
}

function main() {
	local do_mkdir be_verbose dir_name util_name
	while [ "${1:0:1}" == '-' ]; do 
		if [[ "$1" =~ ^-(m|-m(kdir|ake)) ]]; then
			do_mkdir=1
		elif [[ "$1" =~ ^-(v|-verbose) ]]; then
			be_verbose=1
		elif [[ "$1" =~ ^-(vm)|(mv)$ ]]; then
			do_mkdir=1
			be_verbose=1
		fi
		shift
	done
	[[ -n "$be_verbose" ]] && [[ -n "$do_mkdir" ]] && print_err "Will create dir if dne"

	if [ -z "$1" ]; then 
		[[ -n "$be_verbose" ]] && print_err "Command must be given a dir_name arg"
		return 1
	fi
	dir_name="$1"
	util_name="$2"

	local paths
	if [[ "$dir_name" =~ ^a(lias(es)?)?$ ]]; then
		paths=("$MYTHOS_ALIAS_DIR")
	elif [[ "$dir_name" =~ ^b(in)?$ ]]; then
		paths=("$MYTHOS_BIN_DIR")
	elif [[ "$dir_name" =~ ^l(ib(rary)?)?$ ]]; then
		paths=("$MYTHOS_LIB_DIR")
	# Use both local and global dirs	
	elif [[ -z "$do_mkdir" ]] && [[ "$dir_name" =~ ^c(onf(ig)?)? ]]; then
		paths=("$MYTHOS_LOCAL_CONFIG_DIR" "$MYTHOS_CONFIG_DIR")
	elif [[ -z "$do_mkdir" ]] && [[ "$dir_name" =~ ^d(ata)? ]]; then
			paths=("$MYTHOS_LOCAL_DATA_DIR" "$MYTHOS_DATA_DIR")
	# Force the use of either local or global dir, but not both
	else 
		# Ignore case
		shopt -s nocasematch
		if [[ "$dir_name" =~ ^c(onf(ig)?)? ]]; then
			paths=("$MYTHOS_CONFIG_DIR")
		elif [[ "$dir_name" =~ ^d(ata)?$ ]]; then
			paths=("$MYTHOS_DATA_DIR")
		elif [[ "$dir_name" =~ ^l(ocal)?_?c(onf(ig)?)? ]]; then
			paths=("$MYTHOS_LOCAL_CONFIG_DIR")
		elif [[ "$dir_name" =~ ^l(ocal)?_?d(ata)? ]]; then
			paths=("$MYTHOS_LOCAL_DATA_DIR")
		else 
			[[ -n "$be_verbose" ]] && print_err "Could not find dir: $dir_name"
			return 1
		fi
		shopt -u nocasematch
	fi

	local path
	for subdir in "$util_name" ""; do 
		for dir in "${paths[@]}"; do
			path="$dir/$subdir"
			if [[ -n "$do_mkdir" ]] && [ ! -e "$path" ]; then
				[[ -n "$be_verbose" ]] && print_err "Creating directory: $path"
				mkdir -p "$path" || return 2
			fi
			if [ -e "$path" ]; then
				echo "${path%%/}"
				return 0
			fi
		done
	done
	[[ -n "$be_verbose" ]] && print_err "Directory not found"
	return 1
}

if [[ "$1" =~ ^-(h|-help) ]]; then
	echo "mythos-dirs [opts] {dir_name} [util]"
	echo "Get reference to mythos directories"
	echo "Will try and echo a valid path, in the following order:"
	echo "- \$local_dir/\$util"
	echo "- \$global_dir/\$util"
	echo "- \$local_dir/"
	echo "- \$global_dir/"
	echo "NOTE: \$global_dir only applies if dir_name is either 'data' or 'config'. It is ignored for all other values."
	echo "Opts:"
	echo -e "-h | --help\t\tPrint this menu."
	echo -e "-m | --make | --mkdir\t\tmkdir if it dne."
	echo -e "-v | --verbose\t\tLog actions to stderr."
	echo -e "-l | --list\t\tPrint the list of MYTHOS_DIRS."
elif [[ "$1" =~ ^-(l|-list) ]]; then
	echo "Complete list of \$MYTHOS_DIRS"
	echo "Alias:        $MYTHOS_ALIAS_DIR"
	echo "Bin:          $MYTHOS_BIN_DIR"
	echo "Config:       $MYTHOS_CONFIG_DIR"
	echo "Data:         $MYTHOS_DATA_DIR"
	echo "Lib:          $MYTHOS_LIB_DIR"
	echo "Local Config: $MYTHOS_LOCAL_CONFIG_DIR"
	echo "Local Data:   $MYTHOS_LOCAL_DATA_DIR"
else 
	main "$@"
fi
