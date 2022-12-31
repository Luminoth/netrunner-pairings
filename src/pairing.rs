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
    pub(crate) fn new(player: Player, opponent: Option<Player>) -> Self {
        Self {
            player,
            opponent,
            result: None,
        }
    }

    /// Creates a new pairing from a player slice
    #[inline]
    pub(crate) fn from_slice(players: impl AsRef<[Player]>) -> Self {
        let players = players.as_ref();
        assert!(players.len() == 1 || players.len() == 2);

        Self::new(
            players[0].clone(),
            if players.len() > 1 {
                Some(players[1].clone())
            } else {
                None
            },
        )
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
