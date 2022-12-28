//! Netrunner pairings

mod single_swiss;
mod swiss;

use std::collections::HashSet;

pub use single_swiss::*;
pub use swiss::*;

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
    pub fn name(&self) -> &String {
        &self.name
    }
}

pub trait Pairing {
    /// Determine the next pairing of the given players
    fn next_pair(
        players: impl AsRef<[Player]>,
        pairings: HashSet<(Player, Option<Player>)>,
    ) -> Vec<(Player, Option<Player>)>;
}
