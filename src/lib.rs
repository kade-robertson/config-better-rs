use std::{
    fmt, fs,
    path::{Path, PathBuf},
};

use env::OverridableEnv;

mod env;

pub struct Config {
    cache_dir: PathBuf,
    config_dir: PathBuf,
    data_dir: PathBuf,
}

pub struct CreateRemoveResult {
    pub cache: bool,
    pub config: bool,
    pub data: bool,
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

    /// Returns a Config with the ability to retrieve standard cache, config
    /// and data paths, and creating / removing these directories.
    ///
    /// ## Arguments
    ///
    /// * `app_name` - The name of your application, to be used to determine the
    /// sub-folder your app cache/config/data belongs in.
    pub fn new(app_name: &str) -> Config {
        Config::_init(app_name, std::env::consts::OS, env::OverridableEnv::new())
    }

    /// Returns a &PathBuf pointed at the appropriate directory to use for
    /// storing cache data.
    ///
    /// The expected path as follows for an app called "do-stuff" is this:
    /// * If `$XDG_CACHE_HOME` is defined, this returns `$XDG_CACHE_HOME/do-stuff`
    /// * If the OS is Windows, this returns `%APPDATA%/do-stuff/Cache`
    /// * If the OS is MacOS, this returns `$HOME/Library/Caches/do-stuff`
    /// * Otherwise, this returns `$HOME/.cache/do-stuff`
    pub fn cache(&self) -> &PathBuf {
        &self.cache_dir
    }

    /// Returns a &PathBuf pointed at the appropriate directory to use for
    /// storing configuration data.
    ///
    /// The expected path as follows for an app called "do-stuff" is this:
    /// * If `$XDG_CONFIG_HOME` is defined, this returns `$XDG_CONFIG_HOME/do-stuff`
    /// * If the OS is Windows, this returns `%APPDATA%/do-stuff/Config`
    /// * If the OS is MacOS, this returns `$HOME/Library/Preferences/do-stuff`
    /// * Otherwise, this returns `$HOME/.config/do-stuff`
    pub fn config(&self) -> &PathBuf {
        &self.config_dir
    }

    /// Returns a &PathBuf pointed at the appropriate directory to use for
    /// storing any data (likely that isn't considered cache/config data).
    ///
    /// The expected path as follows for an app called "do-stuff" is this:
    /// * If `$XDG_DATA_HOME` is defined, this returns `$XDG_DATA_HOME/do-stuff`
    /// * If the OS is Windows, this returns `%APPDATA%/do-stuff/Data`
    /// * If the OS is MacOS, this returns `$HOME/Library/do-stuff`
    /// * Otherwise, this returns `$HOME/.local/share/do-stuff`
    pub fn data(&self) -> &PathBuf {
        &self.data_dir
    }

    /// Attempts to create all directories, and returns a CreateRemoveResult
    /// indicating whether each individual directory was successfully created.
    ///
    /// Note: One of the result entries being `true` either indicates this
    /// created the directory, or the directory already existed and thus this
    /// did not need to create the directory again.
    pub fn make_dirs(&self) -> CreateRemoveResult {
        let mut result = CreateRemoveResult {
            cache: true,
            config: true,
            data: true,
        };

        if !&self.cache().exists() {
            match fs::create_dir_all(self.cache_dir.as_path()) {
                Ok(()) => (),
                Err(_) => result.cache = false,
            }
        }

        if !&self.config().exists() {
            match fs::create_dir_all(self.config_dir.as_path()) {
                Ok(()) => (),
                Err(_) => result.config = false,
            }
        }

        if !&self.data().exists() {
            match fs::create_dir_all(self.data_dir.as_path()) {
                Ok(()) => (),
                Err(_) => result.data = false,
            }
        }

        return result;
    }

    /// Attempts to remove all directories, and returns a CreateRemoveResult
    /// indicating whether each individual directory was successfully removed.
    ///
    /// Note: One of the result entries being `true` either indicates this
    /// removed the directory, or the directory already did not exist and thus
    /// this did not need to attempt removal.
    pub fn rm_dirs(&self) -> CreateRemoveResult {
        let mut result = CreateRemoveResult {
            cache: true,
            config: true,
            data: true,
        };

        if self.cache().exists() {
            match fs::remove_dir_all(self.cache_dir.as_path()) {
                Ok(()) => (),
                Err(_) => result.cache = false,
            }
        }

        if self.config().exists() {
            match fs::remove_dir_all(self.config_dir.as_path()) {
                Ok(()) => (),
                Err(_) => result.config = false,
            }
        }

        if self.data().exists() {
            match fs::remove_dir_all(self.data_dir.as_path()) {
                Ok(()) => (),
                Err(_) => result.data = false,
            }
        }

        return result;
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
    use rand::{distributions::Alphanumeric, Rng};
    use std::env;

    use crate::{env::OverridableEnv, Config};

    macro_rules! path {
        ($path:expr) => {
            $path.replace("/", &std::path::MAIN_SEPARATOR.to_string())
        };
    }

    fn clean_xdg_env() {
        env::remove_var("XDG_CACHE_HOME");
        env::remove_var("XDG_CONFIG_HOME");
        env::remove_var("XDG_DATA_HOME");
    }

    fn get_random_string() -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(12)
            .map(char::from)
            .collect()
    }

