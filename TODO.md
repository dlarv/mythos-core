- [ ] Charon uninstall should read implied file names
If reading a line with format `target_dir dest_dir [opts]` (Charon install file) and `dest_dir` has no filename, Charon should try using the filename at end of `target_dir` before throwing an error.
- [ ] Mythos error/warning/note printing to terminal. 
Error/etc messages given by mythos-utils have the same format. A module to mythos-core should be added that handles this. Format = "UTIL_NAME (Error/Warning/Note/''): error_msg"
