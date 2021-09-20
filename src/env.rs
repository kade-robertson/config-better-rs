use std::{collections::HashMap, env::VarError};

pub struct OverridableEnv {
    env_map: HashMap<String, String>,
}

impl OverridableEnv {
    pub fn new() -> OverridableEnv {
        OverridableEnv {
            env_map: HashMap::new(),
        }
    }

    #[cfg(test)]
    pub fn add(&mut self, key: &str, value: &str) {
        self.env_map.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Result<String, VarError> {
        if cfg!(test) && self.env_map.contains_key(&key.to_string()) {
            Ok(self.env_map.get(&key.to_string()).unwrap().clone())
        } else {
            std::env::var(key)
        }
    }
}
