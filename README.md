# Mythos-Core
## What is Mythos?
Mythos is a collection of command line utils, intended to fulfill a range of different functions. The reason they are collected like this is really just organization. Instead of having `~/.config/util_i_wrote_1`, `~/.config/util_i_wrote_2`, etc, mythos-utils will be grouped under a shared `~/.config/mythos` directory. 

The individual utils are mostly projects geared toward learning and filling specific (potentially niche) needs I have. These range from wrappers for commands (plutonian-shores) to system navigation and management (arachne). 

## What is Mythos-Core?
Mythos-Core is a library shared between different mythos-utils. 

### Modules 
- cli: Provides functions used to parse command line args. This is mostly `clean_cli_args`, which turns args into an easier to read format.
- conf: Provides functionality for reading values from config files.
- dirs: Provides utils with mythos directories.

## Charon
Charon is a utility to assist with installing mythos-utils from their source code. It saves a list of files/directories which were created into a `charon` file. This file can then be used to remove deprecated files and uninstall utilities. 

Charon looks for a file with the extension `.charon`, which contains a list of local files and their destinations.

### Charon File Syntax
#### Install 
@ $DIR_CODES_TO_CREATE
[target] [destination] [opts]

[target]
- relative/path/to/file 
- rel/path/to/dir/*

[destination]
- /abs/path/to/directory 
- $HOME/path/to/dir
- ~/path/to/dir
- MYTHOS_DIR/path/to/dir, where MYTHOS_DIR is one of the following:
    - A, ALIAS
    - B, BIN
    - C, CONFIG 
    - D, DATA,
    - LB, LIB,
    - LC, LCONFIG, LOCALCONFIG
    - LD, LDATA, LOCALDATA
NOTE: In the final option, if the path DNE, it will be created.

[opts]
- `e` Strip extension
- `E` Don't strip extension
- `o` Overwrite if exists
- `O` No overwrite 
- `_` Copy files that start with an underscore
- `.` Copy dotfiles
- `###` Where `#` is an octal digit, representing file permissions
NOTE: opts can be in any order, however `###` must be contiguous.
Opts are 'false' by default.

#### Uninstall
Charon can check the .charon file output from the last install command and remove any files that weren't included in the current install.
e.g. 
    In version 1.0 a file named `file1` was included.
    In version 1.1, this file is no longer needed. It is excluded from the charon install file.
    Charon can check the compare outputs of v1.0 and v1.1. 
    It will see that `file1` is now 'orphaned' and will remove it.

Charon can read the install syntax as an uninstall file. Alternatively, the absolute paths for each file installed are listed, separated by newlines. 
This file is saved at $MYTHOS_DATA_DIR/charon/$UTIL_NAME.charon. It is not recommended to alter the uninstall file. 
