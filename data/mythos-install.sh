#!/bin/bash 
# Get MYTHOS_DIR vars 
user_home="$(getent passwd $SUDO_USER | cut -d: -f6)"
while read line; do 
	if [[ ! "$line" =~ ^(export MYTHOS_.*) ]]; then
		continue 
	fi

	IFS=' ' read -r key val <<< "$(sed -E 's/(export )|(=?")/ /g' <<< "$line")"

	case "$key" in 
		"MYTHOS_ALIAS_DIR")
			env_var="$MYTHOS_ALIAS_DIR"
		;;
		"MYTHOS_BIN_DIR")
			env_var="$MYTHOS_BIN_DIR"
		;;
		"MYTHOS_CONFIG_DIR")
			env_var="$MYTHOS_CONFIG_DIR"
		;;
		"MYTHOS_DATA_DIR")
			env_var="$MYTHOS_DATA_DIR"
		;;
		"MYTHOS_LOCAL_CONFIG_DIR")
			env_var="$MYTHOS_LOCAL_CONFIG_DIR"
		;;
		"MYTHOS_LOCAL_DATA_DIR")
			env_var="$MYTHOS_LOCAL_DATA_DIR"
		;;
		"MYTHOS_LIB_DIR")
			env_var="$MYTHOS_LIB_DIR"
		;;
		*) continue 
	esac
	
	if [ -z "$env_var" ]; then 
		echo "Setting $key"
		export "$key=${val/'$HOME'/$user_home}"
	else 
		echo "Found $key"
		# Expand $HOME vars if necessary 
		export "$key=${env_var/'$HOME'/$user_home}"
	fi
done <"mythos-vars.sh"

# Charon remembers every dir/file installed by mythos 
# Used to delete files that are no longer necessary
CHARON_DATA=".charon"
MYTHOS_CHARON_DIR="$MYTHOS_DATA_DIR/charon"
source "lib.sh"

# Create directory structure
echo "$MYTHOS_CONFIG_DIR" | create_dir >> "$CHARON_DATA"
echo "$MYTHOS_LOCAL_CONFIG_DIR" | create_dir >> "$CHARON_DATA"
echo "$MYTHOS_DATA_DIR" | create_dir >> "$CHARON_DATA"
echo "$MYTHOS_LOCAL_DATA_DIR" | create_dir >> "$CHARON_DATA"
echo "$MYTHOS_LIB_DIR" | create_dir >> "$CHARON_DATA"
echo "$MYTHOS_CHARON_DIR" | create_dir >> "$CHARON_DATA"

# Install core files
echo "$MYTHOS_ALIAS_DIR" | copy_overwrite "mythos-vars.sh" >> "$CHARON_DATA"
echo "$MYTHOS_BIN_DIR" | copy_overwrite "mythos-uninstall" >> "$CHARON_DATA"
echo "$MYTHOS_BIN_DIR" | copy_overwrite "print-mythos-dirs" >> "$CHARON_DATA"
echo "$MYTHOS_BIN_DIR" | copy_overwrite "charon" >> "$CHARON_DATA"

chmod +x "$MYTHOS_BIN_DIR/charon"

# Uninstall deprecated/etc files
items_to_remove=()
while read line; do 
	if grep -q "$line" < "$CHARON_DATA"; then 
		continue
	else	
		items_to_remove+=($line)
	fi
done < "$MYTHOS_CHARON_DIR/core"

if [ -n "${items_to_remove[*]}" ]; then 
	rm -ir "$items_to_remove"
fi

# Install new charon file
mv "$CHARON_DATA" "$MYTHOS_CHARON_DIR/core"
