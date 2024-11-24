/*!
 * Get paths to MYTHOS_DIR 
 * These are saved as environment variables on the user's system.
 *
 * Var Name                | Default Path
 * MYTHOS_ALIAS_DIR          /etc/.profile.d  
 * MYTHOS_CONFIG_DIR         /etc/mythos 
 * MYTHOS_DATA_DIR           /usr/share/mythos
 * MYTHOS_BIN_DIR            /bin
 * MYTHOS_LIB_DIR            /usr/lib/mythos
 * MYTHOS_LOCAL_CONFIG_DIR   ~/.config/mythos
 * MYTHOS_LOCAL_DATA_DIR     ~/.local/share/mythos",
 * MYTHOS_LOG_DIR           ~/.local/share/mythos/logs
 * 
 */
use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use duct::cmd;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum MythosDir { Config, Data, Bin, Lib, Alias, LocalData, LocalConfig, Log }

// Returns home diretory of $SUDO_USER
pub fn get_home() -> Option<PathBuf> {
    // Get $SUDO_USER or $HOME
    let user = match env::var("SUDO_USER") {
        Ok(user) => user,
        Err(_) => match env::var("HOME") {
            Ok(home) => return Some(PathBuf::from(home)),
            Err(_) => return None
        }
    };
    let output = match cmd!("getent", "passwd", &user)
                    .pipe(cmd!("cut", "-d:", "-f6"))
                    .stdout_capture() 
                    .read() {
        Ok(output) => output,
        Err(_) => return None 
    };

    return Some(PathBuf::from(output));
}

pub fn get_path(dir_name: MythosDir, util_name: &str) -> Option<PathBuf> {
    //! Returns MYTHOS_DIR/util_name, if it exists. 
    //! Path can point to a file or dir.
    let mut path = expand_path(dir_name, util_name);
    if path.exists() {
        return Some(path);
    }
    path = expand_path(dir_name, "");
    if path.exists() {
        return Some(path);
    }
    return None;
}

pub fn make_dir(dir_name: MythosDir, util_name: &str) -> Result<PathBuf, std::io::Error> {
    //! Create directory if it does not exist. 
    //! Return error if directory could not be created and DNE
    let path = expand_path(dir_name, util_name);
    
    // create_dir_all fails if dir already exists or user doesn't have permissions
    // This function should only throw an error for the latter
    if !path.exists() {
        let _ = fs::create_dir_all(path.clone())?;
    }

    return Ok(path);
}

pub fn expand_path(dir_name: MythosDir, util_name: &str) -> PathBuf {
    //! Returns MYTHOS_DIR/util_name, even if path dne.
    //! If util_name is empty, path will be redirected to mythos core.
    let env_var: &str = match &dir_name {
		MythosDir::Config => "MYTHOS_CONFIG_DIR", 
		MythosDir::Data => "MYTHOS_DATA_DIR", 
		MythosDir::Bin => "MYTHOS_BIN_DIR", 
		MythosDir::Lib => "MYTHOS_LIB_DIR", 
		MythosDir::LocalConfig => "MYTHOS_LOCAL_CONFIG_DIR", 
		MythosDir::LocalData => "MYTHOS_LOCAL_DATA_DIR", 
		MythosDir::Alias => "MYTHOS_ALIAS_DIR",
        MythosDir::Log => "MYTHOS_LOG_DIR",
    };

    // Get base path
    let mut path_name = env::var(env_var).unwrap_or(get_default_dir(dir_name));
    let home_dir: String = match get_home() {
        Some(path) => path.to_string_lossy().to_string(),
        None => "/".to_string()
    };
    path_name = path_name.replace("~", &home_dir).replace("$HOME", &home_dir);

    let path = Path::new(&path_name);

    // If only asking for core, return
    if util_name.to_lowercase() == "core" || util_name == "" {
        return path.to_owned();
    }

    // /bin shouldn't have any subdirs
    if MythosDir::Bin == dir_name {
        return path.to_owned();
    }

    // Append util to path
    return path.join(util_name);
}

fn get_default_dir(dir_name: MythosDir) -> String {
    return match dir_name {
		MythosDir::Config => "/etc/mythos", 
		MythosDir::Data => "/usr/share/mythos", 
		MythosDir::Bin => "/bin", 
		MythosDir::Lib => "/usr/lib/mythos", 
		MythosDir::LocalConfig => "~/.config/mythos",
		MythosDir::LocalData => "~/.local/share/mythos",
		MythosDir::Alias => "/etc/profile.d",
        MythosDir::Log => "~/.local/share/mythos/logs",
    }.into();
}

