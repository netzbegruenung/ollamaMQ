use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

#[derive(Deserialize)]
struct UserEntry {
    token_hash: String,
    user_id: String,
}

#[derive(Deserialize)]
struct UsersFile {
    users: Vec<UserEntry>,
}

/// Registry of authenticated users loaded from `users.yaml`.
///
/// Tokens are stored as their SHA-256 hex digest — plaintext tokens are
/// never persisted on disk.
pub struct UserRegistry {
    /// sha256_hex -> user_id
    map: HashMap<String, String>,
}

impl UserRegistry {
    /// Load the registry from the given YAML file path.
    ///
    /// Returns an empty registry (not an error) when the file does not exist,
    /// which causes every request to be rejected with 401. A warning is logged
    /// by the caller.
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let parsed: UsersFile = serde_yaml::from_str(&content)?;
        let map = parsed
            .users
            .into_iter()
            .map(|e| (e.token_hash.to_lowercase(), e.user_id))
            .collect();
        Ok(Self { map })
    }

    /// Return an empty registry (all requests will be rejected).
    pub fn empty() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Hash `raw_token` with SHA-256 and look it up in the registry.
    ///
    /// Returns `Some(user_id)` if the token is known, `None` otherwise.
    pub fn authenticate(&self, raw_token: &str) -> Option<&str> {
        let digest = Sha256::digest(raw_token.as_bytes());
        let hash: String = digest.iter().map(|b| format!("{:02x}", b)).collect();
        self.map.get(&hash).map(|s| s.as_str())
    }
}
