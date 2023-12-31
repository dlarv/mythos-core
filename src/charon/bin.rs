pub mod parser;
use parser::*;
use mythos_core::{dirs, logger::*, fatalmsg};
use mythos_core::{printinfo,printwarn,printerror,printfatal};

use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::path::PathBuf;

pub const UTIL_ID: &str = "CHARON";

fn main() {
    set_logger_id(UTIL_ID);
    let mut do_dry_run = false;
    let mut be_quiet = false;
    let mut do_remove_orphans = true;
    let mut path_arg = String::new();

    let args = mythos_core::cli::clean_cli_args();
    for arg in args {
        match arg.as_str() {
            "-n" | "--dryrun" => do_dry_run = true,
            "-o" | "--no-rm-orphans" => do_remove_orphans = false,
            "-q" | "--quiet" => be_quiet = true,
            "-h" | "--help" => {
                // TODO: Add syntax hints here
                println!("Charon: Mythos-util installer.");
                println!("charon [opts] [path/to/util.charon]");
                println!("Opts:");
                println!("-h | --help\t\tDisplay this menu.");
                println!("-n | --dryrun\t\tRun command but don't make any changes.");
                println!("-q | --quiet\t\tDon't display each item being installed.");
                println!("-o | --no-rm-orphans\t\tDon't remove files previously installed for this util, that are not included in the charon file.");
                return;
            },
            _ => {
                path_arg = arg.into();
                break;
            }
        }
    }

    let charon_path = match get_install_file_path(&path_arg) {
        Ok(path) => path, 
        Err(msg) => {
            printfatal!("{msg}");
        }
    };

    let (source_path, mut charon_file) = match load_charon_file(&charon_path) {
        Ok(file) => file,
        Err(msg) => {
            printfatal!("{msg}");
        },
    };

    let util_name = match source_path.file_stem() {
        Some(name) => name.to_string_lossy(),
        None => "".into()
    }.to_string();

    let contents = &mut String::new();
    if let Err(msg) = read_charon_file(&mut charon_file, contents) {
        printfatal!("{msg}");
    }

    // Read old files installed for this util
    let mut old_files = match &mut read_uninstall_file(&util_name) {
        Some(file) => parse_uninstall_file(file),
        None => Vec::new()
    };
    
    let actions = parse_install_file(contents, source_path, &util_name, do_dry_run);
    execute_actions(actions, do_dry_run, be_quiet, &util_name, &mut old_files); 
    
    if do_dry_run || !do_remove_orphans {
        return;
    }

    for file in old_files {
        if let Err(msg) = fs::remove_file(&file) {
            printwarn!("Could not remove file {file:?}\n{msg}");
        }
        else {
            printinfo!("Removed orphan {file:?}");
        }
    }
}

fn get_install_file_path(arg: &str) -> Result<PathBuf, String>{
    if arg.starts_with("/") {
        return Ok(PathBuf::from(arg));
    }

    // Non-absolute path || empty args
    let cwd = match env::current_dir() {
        Ok(cwd) => cwd,
        Err(_) => return Err("Could not get CWD".into())
    };
    if arg.len() == 0 {
        return Ok(cwd);
    }
    return Ok(cwd.join(arg));
}
fn load_charon_file(source_path: &PathBuf) -> Result<(PathBuf, File), String> {
    if source_path.is_file() && source_path.exists() {
        return match File::open(source_path) {
            Ok(file) => Ok((source_path.parent().unwrap().to_path_buf(), file)),
            Err(_) => Err("Could not read .charon file".into())
        }
    }
    // Try to read dir contents
    let contents = match fs::read_dir(&source_path) {
        Ok(dir) => dir,
        Err(_) => return Err(format!("Could not read directory: '{:?}'", source_path)),
    };


    // Search for .charon file
    for dir_entry in contents {
        let entry = match dir_entry {
            Ok(entry) => entry,
            Err(err) => return Err(err.to_string()),
        };

        if let Some(ext) = entry.path().extension() {
            if ext == "charon" {
                return match File::open(entry.path()) {
                    Ok(file) => Ok((source_path.to_path_buf(), file)),
                    Err(_) => Err("Could not read .charon file".into())
                }
            }
        }
    }
    return Err("Could not find .charon file".into());
}
fn read_charon_file(charon_file: &mut File, contents: &mut String) -> Result<String, String> {
    if let Err(err) = charon_file.read_to_string(contents) {
        return Err(format!("Could not read .charon file.\n{}", err.to_string()));
    }
    return Ok(contents.to_string());
}
fn read_uninstall_file(util_name: &str) -> Option<String> {
    let charon_dir = match dirs::get_dir(dirs::MythosDir::Data, "charon") {
        Some(dir) => dir,
        None => {
            printwarn!("Could not find charon directory at '$MYTHOS_DATA_DIR/charon/'");
            return None;
        }
    };

    let mut path = charon_dir.join(format!("{util_name}.charon"));
    if !path.exists() {
        path = charon_dir.join(format!("{util_name}.dryrun.charon"));
        if !path.exists() {
            return None;
        }
    }

    let mut file = File::open(path).expect("CHARON (Fatal Error): Could not open charon uninstall file"); 
    let contents = &mut String::new();
    file.read_to_string(contents).expect("CHARON (Fatal Error): Could not open charon uninstall file");
    return Some(contents.to_string());
}


