#!/bin/bash 
# Defines functions used by `mythos-install.sh` 

function create_dir() {
	read -r input <"${1:-/dev/stdin}"
	mkdir "$input" 
	echo "$input"
}

function copy_overwrite() {
	read -r dir <"/dev/stdin"
	cp "$1" "$dir"
	echo "$dir/$1"
}

function copy_no_overwrite() {
	read -r dir <"/dev/stdin"
	cp -n "$1" "$dir" 2|&1> /dev/null
	echo "$dir/$1"
}
