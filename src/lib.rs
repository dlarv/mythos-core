pub mod dirs;

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
