#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

use std::{
    fmt::{self, Display},
    fs,
    path::{Path, PathBuf},
};

use errors::{CreateError, RemoveError};

mod errors;

#[derive(Debug, Clone)]
pub struct Directory {
    path: PathBuf,
}

impl Directory {
    pub(crate) fn new(path: PathBuf) -> Directory {
        Directory { path }
    }

    pub fn create(&self) -> std::io::Result<()> {
        fs::create_dir_all(&self.path)
    }

    pub fn remove(&self) -> std::io::Result<()> {
        fs::remove_dir_all(&self.path)
    }
}

impl Display for Directory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path.display())
    }
}

pub struct Config {
    /// The cache directory for your application.
    ///
    /// The expected path is as follows for an app called "do-stuff":
    /// * If `$XDG_CACHE_HOME` is defined: `$XDG_CACHE_HOME/do-stuff`
    /// * If the OS is Windows: `%APPDATA%/do-stuff/Cache`
    /// * If the OS is MacOS: `$HOME/Library/Caches/do-stuff`
    /// * Otherwise: `$HOME/.cache/do-stuff`
    pub cache: Directory,

    /// The config directory for your application.
    ///
    /// The expected path is as follows for an app called "do-stuff":
    /// * If `$XDG_CONFIG_HOME` is defined: `$XDG_CONFIG_HOME/do-stuff`
    /// * If the OS is Windows: `%APPDATA%/do-stuff/Config`
    /// * If the OS is MacOS: `$HOME/Library/Preferences/do-stuff`
    /// * Otherwise: `$HOME/.config/do-stuff`
    pub config: Directory,

    /// The data directory for your application.
    ///
    /// The expected path is as follows for an app called "do-stuff":
    /// * If `$XDG_DATA_HOME` is defined: `$XDG_DATA_HOME/do-stuff`
    /// * If the OS is Windows: `%APPDATA%/do-stuff/Data`
    /// * If the OS is MacOS: `$HOME/Library/do-stuff`
    /// * Otherwise: `$HOME/.local/share/do-stuff`
    pub data: Directory,
}

impl Config {
    fn _init(app_name: &str, os: &str) -> Config {
        Config {
            cache: Directory::new(match std::env::var("XDG_CACHE_HOME") {
                Ok(dir) => Path::new(&dir).join(app_name),
                Err(_) => match os {
                    "windows" => Path::new(&std::env::var("APPDATA").unwrap_or(".".to_string()))
                        .join(app_name)
                        .join("Cache"),
                    "macos" => Path::new(&std::env::var("HOME").unwrap_or(".".to_string()))
                        .join("Library")
                        .join("Caches")
                        .join(app_name),
                    _ => Path::new(&std::env::var("HOME").unwrap_or(".".to_string()))
                        .join(".cache")
                        .join(app_name),
                },
            }),
            config: Directory::new(match std::env::var("XDG_CONFIG_HOME") {
                Ok(dir) => Path::new(&dir).join(app_name),
                Err(_) => match os {
                    "windows" => Path::new(&std::env::var("APPDATA").unwrap_or(".".to_string()))
                        .join(app_name)
                        .join("Config"),
                    "macos" => Path::new(&std::env::var("HOME").unwrap_or(".".to_string()))
                        .join("Library")
                        .join("Preferences")
                        .join(app_name),
                    _ => Path::new(&std::env::var("HOME").unwrap_or(".".to_string()))
                        .join(".config")
                        .join(app_name),
                },
            }),
            data: Directory::new(match std::env::var("XDG_DATA_HOME") {
                Ok(dir) => Path::new(&dir).join(app_name),
                Err(_) => match os {
                    "windows" => Path::new(&std::env::var("APPDATA").unwrap_or(".".to_string()))
                        .join(app_name)
                        .join("Data"),
                    "macos" => Path::new(&std::env::var("HOME").unwrap_or(".".to_string()))
                        .join("Library")
                        .join(app_name),
                    _ => Path::new(&std::env::var("HOME").unwrap_or(".".to_string()))
                        .join(".local")
                        .join("share")
                        .join(app_name),
                },
            }),
        }
    }

    /// Returns a Config with the ability to retrieve standard cache, config
    /// and data paths, and creating / removing these directories.
    ///
    /// ## Arguments
    ///
    /// * `app_name` - The name of your application, to be used to determine the
    /// sub-folder your app cache/config/data belongs in.
    pub fn new(app_name: &str) -> Config {
        Config::_init(app_name, std::env::consts::OS)
    }

    /// Attempts to create all directories. The error produced indicates which
    /// directory failed to be created first, in the order Cache -> Config -> Data.
    pub fn create_all(&self) -> Result<(), CreateError> {
        self.cache.create().map_err(CreateError::Cache)?;
        self.config.create().map_err(CreateError::Config)?;
        self.data.create().map_err(CreateError::Data)?;

        Ok(())
    }

    /// Attemps to remove all directories. The error produced indicates which
    /// directory failed to be removed first, in the order Cache -> Config -> Data.
    pub fn remove_all(&self) -> Result<(), RemoveError> {
        self.cache.remove().map_err(RemoveError::Cache)?;
        self.config.remove().map_err(RemoveError::Config)?;
        self.data.remove().map_err(RemoveError::Data)?;

        Ok(())
    }
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Config {{ cache_dir: {:?}, config_dir: {:?}, data_dir: {:?} }}",
            self.cache.path, self.config.path, self.data.path
        )
    }
}

