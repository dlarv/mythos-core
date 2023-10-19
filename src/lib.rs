pub mod dirs;
pub fn clean_cli_args() -> Vec<String> {
    return std::env::args().into_iter().skip(1).flat_map(|x| {
        if x.starts_with("--") || !x.starts_with("-") {
            vec![x]
        }
        else {
            x.chars().into_iter().skip(1).map(|x| format!("-{}", x)).collect()
        }
    }).collect();
}
#[cfg(test)]
mod tests {
    use super::*;
    use dirs::*;
    use std::path::Path;

    // Separated from others. Does not use setup(), checks to ensure get_path works
    #[test]
    fn test_get_path_core() {
        assert_eq!(*dirs::get_path(MythosDir::Config, "".into()), Path::new(&"/etc/mythos".to_string()));
    }
}
