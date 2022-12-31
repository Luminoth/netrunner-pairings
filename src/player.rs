//! Player related structures

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A Netrunner player
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    // this does not have to be a UUID
    id: String,

    first_name: String,
    last_name: String,
    nickname: Option<String>,
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Player {
    /// Create a new player
    #[inline]
    pub fn new(
        first_name: impl Into<String>,
        last_name: impl Into<String>,
        nickname: Option<String>,
    ) -> Self {
        let id = Uuid::new_v4();
        Self {
            id: id.to_string(),
            first_name: first_name.into(),
            last_name: last_name.into(),
            nickname,
        }
    }

    /// Get the player's internal id
    #[inline]
    pub fn get_id(&self) -> &String {
        &self.id
    }

    /// Get the player's first name
    #[inline]
    pub fn get_first_name(&self) -> &String {
        &self.first_name
    }

    /// Get the player's last name
    #[inline]
    pub fn get_last_name(&self) -> &String {
        &self.last_name
    }

    /// Get the player's full name
    #[inline]
    pub fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    /// Get the player's display name
    #[inline]
    pub fn get_nickname(&self) -> String {
        self.nickname
            .clone()
            .unwrap_or_else(|| self.get_full_name())
    }
}
