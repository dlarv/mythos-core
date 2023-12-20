use mythos_core::conf::MythosConfig;
use std::env;

fn main() {
    let mut args = env::args().skip(1);
    if let Some(util_name) = args.next() {
        println!("{data}", data = get_value(&util_name, args.map(|x| x).collect()));
    }
}

fn get_value(util_name: &str, mut keys: Vec<String>) -> String {
    let mut conf = match MythosConfig::read_file(&util_name) {
        Some(conf) => conf,
        None => return "".into()
    };
    while keys.len() > 1 {
        conf = match conf.get_subsection(&keys.remove(0)) {
            Some(conf) => conf,
            None => return "".into()
        };
    }

    return match conf.force_get_string(&keys.remove(0)) {
        Some(val) => val,
        None => "".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn setup() {
        std::env::set_var("MYTHOS_CONFIG_DIR", "tests/config");
        std::env::set_var("MYTHOS_LOCAL_CONFIG_DIR", "tests/lconfig");
    }

    #[test]
    fn test_get_number() {
        setup();
        assert_eq!(get_value("config_tester", vec!["array".into()]), "0 1");
    }
}
