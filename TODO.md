v1.0.0
- [x] clean_cli_args returns iterator, not vec.
- [x] Logger.printinfo(args*, bool), where bool decides whether info is printed
- [x] Logger saves data to file.
~v1.1.0~ v2.0.0 (A method was renamed, making this not backwards compatible).
- [x] Conf can act as an abstraction over a directory, not just a file. I.e. if given a dir, conf will treat it as a config file containing a dict of lists(subdirs) and dicts(files).