    fn get_windows_config() -> Config {
        clean_xdg_env();
        let mut env = OverridableEnv::new();
        env.add("APPDATA", &path!("/appdata"));
        Config::_init("app-name", "windows", env)
    }

    fn get_macos_config() -> Config {
        clean_xdg_env();
        let mut env = OverridableEnv::new();
        env.add("HOME", &path!("/home/user"));
        Config::_init("app-name", "macos", env)
    }

    fn get_linux_config() -> Config {
        clean_xdg_env();
        let mut env = OverridableEnv::new();
        env.add("HOME", &path!("/home/user"));
        Config::_init("app-name", "linux", env)
    }

    #[test]
    fn it_respects_xdg_cache_home() {
        let mut env = OverridableEnv::new();
        env.add("XDG_CACHE_HOME", &path!("/tmp/cache"));
        let config = Config::_init("app-name", "any", env);
        assert_eq!(
            config.cache().to_str().unwrap(),
            path!("/tmp/cache/app-name")
        );
    }

    #[test]
    fn it_uses_expected_windows_cache_path() {
        let config = get_windows_config();
        assert_eq!(
            config.cache().to_str().unwrap(),
            path!("/appdata/app-name/Cache")
        );
    }

    #[test]
    fn it_uses_expected_macos_cache_path() {
        let config = get_macos_config();
        assert_eq!(
            config.cache().to_str().unwrap(),
            path!("/home/user/Library/Caches/app-name")
        );
    }

    #[test]
    fn it_uses_expected_linux_cache_path() {
        let config = get_linux_config();
        assert_eq!(
            config.cache().to_str().unwrap(),
            path!("/home/user/.cache/app-name")
        );
    }

    #[test]
    fn it_respects_xdg_config_home() {
        let mut env = OverridableEnv::new();
        env.add("XDG_CONFIG_HOME", &path!("/tmp/config"));
        let config = Config::_init("app-name", "any", env);
        assert_eq!(
            config.config().to_str().unwrap(),
            path!("/tmp/config/app-name")
        );
    }

    #[test]
    fn it_uses_expected_windows_config_path() {
        let config = get_windows_config();
        println!("{:?}", config);
        assert_eq!(
            config.config().to_str().unwrap(),
            path!("/appdata/app-name/Config")
        );
    }

    #[test]
    fn it_uses_expected_macos_config_path() {
        let config = get_macos_config();
        assert_eq!(
            config.config().to_str().unwrap(),
            path!("/home/user/Library/Preferences/app-name")
        );
    }

    #[test]
    fn it_uses_expected_linux_config_path() {
        let config = get_linux_config();
        assert_eq!(
            config.config().to_str().unwrap(),
            path!("/home/user/.config/app-name")
        );
    }

    #[test]
    fn it_respects_xdg_data_home() {
        let mut env = OverridableEnv::new();
        env.add("XDG_DATA_HOME", &path!("/tmp/data"));
        let config = Config::_init("app-name", "any", env);
        assert_eq!(config.data().to_str().unwrap(), path!("/tmp/data/app-name"));
    }

    #[test]
    fn it_uses_expected_windows_data_path() {
        let config = get_windows_config();
        assert_eq!(
            config.data().to_str().unwrap(),
            path!("/appdata/app-name/Data")
        );
    }

    #[test]
    fn it_uses_expected_macos_data_path() {
        let config = get_macos_config();
        assert_eq!(
            config.data().to_str().unwrap(),
            path!("/home/user/Library/app-name")
        );
    }

    #[test]
    fn it_uses_expected_linux_data_path() {
        let config = get_linux_config();
        assert_eq!(
            config.data().to_str().unwrap(),
            path!("/home/user/.local/share/app-name")
        );
    }

    #[test]
    fn it_creates_and_removes_directories() {
        clean_xdg_env();
        let config = Config::new(&get_random_string());
        let create_result = config.make_dirs();
        assert!(create_result.cache);
        assert!(create_result.config);
        assert!(create_result.data);
        let remove_result = config.rm_dirs();
        assert!(remove_result.cache);
        assert!(remove_result.config);
        assert!(remove_result.data);
    }
}
