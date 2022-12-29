//! Swiss pairings
//!
//! https://images-cdn.fantasyflightgames.com/ffg_content/organized-play/support/op-flyer-booklet.pdf
//! https://images-cdn.fantasyflightgames.com/ffg_content/android-netrunner/support/FAQ/Android-Netrunner%20Tournament%20Rules.pdf
//! https://nullsignal.games/wp-content/uploads/2022/11/Null_Signal_Organized_Play_Policies_v1.5.pdf

use std::collections::HashSet;

use crate::{Pairing, PairingAlgorithm, PairingsManager, Player, PlayerHandle, Results};

#[derive(Debug)]
struct SwissPlayer {
    player: PlayerHandle,

    strength_of_schedule: f32,
    extended_strength_of_schedule: f32,
}

impl From<PlayerHandle> for SwissPlayer {
    #[inline]
    fn from(player: PlayerHandle) -> Self {
        Self {
            player,
            strength_of_schedule: 0.0,
            extended_strength_of_schedule: 0.0,
        }
    }
}

/// Swiss pairing
#[derive(Debug, Default)]
pub struct SwissPairingAlgorithm {
    players: Vec<SwissPlayer>,
}

impl PairingAlgorithm for SwissPairingAlgorithm {
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

pub type SwissPairing = PairingsManager<SwissPairingAlgorithm>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let players = vec![Player::new("test")];
        let _pairings = SwissPairing::new(players);
    }
}
