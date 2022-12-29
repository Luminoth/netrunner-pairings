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

    fn next_pairings(&self, previous_pairings: &HashSet<Pairing>, _round: usize) -> Vec<Pairing> {
        let pairings = vec![];

        if previous_pairings.is_empty() {
            // TODO: shuffle the players and pull random pairings
        }

        pairings
    }

    fn round_ended<'a>(&self, _results: impl AsRef<[(&'a Player, Results)]>) {
        // TODO: ensure the results make sense (each player has 2 games and the pairing results make sense)
    }
}

pub type SwissPairing = PairingsManager<SwissPairingAlgorithm>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_players() {
        let players = vec![Player::new("test a"), Player::new("test b")];
        let pairings = SwissPairing::new(players.clone());

        let first_round = pairings.next_pairings();
        assert_eq!(first_round.len(), 1);
        assert!(
            first_round[0].get_player_a().get_name() == players[0].get_name()
                || first_round[0].get_player_b().unwrap().get_name() == players[0].get_name()
        );

        pairings.round_ended(vec![
            // game 1
            (&players[0], Results::Win),
            (&players[1], Results::Loss),
            // game 2
            (&players[0], Results::Draw),
            (&players[1], Results::Draw),
        ]);
    }
}
