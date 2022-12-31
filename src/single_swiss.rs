//! Single-sided Swiss pairings
//!
//! https://stimhack.com/single-sided-swiss-how-it-works-by-ysengrin/

use crate::{Pairing, PairingAlgorithm, PairingsManager, Player, Result, Round};

#[derive(Debug)]
struct SingleSwissPlayer {
    player_id: String,
}

impl From<&Player> for SingleSwissPlayer {
    #[inline]
    fn from(player: &Player) -> Self {
        Self {
            player_id: player.get_id().clone(),
        }
    }
}

/// Single-sided Swiss pairing
#[derive(Debug, Default)]
pub struct SingleSwissPairingAlgorithm {
    players: Vec<SingleSwissPlayer>,
}

impl PairingAlgorithm for SingleSwissPairingAlgorithm {
    #[inline]
    fn get_total_rounds(&self, _player_count: usize) -> usize {
        todo!()
    }

    fn get_top_cut(&self, _player_count: usize) -> Option<usize> {
        todo!()
    }

    fn next_pairings(
        &self,
        _players: impl AsRef<[Player]>,
        _rounds: impl AsRef<[Round]>,
    ) -> Vec<Pairing> {
        todo!()
    }

    fn round_ended<'a>(&self, _results: impl AsRef<[(&'a Pairing, Result)]>) {
        todo!()
    }
}

pub type SingleSwissPairing = PairingsManager<SingleSwissPairingAlgorithm>;

#[cfg(test)]
mod tests {}
