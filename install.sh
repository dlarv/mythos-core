#!/bin/bash 

function main() {
	# Use user's $HOME, not root's
	local user_home="$(getent passwd $SUDO_USER | cut -d: -f6)"

	# Ensure MYTHOS_DIR envvars have values
	for mythos_dir in ALIAS BIN CONFIG DATA LIB LOCAL_DATA LOCAL_CONFIG; do  
		local dir=$MYTHOS_{$mythos_dir,}_DIR
		if [ -z "$dir" ]; then 
			dir="$(grep "MYTHOS_"$mythos_dir"_DIR" "data/mythos-vars.sh" | sed -E 's/.*=//')"
			dir="${dir/\$HOME/$user_home}"
			dir="${dir//\"/}"
			echo "$dir"
		fi

		if [ ! -d "$dir" ]; then 
			echo "Created new directory: $dir"
			mkdir "$dir"
		fi
	done

	# Install charon 
	if [ -f "target/release/charon" ]; then 
		cp "target/release/charon" "charon"
	elif [ -f "target/debug/charon" ]; then 
		cp "target/debug/charon" "charon"
	else 
		local red="$(tput setaf 1)"
		local reset="$(tput sgr0)"
		printf "%s\n" "${red}Could not install Charon.${reset}"
		return 1
	fi
	./charon 
	rm charon

}

main "$@"
