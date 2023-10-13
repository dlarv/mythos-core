# Mythos-Core
## What is Mythos?
Mythos is a collection of command line utils, intended to fulfill a range of different functions. The reason they are collected like this is really just organization. Instead of having `~/.config/util_i_wrote_1`, `~/.config/util_i_wrote_2`, etc, mythos-utils will be grouped under a shared `~/.config/mythos` directory. 

The individual utils are mostly projects geared toward learning and filling specific (potentially niche) needs I have. These range from wrappers for commands (plutonian-shores) to system navigation and management (arachne). 

## What is Mythos-Core?
Mythos-Core is a library shared between different mythos-utils. As of now, it only contains `dirs`, which gives utils the mythos directories (like the aforementioned `~/.config/mythos`). If I find other functionality repeating itself across the project, I will add them to this repo.
