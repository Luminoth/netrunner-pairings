//! Single-sided Swiss pairings
//!
//! https://stimhack.com/single-sided-swiss-how-it-works-by-ysengrin/

use std::collections::HashSet;

use crate::{Pairing, PairingAlgorithm, PairingsManager, Player, PlayerHandle, Results};

#[derive(Debug)]
struct SingleSwissPlayer {
    player: PlayerHandle,
}

impl From<PlayerHandle> for SingleSwissPlayer {
    #[inline]
    fn from(player: PlayerHandle) -> Self {
        Self { player }
    }
}

/// Single-sided Swiss pairing
#[derive(Debug, Default)]
pub struct SingleSwissPairingAlgorithm {
    players: Vec<SingleSwissPlayer>,
}

impl PairingAlgorithm for SingleSwissPairingAlgorithm {
    fn new(players: impl AsRef<[PlayerHandle]>) -> Self {
        let players = players
            .as_ref()
            .iter()
            .map(|player| player.clone().into())
            .collect::<Vec<_>>();
        Self { players }
    }

    fn next_pairings(&self, previous_pairings: &HashSet<Pairing>, _round: usize) -> Vec<Pairing> {
        let pairings = vec![];

        if previous_pairings.is_empty() {
            // TODO: shuffle the players and pull random pairings
        }

        pairings
    }

    fn round_ended<'a>(&self, _results: impl AsRef<[(&'a Player, Results)]>) {
        // TODO: ensure the results make sense (each player has 1 game and the pairing results make sense)
    }
}

pub type SingleSwissPairing = PairingsManager<SingleSwissPairingAlgorithm>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let _pairings = SingleSwissPairing::new(vec![]);
    }
}
