use crate::parser::*;
use std::{path::PathBuf, os::unix::prelude::PermissionsExt, fs};
use glob::glob;

// Structs
pub fn parse_target(target: &str, charon_path: &PathBuf) -> Result<(PathBuf, String), String> {
    let (root, name) = match target.rsplit_once("/") {
        Some(data) => data,
        None => ("", target)
    };
    let mut path = charon_path.clone();
    path.push(PathBuf::from(root));

    if !path.exists() {
        return Err(format!("{:?} could not be found", path));
    }

    if name.len() > 0 {
        path.push(name);
        if !name.contains("*") && !path.exists() {
            return Err(format!("{:?} could not be found", path));
        }
        path.pop();
    }
    return Ok((path, name.into()));
}
pub fn parse_dest(dest: &str) -> Result<PathBuf, String> {
    let (top_level, path) = match dest.trim().split_once("/") {
        Some(data) => data,
        None => (dest, "")
    };
    // Expand vars into MYTHOS_DIRS, etc
    let mut root = match expand_mythos_shortcut(top_level) {
        Some(path) => path,
        None => {
            let tmp = PathBuf::from(dest);
            if tmp.exists() {
                tmp 
            } else {
                return Err("Could not parse destination".into());
            }
        }
    };

    if path.len() > 0 {
        root.push(PathBuf::from(path));
    }
    return Ok(root);
}
pub fn parse_opts(opts: Option<&str>) -> Result<Opts, String> {
    let mut output = Opts {
        strip_ext: false,
        perms: 0,
        overwrite: false,
        copy_underscore_files: false,
        copy_dot_files: false,
    };

    if opts.is_none() {
        return Ok(output);
    }

    for opt in opts.unwrap().chars() {
        if opt.is_digit(8) {
            output.perms *= 8;
            output.perms += opt.to_digit(8).unwrap();
            continue;
        }
        match opt {
            'e' => output.strip_ext = true,
            'E' => output.strip_ext = false,
            'o' => output.overwrite = true,
            'O' => output.overwrite = false,
            '_' => output.copy_underscore_files = true,
            '.' => output.copy_dot_files = true,
            _ => return Err(format!("Unknown opt: '{opt}'"))
        }
    }
    return Ok(output);
}

impl InstallAction {
    pub fn execute(&self, dry_run: bool, old_files: &mut Vec<PathBuf>) -> Result<String, String> {
        return match self {
            InstallAction::File(file) => file.execute(dry_run, old_files),
            InstallAction::Dir(dir) => dir.execute(dry_run, old_files)
        };
    }
}
impl InstallFile {
    pub fn new(target_dir: PathBuf, target_name: &str, dest_dir: PathBuf, opts: Opts) -> InstallFile {
        return InstallFile {
            target_dir,
            target_name: target_name.to_string(), 
            dest_dir,
            opts
        };
    }
    pub fn get_targets(&self) -> Vec<PathBuf> {
        let target = self.target_dir.join(self.target_name.clone());
        
        if let Some(data) = target.to_str() {
            let target_glob = glob(data).expect(&format!("Could not parse target: '{:?}'", target));
            return target_glob.into_iter()
                .filter(|t| t.is_ok())
                .map(|t| { 
                    let mut path = PathBuf::from(t.unwrap());
                    if self.opts.strip_ext {
                        path.set_extension("");
                    }
                    return path;
                }).filter(|t| {
                    let name = match t.file_name() {
                        Some(name) => name,
                        None => return false
                    };
                    return (self.opts.copy_underscore_files || !name.to_string_lossy().starts_with("_"))
                        && (self.opts.copy_dot_files || !name.to_string_lossy().starts_with("."));
                }).collect::<Vec<PathBuf>>();
        }
        panic!("Could not find target file: {:?}", target);
    }

    pub fn get_opts(&self) -> Opts {
        return self.opts;
    }
    pub fn get_dest(&self) -> PathBuf {
        return self.dest_dir.clone();
    }
    pub fn execute(&self, dry_run: bool, old_files: &mut Vec<PathBuf>) -> Result<String, String> {
        let mut log: Vec<String> = Vec::new();
        for target in self.get_targets() {
            // Copy 
            let filename = target.file_name().unwrap();
            let dest = self.dest_dir.join(filename);
            log.push(format!("{:?}\n\t# ", dest));

            let mut msg = String::new();
            old_files.retain(|file| { 
                *file != dest 
            });
            if self.dest_dir.join(filename).exists() {
                if !self.opts.overwrite {
                    log.push("Did not copy: File exists && !overwrite\n".into());
                    continue;
                }
                msg += "File was overwritten.";
            }

            if !dry_run {
                target.metadata().unwrap().permissions().set_mode(self.opts.perms);
                if let Err(err) = std::fs::copy(&target, dest) {
                    log.push(format!("Did not copy. Error: {}\n", err.to_string()));
                    continue;
                }
            }
            log.push(format!("Copied! {}\t{:o}\n", msg, self.opts.perms));
        }
        return Ok(log.join("").replace("\"", ""));
    }
}
impl InstallDir {
    pub fn execute(&self, dry_run: bool, old_files: &mut Vec<PathBuf>) -> Result<String, String> {
        let mut msg = format!("{:?}\n\t# ", self.dir);

        old_files.retain(|file| { 
            *file != self.dir
        });

        if self.dir.exists() && self.dir.is_dir() {
            msg += "Did not create: Directory exists.";
        }
        else if self.dir.is_file() {
            msg += "Did not create: File exists with that name.";
        }
        else if !dry_run {
            if let Err(err) = std::fs::create_dir(self.dir.clone()) {
                return Err(format!("CHARON (Error): Could not create dir {:?}. Error: {}", self.dir, err.to_string()));
            }
            msg += "Created directory!";
        }
        msg += "\n";
        return Ok(msg.replace("\"", "")); 
    }
}
