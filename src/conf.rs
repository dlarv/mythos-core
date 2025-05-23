use toml::{Table, Value};
use std::path::PathBuf;
use serde_derive::{Serialize, Deserialize};
use crate::{self as mythos_core, printwarn};
use crate::{dirs, printerror};

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
     *
     * This will always try to open a file, unlike open(...), which will treat directories as
     * abstract config files.
     */
    pub fn open_file(path_snippet: &str) -> Option<MythosConfig> {
        let path = match try_get_file(path_snippet, false) {
            Some(path) => path,
            None => {
                printerror!("Could not find a config file for '{}'", path_snippet);
                return None;
            }
        };
        return MythosConfig::read_file(&path);
    }
    fn read_file(path: &PathBuf) -> Option<MythosConfig> {
        let contents = match std::fs::read_to_string(&path) {
            Ok(contents) => contents,
            Err(err) => {
                printerror!("Could not read config file for {:?}. Error msg: {}", path, err.to_string());
                return None;
            }
        };

        return match toml::from_str(&contents) {
            Ok(config) => Some(MythosConfig(config)),
            Err(_) => None
        };
    }
    /**
     * Tries to open config file.
     * If path is a dir, it is treated as an abstract config file, where each file and subdirectory are entries.
     * If path is a file, this method acts like MythosConfig::open_file(...).
     */
    pub fn open(path_snippet: &str) -> Option<MythosConfig> {
        let path = try_get_file(path_snippet, true)?;
        if path.is_file() { 
            return MythosConfig::read_file(&path) 
        };

        return match MythosConfig::read_dir(&path) {
            Ok(data) => Some(data),
            Err(err) => {
                printerror!("Error reading contents of path. \"{err}\".");
                return None;
            }
        }
    }
    fn read_dir(path: &PathBuf) -> Result<MythosConfig, std::io::Error> {
        let contents = path.read_dir()?; 
        let mut table: MythosConfig = MythosConfig(Table::new());

        for item in contents {
            let item = item?;
            let path = item.path();
            let key = path.file_stem().unwrap().to_str().unwrap();
            if path.is_dir() {
                table.extend(key, MythosConfig::read_dir(&path)?);
            } else if let Some(file) = MythosConfig::read_file(&path) {
                table.extend(key, file);
            } else {
                printwarn!("Tried to read {path:?} as config file. File was skipped.");
            }
        }
        return Ok(table);
    }
    pub fn extend(&mut self, key: &str, other: MythosConfig) {
        self.0.insert(key.to_string(), toml::Value::Table(other.0));
    }

    pub fn list_keys(&self) -> Vec<String> {
        return self.0.keys().into_iter().map(|x| x.to_owned()).collect();
    }

    pub fn get_subsection(&self, key: &str) -> Option<MythosConfig> {
        return match &self.0.get(key) {
            Some(Value::Table(val)) => {
                Some(MythosConfig(val.to_owned()))
            },
            _ => None
        };
    }

    pub fn get_string(&self, key: &str, default_val: &str) -> String {
        return match &self.0.get(key) {
            Some(Value::String(val)) => val.to_owned(),
            _ => default_val.to_string()
        };
    }
    pub fn try_get_string(&self, key: &str) -> Option<String> {
        return match &self.0.get(key) {
            Some(Value::String(val)) => Some(val.to_owned()),
            _ => None
        };
    }
    pub fn force_get_string(&self, key: &str) -> Option<String> {
        return match &self.0.get(key) {
            Some(Value::String(val)) => Some(val.into()),
            Some(Value::Float(val)) => Some(format!("{val}")),
            Some(Value::Integer(val)) => Some(format!("{val}")),
            Some(Value::Boolean(val)) => Some(format!("{val}")),
            Some(Value::Datetime(val)) => Some(format!("{val}")),
            Some(Value::Array(val)) => {
                let arr: String = val.into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
                Some(arr)
            },
            Some(Value::Table(val)) => {
                let tab: String = val.into_iter().map(|x| format!("{}:{}", x.0, x.1)).collect::<Vec<String>>().join(" ");
                Some(tab)
            },
            None => Some("".into()),
        }
    }

    pub fn get_integer(&self, key: &str, default_val: i64) -> i64 {
        return match &self.0.get(key) {
            Some(Value::Integer(val)) => val.to_owned(),
            _ => default_val
        };
    }
    pub fn try_get_integer(&self, key: &str) -> Option<i64> {
        return match &self.0.get(key) {
            Some(Value::Integer(val)) => Some(val.to_owned()),
            _ => None
        };
    }

    pub fn get_float(&self, key: &str, default_val: f64) -> f64 {
        return match &self.0.get(key) {
            Some(Value::Float(val)) => val.to_owned(),
            _ => default_val
        };
    }
    pub fn try_get_float(&self, key: &str) -> Option<f64> {
        return match &self.0.get(key) {
            Some(Value::Float(val)) => Some(val.to_owned()),
            _ => None
        };
    }
    pub fn get_boolean(&self, key: &str, default_val: bool) -> bool {
        return match &self.0.get(key) {
            Some(Value::Boolean(val)) => val.to_owned(),
            _ => default_val
        };
    }
    pub fn try_get_boolean(&self, key: &str) -> Option<bool> {
        return match &self.0.get(key) {
            Some(Value::Boolean(val)) => Some(val.to_owned()),
            _ => None
        };
    }

    pub fn get_datetime(&self, key: &str, default_val: &str) -> String{
        return match &self.0.get(key) {
            Some(Value::Datetime(val)) => val.to_string(),
            _ => default_val.to_string()
        };
    }
    pub fn try_get_datetime(&self, key: &str) -> Option<String> {
        return match &self.0.get(key) {
            Some(Value::Datetime(val)) => Some(val.to_string()),
            _ => None 
        };
    }
    /* 
     * While the other methods encapsulate/abstract the toml crate,
     * these three methods cannot due to their datatypes.
     */
    pub fn get_array(&self, key: &str, default_val: Vec<Value>) -> Vec<Value> {
        return match &self.0.get(key) {
            Some(Value::Array(val)) => val.to_owned(),
            _ => default_val
        };
    }
    pub fn try_get_array(&self, key: &str) -> Option<Vec<Value>> {
        return match &self.0.get(key) {
            Some(Value::Array(val)) => Some(val.to_owned()),
            _ => None
        };
    }
    pub fn get_typed_array<'a, T>(&self, key: &str) -> Vec<T> where T: serde::Deserialize<'a> {
        return match &self.0.get(key) {
            Some(Value::Array(val)) => val.to_owned(),
            _ => return vec![]
        }.into_iter()
            .filter_map(|x| x.try_into().ok())
            .collect()
    }
    pub fn get_table(&self, key: &str, default_val: Table) -> Table {
        return match &self.0.get(key) {
            Some(Value::Table(val)) => val.to_owned(),
            _ => default_val
        };
    }
    pub fn try_get_table(&self, key: &str) -> Option<Table> {
        return match &self.0.get(key) {
            Some(Value::Table(val)) => Some(val.to_owned()),
            _ => None
        };
    }
}
fn try_get_file(path: &str, allow_dir: bool) -> Option<PathBuf> {
    match clean_and_validate(dirs::expand_path(dirs::MythosDir::LocalConfig, path), allow_dir) {
        Some(path) => return Some(path),
        None => ()
    };
    return clean_and_validate(dirs::expand_path(dirs::MythosDir::Config, path), allow_dir);
}
/**
 * Caller can optionally omit file extension.
 * If {path} exists -> use that
 * Else -> try from list of valid extensions
 */
