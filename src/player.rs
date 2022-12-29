//! Player related structures

use std::rc::Rc;

/// A Netrunner player
#[derive(Debug, Clone)]
pub struct Player {
    name: String,
}

impl Player {
    /// Create a new player
    #[inline]
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    /// Get the player's name
    #[inline]
    pub fn get_name(&self) -> &String {
        &self.name
    }
}

/// Handle to a shared player ref
pub type PlayerHandle = Rc<Player>;
