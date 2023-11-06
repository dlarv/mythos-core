pub mod parser;
use parser::*;
use mythos_core::dirs;
use std::env;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;


fn main() {
    let mut do_dry_run = false;
    let mut do_remove_orphans = true;
    let mut path_arg = String::new();

    let args = mythos_core::cli::clean_cli_args();
    for arg in args {
        match arg.as_str() {
            "-n" | "--dryrun" => do_dry_run = true,
            "-o" | "--no-rm-orphans" => do_remove_orphans = false,
            _ => {
                path_arg = arg.into();
                break;
            }
        }
    }

    let charon_path = match get_install_file_path(&path_arg) {
        Ok(path) => path, 
        Err(msg) => {
            eprintln!("CHARON (Fatal Error): {msg}");
            return;
        }
    };

    let (source_path, mut charon_file) = match load_charon_file(&charon_path) {
        Ok(file) => file,
        Err(msg) => {
            eprintln!("CHARON (Fatal Error): {msg}");
            return;
        },
    };

    let util_name = match source_path.file_stem() {
        Some(name) => name.to_string_lossy(),
        None => "".into()
    }.to_string();

    let contents = &mut String::new();
    if let Err(msg) = read_charon_file(&mut charon_file, contents) {
        eprintln!("CHARON (Fatal Error): {msg}");
        return;
    }

    // Read old files installed for this util
    let mut old_files = match &mut read_uninstall_file(&util_name) {
        Some(file) => parse_uninstall_file(file),
        None => Vec::new()
    };
    
    let actions = parse_install_file(contents, source_path);
    execute_actions(actions, do_dry_run, &util_name, &mut old_files); 
    
    if do_dry_run || !do_remove_orphans {
        return;
    }

    for file in old_files {
        if let Err(msg) = fs::remove_file(&file) {
            eprintln!("CHARON (Error): Could not remove file {:?}\n{}", file, msg);
        }
        else {
            println!("CHARON (INFO): Removed orphan {:?}", file);
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
    let charon_dir = *match dirs::get_dir(dirs::MythosDir::Data, "charon") {
        Some(dir) => dir,
        None => {
            eprintln!("CHARON (Warning): Could not find charon directory at '$MYTHOS_DATA_DIR/charon/'");
            return None;
        }
    };

    let path = charon_dir.join(format!("{}.charon", util_name));
    if !path.exists() {
        return None;
    }
    let mut file = File::open(path).expect("CHARON (Fatal Error): Could not open charon uninstall file"); 
    let contents = &mut String::new();
    file.read_to_string(contents).expect("CHARON (Fatal Error): Could not open charon uninstall file");
    return Some(contents.to_string());
}


/**
 * dry_run: bool -> if true, create charon_file, but don't make any changes
 */
fn execute_actions(actions: Vec<InstallFile>, dry_run: bool, util_name: &str, old_files: &mut Vec<PathBuf>) {
    let err_msg = "CHARON (Fatal Error):";
    let log_path_root = dirs::get_dir(dirs::MythosDir::Data, "charon").expect(
        &format!("{err_msg} Could not get mythos data dir")
    );
    let log_path = if dry_run {
        log_path_root.join(format!("{util_name}.dryrun.charon"))
    } else {
        log_path_root.join(format!("{util_name}.charon"))
    };

    // TODO: read charon file if it already exists, to run a comparison.
    let mut writer = BufWriter::new(File::create(log_path).expect("Could not open charon file"));

    for action in actions {
        let msg = match action.execute(dry_run, old_files) {
            Ok(msg) => msg,
            Err(msg) => { 
                eprintln!("{msg}");
                continue;
            }
        };
        println!("{msg}");
        writer.write(&msg.into_bytes()).expect("Could not write to charon file");
    }
    writer.flush().expect("Could not write to charon file");
}
#[cfg(test)]
mod tests {
    #![allow(warnings)]
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
    }

//    #[test]
    fn read_install_file() {
        setup();
        let (path, mut file) = load_charon_file(&PathBuf::from("tests/charon/target/test1.charon")).unwrap();
        let mut contents = super::read_charon_file(&mut file, &mut "".into()).unwrap();
        let actions = parse_install_file(&mut contents, path);
        execute_actions(actions, true, "test", &mut Vec::new());

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
        let (path, mut file) = load_charon_file(&PathBuf::from("tests/charon/target/test2.charon")).unwrap();
        let mut contents = super::read_charon_file(&mut file, &mut "".into()).unwrap();
        let actions = parse_install_file(&mut contents, path);
        let mut old_files = parse_uninstall_file(&mut super::read_uninstall_file("test2").unwrap());

        execute_actions(actions, true, "test", &mut old_files);
        assert_eq!(
            old_files, 
            vec![PathBuf::from("this/file/should/be/here")]
        );
    }
}