fn clean_and_validate(path: PathBuf, allow_dir: bool) -> Option<PathBuf> {
    if path.exists() {
        if !allow_dir && path.is_dir() {
            let err_msg = format!("'{:?}' is a directory", path);

            // Check CONFIG_DIR/util_name/config
            let mut new_path = path;
            new_path.push("config");

            if new_path.exists() {
                return Some(new_path);
            }
            eprintln!("{}", err_msg);
            return None;
        }
        return Some(path);
    }

    for ext in VALID_CONFIG_EXT {
        if path.with_extension(ext).exists() {
            return Some(path.with_extension(ext));
        }
    }
    printerror!("Path {path:?} does not exist and doesn't contain any config-like files.");
    return None;
}

#[cfg(test)]
pub mod tests {
    #![allow(warnings)]
    use super::*;
    fn setup() {
        unsafe {
            std::env::set_var("MYTHOS_LOCAL_CONFIG_DIR", "tests/lconfig");
            std::env::set_var("MYTHOS_CONFIG_DIR", "tests/config");
        }
    }
    #[test]
    pub fn try_open_dir_as_file() {
        setup();
        let dir = try_get_file("empty_dir", false);
        assert!(dir.is_none());
    }
    #[test]
    pub fn try_open_dir() {
        setup();
        let conf = MythosConfig::open("abstract_config").unwrap();
        let dict2 = conf.get_subsection("dict2").unwrap();
        assert_eq!(dict2.try_get_integer("value"), Some(100));
        assert_eq!(dict2.get_subsection("dict3").unwrap().try_get_integer("value"), Some(1000));
        
        let list1 = conf.get_subsection("list1").unwrap();
        let dict1 = list1.get_subsection("list1_dict1").unwrap();
        assert_eq!(dict1.try_get_integer("value"), Some(9));

        let list2 = list1.get_subsection("list1_list2").unwrap();
        let config = list2.get_subsection("config").unwrap();
        assert_eq!(config.try_get_integer("value"), Some(11));

        let list3 = list2.get_subsection("list1_list2_dict1").unwrap();
        assert_eq!(list3.try_get_integer("value"), Some(1));
    }
    #[test]
    pub fn try_open_file() {
        setup();
        let conf = MythosConfig::open("config_tester").unwrap();
        assert_eq!(conf.try_get_integer("int"), Some(1));
    }
    #[test]
    pub fn get_file_with_no_ext() {
        setup();
        let root = dirs::expand_path(dirs::MythosDir::Config, "config_tester");
        assert_eq!(clean_and_validate(root, false), Some(PathBuf::from("tests/config/config_tester.conf")));
        let root = dirs::expand_path(dirs::MythosDir::Config, "arachne");
        assert_eq!(clean_and_validate(root, false), Some(PathBuf::from("tests/config/arachne")));
    }
    #[test]
    pub fn defaults_to_local_config() {
        setup();
        let file = try_get_file("config_tester", false);
        assert_eq!(file, Some(PathBuf::from("tests/lconfig/config_tester.toml")));
    }
    #[test]
    pub fn get_file_named_config() {
        setup();
        let file = try_get_file("config_tester_dir", false);
        assert_eq!(file, Some(PathBuf::from("tests/lconfig/config_tester_dir/config")));
    }
    #[test]
    pub fn get_value() {
        setup();
        let config = MythosConfig::open_file("config_tester").unwrap();

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
    #[test]
    pub fn try_get_value() {
        setup();
        let config = MythosConfig::open_file("config_tester").unwrap();

        assert_eq!(config.try_get_float("float2"), None);
        assert_eq!(config.try_get_string("string2"), None);
        assert_eq!(config.try_get_integer("int2"), None);
        assert_eq!(config.try_get_boolean("bool2"), None);
        assert_eq!(config.try_get_datetime("date2"), None);
        assert_eq!(config.try_get_array("array2"),  None);
        assert_eq!(config.try_get_table("table2"), None);
    }
    #[test]
    pub fn get_typed_array() {
        setup();
        let config = MythosConfig::open_file("config_tester").unwrap();
        let array = config.get_typed_array::<i64>("typed_array");
        assert_eq!(array, vec![1, 2, 3]);
        let array = config.get_typed_array::<String>("typed_array");
        assert_eq!(array.len(), 0);
    }
}

