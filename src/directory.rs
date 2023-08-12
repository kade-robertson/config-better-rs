use std::{
    fmt::{self, Display},
    path::PathBuf,
};

#[derive(Debug, Clone)]
pub struct Directory {
    pub path: PathBuf,
}

impl Directory {
    pub(crate) fn new(path: PathBuf) -> Directory {
        Directory { path }
    }
}

#[cfg(feature = "sync")]
impl Directory {
    pub fn create(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.path)
    }

    pub fn remove(&self) -> std::io::Result<()> {
        std::fs::remove_dir_all(&self.path)
    }
}

#[cfg(feature = "async")]
use async_std::fs::create_dir_all as create_dir_all_async;

#[cfg(all(not(feature = "async"), feature = "async-tokio"))]
use tokio::fs::create_dir_all as create_dir_all_async;

#[cfg(any(feature = "async", feature = "async-tokio"))]
impl Directory {
    pub async fn create_async(&self) -> std::io::Result<()> {
        create_dir_all_async(&self.path).await
    }

    pub async fn remove_async(&self) -> std::io::Result<()> {
        create_dir_all_async(&self.path).await
    }
}

impl Display for Directory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path.display())
    }
}

#[cfg(test)]
mod tests {
    use rand::{distributions::Alphanumeric, Rng};

    use super::*;

    fn get_random_string() -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(12)
            .map(char::from)
            .collect()
    }

    #[test]
    #[cfg(feature = "sync")]
    fn test_create_remove_sync() {
        let dir = Directory::new(std::env::temp_dir().join(get_random_string()));
        assert!(dir.create().is_ok());
        assert!(dir.remove().is_ok());
    }

    #[cfg(feature = "async-tokio")]
    #[tokio::test]
    async fn test_create_remove_tokio() {
        let dir = Directory::new(std::env::temp_dir().join(get_random_string()));
        assert!(dir.create_async().await.is_ok());
        assert!(dir.remove_async().await.is_ok());
    }

    #[cfg(feature = "async")]
    #[test]
    fn test_create_remove_async_std() {
        let dir = Directory::new(std::env::temp_dir().join(get_random_string()));
        assert!(async_std::task::block_on(dir.create_async()).is_ok());
        assert!(async_std::task::block_on(dir.remove_async()).is_ok());
    }
}
