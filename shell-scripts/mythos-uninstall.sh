#!/bin/bash 

function _get_charon_data() {
	CHARON_DATA="$MYTHOS_DATA_DIR/charon/$1"
	if [ ! -e "$CHARON_DATA" ]; then 
		echo "Directory '$CHARON_DATA' not found. Please ensure mythos has been properly installed and the env var 'MYTHOS_DATA_DIR' is defined" 
		return 1
	fi
}

function list_utils() {
	_get_charon_data || return 1
	echo "Util Charon Files:"
	ls "$CHARON_DATA"
}

function dry_run() {
	_get_charon_data "$1" || return 1
	echo "Uninstalling '$1'"
	while read -r item; do 
		echo "REMOVING: $item"
	done < "$CHARON_DATA"
}
function move_util() {
	_get_charon_data "$1" || return 1
	echo "Uninstalling '$1'"
	local trash_dir="$CHARON_DATA/.TRASH"
	echo "Trash directory: $trash_dir"

	while read -r item; do 
		mv "$item" "$trash_dir"
	done < "$CHARON_DATA"
}
function remove_util() {
	_get_charon_data "$1" || return 1
	echo "Uninstalling '$1'"

	readarray -t items < "$CHARON_DATA"
	for item in "${items[@]}"; do
		rm -ri "$item"
	done
}

if [[ "$1" =~ -(l|-list) ]]; then 
	list_utils || return 1
elif [[ "$1" =~ -(n|-dry-run) ]]; then 
	for util in "${@:2}"; do 
		dry_run "$util"  || return 1 
	done
elif [[ "$1" =~ -(m|-move) ]]; then 
	for util in "${@:2}"; do 
		move_util "$util"  || return 1 
	done
elif [ $# -eq 1 ]; then 
	echo "$util"
	for util in "$@"; do 
		remove_util "$util"  || return 1 
	done
else
	echo "mythos-uninstall [opts] [utils]"
	echo "Remove all files and directories installed by [utils]"
	echo "OPTS"
	echo "-h|--help			Print this menu"
	echo "-l|--list			List valid [utils] args"
	echo "-n|--dry-run		List items that would be removed without actually removing them"
	echo "-m|--move	{path}	Move items to {path} without removing them"
fi

