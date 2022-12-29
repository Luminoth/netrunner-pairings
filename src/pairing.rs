//! Pairing related structures

use crate::{Player, PlayerHandle};

/// A pairing of players for a round
#[derive(Debug)]
pub struct Pairing {
    round: usize,

    player_a: PlayerHandle,
    player_b: Option<PlayerHandle>,
}

impl Pairing {
    /// Creates a new pairing
    #[inline]
    fn new(round: usize, player_a: PlayerHandle, player_b: Option<PlayerHandle>) -> Self {
        Self {
            round,
            player_a,
            player_b,
        }
    }

    /// Gets the round the pairing occurred in
    #[inline]
    pub fn get_round(&self) -> usize {
        self.round
    }

    /// Gets the first player in the pairing
    #[inline]
    pub fn get_player_a(&self) -> &Player {
        self.player_a.as_ref()
    }

    /// Gets the second player in the pairing
    ///
    /// This will be None if the first player had a bye this round
    #[inline]
    pub fn get_player_b(&self) -> Option<&Player> {
        self.player_b.as_deref()
    }
}
