//! Pairing related structures

use serde::{Deserialize, Serialize};

use crate::Player;

/// Pairing results
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Result {
    Win,
    Draw,
    Loss,
    Bye,
}

/// A pairing of players for a round
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pairing {
    player: Player,
    opponent: Option<Player>,
    result: Option<Result>,
}

impl PartialEq for Pairing {
    fn eq(&self, other: &Self) -> bool {
        self.player == other.player && self.opponent == other.opponent
    }
}

impl Pairing {
    /// Creates a new pairing
    #[inline]
    pub(crate) fn new(player: Player, opponent: Player) -> Self {
        Self {
            player,
            opponent: Some(opponent),
            result: None,
        }
    }

    /// Creates a new Bye pairing
    #[inline]
    pub(crate) fn new_bye(player: Player) -> Self {
        Self {
            player,
            opponent: None,
            result: None,
        }
    }

    /// Creates a new pairing from a player slice
    #[inline]
    pub(crate) fn from_slice(players: impl AsRef<[Player]>) -> Self {
        let players = players.as_ref();
        assert!(players.len() == 1 || players.len() == 2);

        Self {
            player: players[0].clone(),
            opponent: if players.len() > 1 {
                Some(players[1].clone())
            } else {
                None
            },
            result: None,
        }
    }

    /// Checks if the given player is in this pairing
    #[inline]
    pub fn has_player(&self, player: &Player) -> bool {
        if self.player == *player {
            return true;
        }

        if let Some(opponent) = &self.opponent {
            return *opponent == *player;
        }

        false
    }

    /// Gets the first player in the pairing
    #[inline]
    pub fn get_player(&self) -> &Player {
        &self.player
    }

    /// Gets the second player in the pairing
    ///
    /// This will be None if the first player had a bye this round
    #[inline]
    pub fn get_opponent(&self) -> &Option<Player> {
        &self.opponent
    }

    /// Returns the given player's opponent if they were in this pairing and had an opponent
    #[inline]
    pub fn get_player_opponent(&self, player: &Player) -> Option<&Player> {
        if self.player == *player {
            return self.opponent.as_ref();
        }

        if let Some(opponent) = &self.opponent {
            if *opponent == *player {
                return Some(&self.player);
            }
        }

        None
    }

    /// Gets the pairing result
    ///
    /// This will be None if the game result has not been reported
    #[inline]
    pub fn get_result(&self) -> Option<Result> {
        self.result
    }

    #[inline]
    pub(crate) fn update_result(&mut self, result: Result) {
        self.result = Some(result);
    }
}
