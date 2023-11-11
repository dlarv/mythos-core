pub mod install;
use std::path::PathBuf;
use mythos_core::dirs;

#[derive(Debug)]
pub enum InstallAction {
    File(InstallFile),
    Dir(InstallDir)
}
#[derive(Debug)]
pub struct InstallFile {
    target_dir: PathBuf,
    target_name: String,
    dest_dir: PathBuf,
    opts: Opts, 
}
#[derive(Debug)]
pub struct InstallDir {
    dir: PathBuf,
    msg: String,
}
#[derive(Debug, Copy, Clone)]
pub struct Opts {
    pub strip_ext: bool,
    pub copy_underscore_files: bool,
    pub copy_dot_files: bool,
    pub perms: u32,
    pub overwrite: bool,
}
// TODO: use util_name
pub fn expand_mythos_shortcut(shortcut: &str) -> Option<PathBuf> {
    return match shortcut.trim_start_matches("$"){
        "A" | "ALIAS" => dirs::get_dir(dirs::MythosDir::Alias, ""),
        "B" | "BIN" => dirs::get_dir(dirs::MythosDir::Bin, ""),
        "C" | "CONFIG" => dirs::get_dir(dirs::MythosDir::Config, ""),
        "D" | "DATA" => dirs::get_dir(dirs::MythosDir::Data, ""),
        "LB" | "LIB" => dirs::get_dir(dirs::MythosDir::Lib, ""),
        "LC" | "LCONFIG" | "LOCALCONFIG" => dirs::get_dir(dirs::MythosDir::LocalConfig, ""),
        "LD" | "LDATA" | "LOCALDATA" => dirs::get_dir(dirs::MythosDir::LocalData, ""),
        "HOME" | "~" => dirs::get_home(),
        _ => None
    }
}
pub fn create_util_dir(root: PathBuf, util_name: &str, dry_run: bool) -> Result<InstallDir, String> {
    let path = root.join(util_name);
    let mut msg = format!("{:?}\n\t# ", path);

    if path.exists() && path.is_dir() {
        msg += "Did not create: Directory exists.";
    }
    else if path.is_file() {
        msg += "Did not create: File exists with that name.";
    }
    else {
        if !dry_run {
            if let Err(err) = std::fs::create_dir(path.clone()) {
                return Err(format!("CHARON (Error): Could not create dir {:?}. Error: {}", path, err.to_string()));
            }
        }
        msg += "Created directory!";
    }
    msg += "\n";
    msg = msg.replace("\"", ""); 
    return Ok(InstallDir { dir: path, msg }); 
}
// `@ dirs` statements parsed in this function b/c otherwise dest tokens will not expand properly.
pub fn parse_install_file(contents: &mut String, path: PathBuf, util_name: &str, dry_run: bool) -> Vec<InstallAction> {
    /*!
     * Turn .charon file into list of actions 
     * Format: 
     * target_dir dest_dir opts 
     * target_dir dest_dir
     */
    let mut actions: Vec<InstallAction> = Vec::new();

    for (count, line) in contents.split("\n").enumerate() {
        let err_msg = format!("CHARON (Fatal Error on line {}):",count + 1);
        let mut tokens = line.trim().split(" ");
        let target = tokens.next().expect(&format!("{err_msg} Expected a path to source code file."));

        // Line contains dirs to install 
        if target.starts_with("@") {
            for dir in tokens {
                let res = match expand_mythos_shortcut(dir) {
                    Some(path) => path,
                    None => panic!("{err_msg} Could not read dir to create")
                };
                match create_util_dir(res, util_name, dry_run) {
                    Ok(path) => actions.push(InstallAction::Dir(path)),
                    Err(msg) => panic!("{err_msg} {msg}")
                }
            }
            continue;
        }

        // Line is a comment, empty, or uninstall command
        if target.starts_with("#") || target.len() <= 1 { continue; }


        let (target_dir, target_name) = match install::parse_target(target, &path) {
            Ok(data) => data,
            Err(msg) => panic!("{err_msg} {msg}")
        };

        // Parse destination file
        let dest = tokens.next().expect(&format!("{err_msg} Expected a path to destination directory."));
        let dest_dir = match install::parse_dest(dest) {
            Ok(data) => data,
            Err(msg) => panic!("{err_msg} {msg}")
        };

        let opts = match install::parse_opts(tokens.next()) {
            Ok(opts) => opts,
            Err(msg) => panic!("{err_msg} {msg}")
        };

        actions.push(InstallAction::File(InstallFile::new(target_dir, &target_name, dest_dir, opts)));
    }
    return actions;
}
pub fn parse_uninstall_file(contents: &mut String) -> Vec<PathBuf> {
    /*!
     * Parses charon install file into list of paths to remove.
     * Can read install charon files.
     * Format:
     * rm_dir
     * ignore rm_dir 
     * ignore rm_dir ignore 
     */
    let mut targets: Vec<PathBuf> = Vec::new();
    for line in contents.split("\n") {
        let tokens: Vec<&str> = line.trim().split(" ").collect();

        if tokens.len() == 0 || tokens[0].starts_with("#") || tokens[0].len() == 0 { continue; }

        let path = PathBuf::from(match tokens.len() {
            1 => tokens[0],
            2 | 3 => tokens[1],
            _ => {
                eprintln!("CHARON (Error): Expected 1-3 items per line inside charon uninstall file, found {}. Skipping...", tokens.len());
                continue
            }
        });

        targets.push(path);
    }
    return targets;
}