/**
 * dry_run: bool -> if true, create charon_file, but don't make any changes
 */
fn execute_actions(actions: Vec<InstallAction>, dry_run: bool, quiet: bool, util_name: &str, old_files: &mut Vec<PathBuf>) {
    let log_path_root = dirs::get_dir(dirs::MythosDir::Data, "charon").expect(
        &fatalmsg!("Could not get mythos data dir")
    );

    let log_path = if dry_run {
        log_path_root.join(format!("{util_name}.dryrun.charon"))
    } else {
        log_path_root.join(format!("{util_name}.charon"))
    };

    let mut writer = BufWriter::new(File::create(log_path).expect("Could not open charon file"));

    for action in actions {
        let msg = match action.execute(dry_run, old_files) {
            Ok(msg) => msg,
            Err(msg) => { 
                eprintln!("{msg}");
                continue;
            }
        };
        if !quiet {
            println!("{}", msg.trim_end().to_string());
        }
        writer.write(&msg.into_bytes()).expect(&fatalmsg!("Could not write to charon file"));
    }
    writer.flush().expect(&fatalmsg!("Could not write to charon file"));
}
#[cfg(test)]
mod tests {
    #![allow(warnings)]
    use std::{panic, ffi::OsString};

    use super::*;

    fn setup() {
        let mut root = env::current_dir().unwrap();
        root.push("tests/charon/dest");

        let mut path = root.clone();
        path.push("alias");
        env::set_var("MYTHOS_ALIAS_DIR", path);

        path = root.clone();
        path.push("config");
        env::set_var("MYTHOS_CONFIG_DIR", path);

        path = root.clone();
        path.push("charon_file");
        env::set_var("MYTHOS_DATA_DIR", path);

        path = root.clone();
        path.push("data");
        env::set_var("MYTHOS_LOCAL_DATA_DIR", path);
        set_logger_id(UTIL_ID);
    }

   #[test]
    fn read_install_file() {
        setup();
        let (path, mut file) = load_charon_file(&PathBuf::from("tests/charon/targets/test1.charon")).unwrap();
        let mut contents = super::read_charon_file(&mut file, &mut "".into()).unwrap();
        let actions = parse_install_file(&mut contents, path, "", true);
        execute_actions(actions, true, true, "test", &mut Vec::new());

        // NOTE: Its easier to check this manually at the moment
        // let mut charon_file = File::open(PathBuf::from("tests/charon/dest/charon_file/charon/test.dryrun.charon")).unwrap(); 
        // charon_file.read_to_string(&mut contents).unwrap();
    }

    #[test]
    fn read_uninstall_file() {
        setup();
        let (path, mut file) = load_charon_file(&PathBuf::from("tests/charon/dest/charon_file/charon/test_uninstall.charon")).unwrap();
        let mut contents = super::read_charon_file(&mut file, &mut "".into()).unwrap();
        let paths = parse_uninstall_file(&mut contents);
        assert_eq!(
            paths, 
            vec![PathBuf::from("include/this/1"), PathBuf::from("include/this/2")]
        );
    }
    #[test]
    fn check_old_files() {
        setup();
        let (path, mut file) = load_charon_file(&PathBuf::from("tests/charon/targets/test2.charon")).unwrap();
        let mut contents = super::read_charon_file(&mut file, &mut "".into()).unwrap();
        let actions = parse_install_file(&mut contents, path, "", true);
        let mut old_files = parse_uninstall_file(&mut super::read_uninstall_file("test2").unwrap());

        execute_actions(actions, true, true, "test", &mut old_files);
        assert_eq!(
            old_files, 
            vec![PathBuf::from("this/file/should/be/here")]
        );
    }

    #[test]
    fn create_new_dirs() {
        setup();
        let (path, mut file) = load_charon_file(&PathBuf::from("tests/charon/targets/test3.charon")).unwrap();
        let mut contents = super::read_charon_file(&mut file, &mut "".into()).unwrap();
        let actions = parse_install_file(&mut contents, path, "test3", true);

        let p = match &actions[2] {
            InstallAction::File(file) => file,
            InstallAction::Dir(_) => panic!("")
        };
        assert_eq!(p.get_dest().file_name(), Some(OsString::from("test3").as_os_str()));
        // assert!(p.get_dest().to_string_lossy().ends_with("test3/test3"));

        let mut old_files = parse_uninstall_file(&mut super::read_uninstall_file("test3").unwrap());
        execute_actions(actions, true, false, "test3", &mut old_files);
    }
}