pub fn expand_mythos_shortcut(shortcut: &str, util_name: &str) -> Option<PathBuf> {
    //! Expands a simplified shortcut into the full path. e.g. MYTHOS_DIR/util_name.
    //! shortcuts can be any of the following:
    //! A | ALIAS       $MYTHOS_ALIAS_DIR
    //! B | BIN         $MYTHOS_BIN_DIR
    //! C | CONFIG      $MYTHOS_CONFIG_DIR
    //! D | DATA        $MYTHOS_DATA_DIR
    //! LB | LIB        $MYTHOS_LIB_DIR
    //! LO | LOG        $MYTHOS_LOG_DIR
    //! HOME | ~        $HOME
    //! LC | LCONFIG | LOCALCONFIG      $MYTHOS_LOCAL_CONFIG_DIR
    //! LD | LDATA   | LOCALDATA        $MYTHOS_LOCAL_DATA_DIR
    //!
    return match shortcut.trim_start_matches("$").to_uppercase().as_str() {
        "A" | "ALIAS" => Some(expand_path(MythosDir::Alias, util_name)),
        "B" | "BIN" => Some(expand_path(MythosDir::Bin, util_name)),
        "C" | "CONFIG" => Some(expand_path(MythosDir::Config, util_name)),
        "D" | "DATA" => Some(expand_path(MythosDir::Data, util_name)),
        "LB" | "LIB" => Some(expand_path(MythosDir::Lib, util_name)),
        "LC" | "LCONFIG" | "LOCALCONFIG" => Some(expand_path(MythosDir::LocalConfig, util_name)),
        "LD" | "LDATA" | "LOCALDATA" => Some(expand_path(MythosDir::LocalData, util_name)),
        "LO" | "LOG" => Some(expand_path(MythosDir::Log, util_name)),
        "HOME" | "~" => get_home(),
        _ => None
    }
}
#[cfg(test)]
mod tests {
    #![allow(warnings)]
    use std::fs::remove_dir;
    use std::sync::{Arc, Mutex};
    use crate::cli::clean_cli_args;

    use super::*;
    
    // Create environment to run tests
    fn setup() {
        unsafe {
            env::set_var("MYTHOS_ALIAS_DIR", "tests/alias");
            env::set_var("MYTHOS_BIN_DIR", "tests/bin");
            env::set_var("MYTHOS_CONFIG_DIR", "tests/config");
            env::set_var("MYTHOS_DATA_DIR", "tests/data");
            env::set_var("MYTHOS_LIB_DIR", "tests/lib");
            env::set_var("MYTHOS_LOCAL_CONFIG_DIR", "tests/lconfig");
            env::set_var("MYTHOS_LOCAL_DATA_DIR", "tests/ldata");
        }
    }

    #[test]
    fn test_get_home() {
        let actual = PathBuf::from(env::var("HOME").unwrap());
        unsafe {
            env::set_var("HOME", "noname");
            env::set_var("SUDO_USER", env::var("USER").unwrap());
            let dir = get_home().unwrap();
            assert_eq!(dir, actual);
            env::set_var("HOME", actual);
        }
    }

    #[test]
    fn check_test_env() {
        setup();
        assert_eq!(super::expand_path(MythosDir::Alias, "".into()), Path::new(&"tests/alias".to_string()));
        assert_eq!(super::expand_path(MythosDir::Bin, "".into()), Path::new(&"tests/bin".to_string()));
        assert_eq!(super::expand_path(MythosDir::Config, "".into()), Path::new(&"tests/config".to_string()));
        assert_eq!(super::expand_path(MythosDir::Data, "".into()), Path::new(&"tests/data".to_string()));
        assert_eq!(super::expand_path(MythosDir::Lib, "".into()), Path::new(&"tests/lib".to_string()));
        assert_eq!(super::expand_path(MythosDir::LocalData, "".into()), Path::new(&"tests/ldata".to_string()));
        assert_eq!(super::expand_path(MythosDir::LocalConfig, "".into()), Path::new(&"tests/lconfig".to_string()));
        
    }
    #[test]
    fn test_get_path() {
        setup();
        assert_eq!(expand_path(MythosDir::Config, "arachne"), Path::new(&"tests/config/arachne".to_string()));
        
    }
    #[test]
    fn get_dir_that_exists() {
        setup();
        let path = get_path(MythosDir::Config, "arachne");
        assert_eq!(path, Some(PathBuf::from(&"tests/config/arachne")));
        
    }
    #[test]
    fn get_dir_that_dne() {
        setup();
        let path = get_path(MythosDir::Config, "nonameutil");
        assert_eq!(path, Some(PathBuf::from("tests/config")));
    }
    #[test]
    fn make_dir_that_exists() {
        setup();
        let path = make_dir(MythosDir::Config, "arachne").unwrap();
        assert_eq!(path, PathBuf::from("tests/config/arachne"));
    }
    #[test]
    fn make_dir_that_dne() {
        setup();
        let path = make_dir(MythosDir::Config, "mythos-test-file").unwrap();
        assert_eq!(path, PathBuf::from("tests/config/mythos-test-file"));
        remove_dir(path);
        
    }
}
