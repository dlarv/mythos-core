use toml::{Table, Value};
use std::path::PathBuf;
use serde_derive::{Serialize, Deserialize};
use crate::dirs;

const VALID_CONFIG_EXT: [&str; 3] = [
    "conf",
    "ini",
    "toml",
];

#[derive(Debug, Serialize, Deserialize)]
pub struct MythosConfig(Table);

impl MythosConfig {
    /**
     * path_snippet: &str can be either:
     * - "util_name" -> "$MYTHOS_CONFIG_DIR/util_name{.ext}" || "$MYTHOS_CONFIG_DIR/util_name/config"
     * - "dir_name/file_name" -> "$MYTHOS_CONFIG_DIR/file_name{.ext}"
     */
    pub fn read_file(path_snippet: &str) -> Option<MythosConfig> {
        let path = match try_get_file(path_snippet) {
            Some(path) => path,
            None => {
                eprintln!("Could not find a config file for '{}'", path_snippet);
                return None;
            }
        };

        let contents = match std::fs::read_to_string(path) {
            Ok(contents) => contents,
            Err(err) => {
                eprintln!("Could not read config file for '{}'. Error msg: {}", path_snippet, err.to_string());
                return None;
            }
        };

        return match toml::from_str(&contents) {
            Ok(config) => Some(MythosConfig(config)),
            Err(_) => None
        };
    }

    pub fn get_string(&self, key: &str, default_val: &str) -> String {
        return match &self.0[key] {
            Value::String(val) => val.to_owned(),
            _ => default_val.to_string()
        };
    }

    pub fn get_integer(&self, key: &str, default_val: i64) -> i64 {
        return match &self.0[key] {
            Value::Integer(val) => val.to_owned(),
            _ => default_val
        };
    }

    pub fn get_float(&self, key: &str, default_val: f64) -> f64 {
        return match &self.0[key] {
            Value::Float(val) => val.to_owned(),
            _ => default_val
        };
    }
    pub fn get_boolean(&self, key: &str, default_val: bool) -> bool {
        return match &self.0[key] {
            Value::Boolean(val) => val.to_owned(),
            _ => default_val
        };
    }

    pub fn get_datetime(&self, key: &str, default_val: &str) -> String{
        return match &self.0[key] {
            Value::Datetime(val) => val.to_string(),
            _ => default_val.to_string()
        };
    }
    /* 
     * While the other methods encapsulate/abstract the toml crate,
     * these three methods cannot due to their datatypes.
     */
    pub fn get_array(&self, key: &str, default_val: Vec<Value>) -> Vec<Value> {
        return match &self.0[key] {
            Value::Array(val) => val.to_owned(),
            _ => default_val
        };
    }
    pub fn get_table(&self, key: &str, default_val: Table) -> Table {
        return match &self.0[key] {
            Value::Table(val) => val.to_owned(),
            _ => default_val
        };
    }


}
fn try_get_file(path: &str) -> Option<PathBuf> {
    match clean_and_validate(*dirs::get_path(dirs::MythosDir::LocalConfig, path)) {
        Some(path) => return Some(path),
        None => ()
    };
    match clean_and_validate(*dirs::get_path(dirs::MythosDir::Config, path)) {
        Some(path) => return Some(path),
        None => return None 
    };
}
/**
 * Caller can optionally omit file extension.
 * If {path} exists -> use that
 * Else -> try from list of valid extensions
 */
fn clean_and_validate(path: PathBuf) -> Option<PathBuf> {
    if path.exists() {
        if path.is_dir() {
            let err_msg = format!("'{:?}' is a directory", path);

            // Check CONFIG_DIR/util_name/config
            let mut new_path = path;
            new_path.push("config");

            if new_path.exists() {
                return Some(new_path);
            }
            else {
                eprintln!("{}", err_msg);
                return None;
            }
        }
        return Some(path);
    }

    for ext in VALID_CONFIG_EXT {
        if path.with_extension(ext).exists() {
            return Some(path.with_extension(ext));
        }
    }
    return None;
}


#[cfg(test)]
pub mod tests {
    use super::*;
    fn setup() {
        std::env::set_var("MYTHOS_CONFIG_DIR", "tests/config");
        std::env::set_var("MYTHOS_LOCAL_CONFIG_DIR", "tests/lconfig");
    }
    #[test]
    pub fn get_file_with_implicit_ext() {
        setup();
        let root = dirs::get_path(dirs::MythosDir::Config, "config_tester");
        assert_eq!(clean_and_validate(*root), Some(PathBuf::from("tests/config/config_tester.conf")));
    }
    #[test]
    pub fn get_file_with_no_ext() {
        setup();
        let root = dirs::get_path(dirs::MythosDir::Config, "arachne");
        assert_eq!(clean_and_validate(*root), Some(PathBuf::from("tests/config/arachne")));
    }
    #[test]
    pub fn defaults_to_local_config() {
        setup();
        let file = try_get_file("config_tester");
        assert_eq!(file, Some(PathBuf::from("tests/lconfig/config_tester.toml")));
    }
    #[test]
    pub fn get_file_named_config() {
        setup();
        let file = try_get_file("config_tester_dir");
        assert_eq!(file, Some(PathBuf::from("tests/lconfig/config_tester_dir/config")));
    }
    #[test]
    pub fn get_value() {
        setup();
        let config = MythosConfig::read_file("config_tester").unwrap();

        assert_eq!(config.get_float("float", -1.0), 1.1);
        assert_eq!(config.get_string("string", ""), "string".to_string());
        assert_eq!(config.get_integer("int", -1), 1);
        assert_eq!(config.get_boolean("bool", false), true);

        assert_eq!(config.get_datetime("date", "2000-01-01"), "1970-01-01");
        assert_eq!(
            config.get_array("array", vec![Value::Integer(-1), Value::Integer(-2)]),
            vec![Value::Integer(0), Value::Integer(1)]
            );
        assert_eq!(config.get_table("table", Table::new())["int1"], Value::from(1));

    }
}