#[cfg(test)]
mod tests {
    use rand::{distributions::Alphanumeric, Rng};

    use crate::Config;

    macro_rules! path {
        ($path:expr) => {
            $path.replace("/", &std::path::MAIN_SEPARATOR.to_string())
        };
    }

    fn get_random_string() -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(12)
            .map(char::from)
            .collect()
    }

    fn with_windows_config<F>(closure: F)
    where
        F: FnOnce(Config) + Copy,
    {
        temp_env::with_vars(
            [
                ("APPDATA", Some(&path!("/appdata"))),
                ("XDG_CONFIG_HOME", None),
                ("XDG_CACHE_HOME", None),
                ("XDG_DATA_HOME", None),
            ],
            || closure(Config::_init("app-name", "windows")),
        );
    }

    fn with_macos_config<F>(closure: F)
    where
        F: FnOnce(Config) + Copy,
    {
        temp_env::with_vars(
            [
                ("HOME", Some(&path!("/home/user"))),
                ("XDG_CONFIG_HOME", None),
                ("XDG_CACHE_HOME", None),
                ("XDG_DATA_HOME", None),
            ],
            || closure(Config::_init("app-name", "macos")),
        );
    }

    fn with_linux_config<F>(closure: F)
    where
        F: FnOnce(Config) + Copy,
    {
        temp_env::with_vars(
            [
                ("HOME", Some(&path!("/home/user"))),
                ("XDG_CONFIG_HOME", None),
                ("XDG_CACHE_HOME", None),
                ("XDG_DATA_HOME", None),
            ],
            || closure(Config::_init("app-name", "linux")),
        );
    }

    #[test]
    fn it_respects_xdg_cache_home() {
        temp_env::with_vars([("XDG_CACHE_HOME", Some(path!("/tmp/cache")))], || {
            let config = Config::_init("app-name", "any");
            assert_eq!(config.cache.to_string(), path!("/tmp/cache/app-name"));
        });
    }

    #[test]
    fn it_uses_expected_windows_cache_path() {
        with_windows_config(|c| {
            assert_eq!(c.cache.to_string(), path!("/appdata/app-name/Cache"));
        });
    }

    #[test]
    fn it_uses_expected_macos_cache_path() {
        with_macos_config(|c| {
            assert_eq!(
                c.cache.to_string(),
                path!("/home/user/Library/Caches/app-name")
            );
        });
    }

    #[test]
    fn it_uses_expected_linux_cache_path() {
        with_linux_config(|c| {
            assert_eq!(c.cache.to_string(), path!("/home/user/.cache/app-name"));
        });
    }

    #[test]
    fn it_respects_xdg_config_home() {
        temp_env::with_vars([("XDG_CONFIG_HOME", Some(path!("/tmp/config")))], || {
            let config = Config::_init("app-name", "any");
            assert_eq!(config.config.to_string(), path!("/tmp/config/app-name"));
        });
    }

    #[test]
    fn it_uses_expected_windows_config_path() {
        with_windows_config(|c| {
            assert_eq!(c.config.to_string(), path!("/appdata/app-name/Config"));
        });
    }

    #[test]
    fn it_uses_expected_macos_config_path() {
        with_macos_config(|c| {
            assert_eq!(
                c.config.to_string(),
                path!("/home/user/Library/Preferences/app-name")
            );
        });
    }

    #[test]
    fn it_uses_expected_linux_config_path() {
        with_linux_config(|c| {
            assert_eq!(c.config.to_string(), path!("/home/user/.config/app-name"));
        });
    }

    #[test]
    fn it_respects_xdg_data_home() {
        temp_env::with_vars([("XDG_DATA_HOME", Some(path!("/tmp/data")))], || {
            let config = Config::_init("app-name", "any");
            assert_eq!(config.data.to_string(), path!("/tmp/data/app-name"));
        });
    }

    #[test]
    fn it_uses_expected_windows_data_path() {
        with_windows_config(|c| {
            assert_eq!(c.data.to_string(), path!("/appdata/app-name/Data"));
        });
    }

    #[test]
    fn it_uses_expected_macos_data_path() {
        with_macos_config(|c| {
            assert_eq!(c.data.to_string(), path!("/home/user/Library/app-name"));
        });
    }

    #[test]
    fn it_uses_expected_linux_data_path() {
        with_linux_config(|c| {
            assert_eq!(
                c.data.to_string(),
                path!("/home/user/.local/share/app-name")
            );
        });
    }

    #[test]
    fn it_creates_and_removes_directories() {
        temp_env::with_vars_unset(
            ["XDG_CACHE_HOME", "XDG_CONFIG_HOME", "XDG_DATA_HOME"],
            || {
                let config = Config::new(&get_random_string());
                let create_result = config.create_all();
                assert!(create_result.is_ok());
                let remove_result = config.remove_all();
                assert!(remove_result.is_ok());
            },
        );
    }
}
