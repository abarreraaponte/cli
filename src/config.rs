use std::path::{Path, PathBuf};

pub fn credential_path(config_home: &Path) -> PathBuf {
    config_home.join("contextdrive").join("credentials")
}

pub fn default_config_home() -> Option<PathBuf> {
    dirs::config_dir()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn credential_file_is_under_the_config_home() {
        let path = credential_path(Path::new("/tmp/cfg"));
        assert_eq!(path, PathBuf::from("/tmp/cfg/contextdrive/credentials"));
    }
}
