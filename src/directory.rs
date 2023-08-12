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
impl Directory {
    pub async fn create_async(&self) -> std::io::Result<()> {
        tokio::fs::create_dir_all(&self.path).await
    }

    pub async fn remove_async(&self) -> std::io::Result<()> {
        tokio::fs::remove_dir_all(&self.path).await
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

    #[tokio::test]
    #[cfg(feature = "async")]
    async fn test_create_remove_async() {
        let dir = Directory::new(std::env::temp_dir().join(get_random_string()));
        assert!(dir.create_async().await.is_ok());
        assert!(dir.remove_async().await.is_ok());
    }
}
