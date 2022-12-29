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

    fn next_pairings(&self, _pairings: &HashSet<Pairing>, _round: usize) -> Vec<Pairing> {
        vec![]
    }

    fn round_ended(&self, _results: impl AsRef<[(Player, Results)]>) {}
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
