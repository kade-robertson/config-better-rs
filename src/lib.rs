use std::{
    fmt,
    path::{Path, PathBuf},
};

use env::OverridableEnv;

mod env;

pub struct Config {
    cache_dir: PathBuf,
    config_dir: PathBuf,
    data_dir: PathBuf,
}

impl Config {
    fn _init(app_name: &str, os: &str, env: OverridableEnv) -> Config {
        Config {
            cache_dir: match env.get("XDG_CACHE_HOME") {
                Ok(dir) => Path::new(&dir).join(app_name),
                Err(_) => match os {
                    "windows" => Path::new(&env.get("APPDATA").unwrap_or(".".to_string()))
                        .join(app_name)
                        .join("Cache"),
                    "macos" => Path::new(&env.get("HOME").unwrap_or(".".to_string()))
                        .join("Library")
                        .join("Caches")
                        .join(app_name),
                    _ => Path::new(&env.get("HOME").unwrap_or(".".to_string()))
                        .join(".cache")
                        .join(app_name),
                },
            },
            config_dir: match env.get("XDG_CONFIG_HOME") {
                Ok(dir) => Path::new(&dir).join(app_name),
                Err(_) => match os {
                    "windows" => Path::new(&env.get("APPDATA").unwrap_or(".".to_string()))
                        .join(app_name)
                        .join("Config"),
                    "macos" => Path::new(&env.get("HOME").unwrap_or(".".to_string()))
                        .join("Library")
                        .join("Preferences")
                        .join(app_name),
                    _ => Path::new(&env.get("HOME").unwrap_or(".".to_string()))
                        .join(".config")
                        .join(app_name),
                },
            },
            data_dir: match env.get("XDG_DATA_HOME") {
                Ok(dir) => Path::new(&dir).join(app_name),
                Err(_) => match os {
                    "windows" => Path::new(&env.get("APPDATA").unwrap_or(".".to_string()))
                        .join(app_name)
                        .join("Data"),
                    "macos" => Path::new(&env.get("HOME").unwrap_or(".".to_string()))
                        .join("Library")
                        .join(app_name),
                    _ => Path::new(&env.get("HOME").unwrap_or(".".to_string()))
                        .join(".local")
                        .join("share")
                        .join(app_name),
                },
            },
        }
    }

    pub fn new(app_name: &str) -> Config {
        Config::_init(app_name, std::env::consts::OS, env::OverridableEnv::new())
    }
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Config {{ cache_dir: {:?}, config_dir: {:?}, data_dir: {:?} }}",
            self.cache_dir, self.config_dir, self.data_dir
        )
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::{env::OverridableEnv, Config};

    fn get_windows_config() -> Config {
        env::remove_var("XDG_CACHE_HOME");
        env::remove_var("XDG_CONFIG_HOME");
        env::remove_var("XDG_DATA_HOME");
        let mut env = OverridableEnv::new();
        env.add("APPDATA", "/appdata");
        Config::_init("app-name", "windows", env)
    }

    fn get_macos_config() -> Config {
        env::remove_var("XDG_CACHE_HOME");
        env::remove_var("XDG_CONFIG_HOME");
        env::remove_var("XDG_DATA_HOME");
        let mut env = OverridableEnv::new();
        env.add("HOME", "/home/user");
        Config::_init("app-name", "macos", env)
    }

    fn get_linux_config() -> Config {
        env::remove_var("XDG_CACHE_HOME");
        env::remove_var("XDG_CONFIG_HOME");
        env::remove_var("XDG_DATA_HOME");
        let mut env = OverridableEnv::new();
        env.add("HOME", "/home/user");
        Config::_init("app-name", "linux", env)
    }

    #[test]
    fn it_respects_xdg_cache_home() {
        let mut env = OverridableEnv::new();
        env.add("XDG_CACHE_HOME", "/tmp/cache");
        let config = Config::_init("app-name", "any", env);
        assert_eq!(config.cache_dir.to_str().unwrap(), "/tmp/cache/app-name");
    }

    #[test]
    fn it_uses_expected_windows_cache_path() {
        let config = get_windows_config();
        assert_eq!(
            config.cache_dir.to_str().unwrap(),
            "/appdata/app-name/Cache"
        );
    }

    #[test]
    fn it_uses_expected_macos_cache_path() {
        let config = get_macos_config();
        assert_eq!(
            config.cache_dir.to_str().unwrap(),
            "/home/user/Library/Caches/app-name"
        );
    }

    #[test]
    fn it_uses_expected_linux_cache_path() {
        let config = get_linux_config();
        assert_eq!(
            config.cache_dir.to_str().unwrap(),
            "/home/user/.cache/app-name"
        );
    }

    #[test]
    fn it_respects_xdg_config_home() {
        let mut env = OverridableEnv::new();
        env.add("XDG_CONFIG_HOME", "/tmp/config");
        let config = Config::_init("app-name", "any", env);
        assert_eq!(config.config_dir.to_str().unwrap(), "/tmp/config/app-name");
    }

    #[test]
    fn it_uses_expected_windows_config_path() {
        let config = get_windows_config();
        println!("{:?}", config);
        assert_eq!(
            config.config_dir.to_str().unwrap(),
            "/appdata/app-name/Config"
        );
    }

    #[test]
    fn it_uses_expected_macos_config_path() {
        let config = get_macos_config();
        assert_eq!(
            config.config_dir.to_str().unwrap(),
            "/home/user/Library/Preferences/app-name"
        );
    }

    #[test]
    fn it_uses_expected_linux_config_path() {
        let config = get_linux_config();
        assert_eq!(
            config.config_dir.to_str().unwrap(),
            "/home/user/.config/app-name"
        );
    }

    #[test]
    fn it_respects_xdg_data_home() {
        let mut env = OverridableEnv::new();
        env.add("XDG_DATA_HOME", "/tmp/data");
        let config = Config::_init("app-name", "any", env);
        assert_eq!(config.data_dir.to_str().unwrap(), "/tmp/data/app-name");
    }

    #[test]
    fn it_uses_expected_windows_data_path() {
        let config = get_windows_config();
        assert_eq!(config.data_dir.to_str().unwrap(), "/appdata/app-name/Data");
    }

    #[test]
    fn it_uses_expected_macos_data_path() {
        let config = get_macos_config();
        assert_eq!(
            config.data_dir.to_str().unwrap(),
            "/home/user/Library/app-name"
        );
    }

    #[test]
    fn it_uses_expected_linux_data_path() {
        let config = get_linux_config();
        assert_eq!(
            config.data_dir.to_str().unwrap(),
            "/home/user/.local/share/app-name"
        );
    }
}
