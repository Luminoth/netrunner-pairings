//! Swiss pairings
//!
//! https://en.wikipedia.org/wiki/Swiss-system_tournament
//! https://images-cdn.fantasyflightgames.com/ffg_content/organized-play/support/op-flyer-booklet.pdf
//! https://images-cdn.fantasyflightgames.com/ffg_content/android-netrunner/support/FAQ/Android-Netrunner%20Tournament%20Rules.pdf
//! https://nullsignal.games/wp-content/uploads/2022/11/Null_Signal_Organized_Play_Policies_v1.5.pdf

use rand::{seq::SliceRandom, thread_rng};

use crate::{Pairing, PairingAlgorithm, PairingsManager, Player, Result, Round};

#[derive(Debug)]
struct SwissPlayer {
    player_id: String,

    strength_of_schedule: f32,
    extended_strength_of_schedule: f32,
}

impl From<&Player> for SwissPlayer {
    #[inline]
    fn from(player: &Player) -> Self {
        Self {
            player_id: player.get_id().clone(),
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
    #[inline]
    fn get_total_rounds(&self, player_count: usize) -> usize {
        match player_count {
            0..=1 => panic!("not enough players"),
            2..=9 => 3,
            10..=32 => 4,
            33..=56 => 5,
            57..=80 => 6,
            81..=128 => 7,
            129..=192 => 7,
            193..=256 => 8,
            _ => 9,
        }
    }

    #[inline]
    fn get_top_cut(&self, player_count: usize) -> Option<usize> {
        match player_count {
            0..=1 => panic!("not enough players"),
            2..=15 => None,
            16..=24 => Some(4),
            25..=128 => Some(8),
            _ => Some(16),
        }
    }

    fn next_pairings(
        &self,
        players: impl AsRef<[Player]>,
        rounds: impl AsRef<[Round]>,
    ) -> Vec<Pairing> {
        let mut players = players.as_ref().to_owned();

        let mut pairings = vec![];

        // first round is always random pairing
        if rounds.as_ref().is_empty() {
            players.shuffle(&mut thread_rng());

            for pairing in players.chunks(2) {
                pairings.push(Pairing::from_slice(pairing));
            }
            return pairings;
        }

        // TODO: do the algorithm

        pairings
    }

    fn round_ended<'a>(&self, _results: impl AsRef<[(&'a Pairing, Result)]>) {
        // TODO: ensure the results make sense (each player has 2 games and the pairing results make sense)
    }
}

pub type SwissPairing = PairingsManager<SwissPairingAlgorithm>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn invalid_round_count_0_players() {
        let algorithm = SwissPairingAlgorithm::default();

        algorithm.get_total_rounds(0);
    }

    #[test]
    #[should_panic]
    fn invalid_round_count_1_player() {
        let algorithm = SwissPairingAlgorithm::default();

        algorithm.get_total_rounds(1);
    }

    #[test]
    fn round_counts() {
        let algorithm = SwissPairingAlgorithm::default();

        assert_eq!(algorithm.get_total_rounds(2), 3);
        assert_eq!(algorithm.get_total_rounds(9), 3);

        assert_eq!(algorithm.get_total_rounds(10), 4);
        assert_eq!(algorithm.get_total_rounds(15), 4);
        assert_eq!(algorithm.get_total_rounds(16), 4);
        assert_eq!(algorithm.get_total_rounds(24), 4);
        assert_eq!(algorithm.get_total_rounds(25), 4);
        assert_eq!(algorithm.get_total_rounds(32), 4);

        assert_eq!(algorithm.get_total_rounds(33), 5);
        assert_eq!(algorithm.get_total_rounds(56), 5);

        assert_eq!(algorithm.get_total_rounds(57), 6);
        assert_eq!(algorithm.get_total_rounds(80), 6);

        assert_eq!(algorithm.get_total_rounds(81), 7);
        assert_eq!(algorithm.get_total_rounds(128), 7);
        assert_eq!(algorithm.get_total_rounds(129), 7);
        assert_eq!(algorithm.get_total_rounds(192), 7);

        assert_eq!(algorithm.get_total_rounds(193), 8);
        assert_eq!(algorithm.get_total_rounds(256), 8);

        assert_eq!(algorithm.get_total_rounds(257), 9);
        assert_eq!(algorithm.get_total_rounds(usize::MAX), 9);
    }

    #[test]
    #[should_panic]
    fn invalid_top_cut_0_players() {
        let algorithm = SwissPairingAlgorithm::default();

        algorithm.get_top_cut(0);
    }

    #[test]
    #[should_panic]
    fn invalid_top_cut_1_player() {
        let algorithm = SwissPairingAlgorithm::default();

        algorithm.get_top_cut(1);
    }

    #[test]
    fn top_cut() {
        let algorithm = SwissPairingAlgorithm::default();

        assert_eq!(algorithm.get_top_cut(2), None);
        assert_eq!(algorithm.get_top_cut(9), None);
        assert_eq!(algorithm.get_top_cut(10), None);
        assert_eq!(algorithm.get_top_cut(15), None);

        assert_eq!(algorithm.get_top_cut(16), Some(4));
        assert_eq!(algorithm.get_top_cut(24), Some(4));

        assert_eq!(algorithm.get_top_cut(25), Some(8));
        assert_eq!(algorithm.get_top_cut(32), Some(8));
        assert_eq!(algorithm.get_top_cut(33), Some(8));
        assert_eq!(algorithm.get_top_cut(56), Some(8));
        assert_eq!(algorithm.get_top_cut(57), Some(8));
        assert_eq!(algorithm.get_top_cut(80), Some(8));
        assert_eq!(algorithm.get_top_cut(81), Some(8));
        assert_eq!(algorithm.get_top_cut(128), Some(8));

        assert_eq!(algorithm.get_top_cut(129), Some(16));
        assert_eq!(algorithm.get_top_cut(192), Some(16));
        assert_eq!(algorithm.get_top_cut(193), Some(16));
        assert_eq!(algorithm.get_top_cut(256), Some(16));
        assert_eq!(algorithm.get_top_cut(257), Some(16));
        assert_eq!(algorithm.get_top_cut(usize::MAX), Some(16));
    }

    #[test]
    fn two_players_first_round() {
        let players = vec![
            Player::new("afirst", "alast", None),
            Player::new("bfirst", "blast", None),
        ];

        let rounds = vec![];

        let algorithm = SwissPairingAlgorithm::default();

        let first_round = algorithm.next_pairings(&players, &rounds);
        assert_eq!(first_round.len(), 1);
        assert!(
            first_round[0].get_player().get_full_name() == players[0].get_full_name()
                || first_round[0]
                    .get_opponent()
                    .as_ref()
                    .unwrap()
                    .get_full_name()
                    == players[0].get_full_name()
        );

        algorithm.round_ended(vec![
            // game 1
            (&first_round[0], Result::Win),
            // game 2
            (&first_round[0], Result::Draw),
        ]);

        // TODO: validate the updated results
    }

    #[test]
    fn three_players_first_round() {
        let players = vec![
            Player::new("afirst", "alast", None),
            Player::new("bfirst", "blast", None),
            Player::new("cfirst", "clast", None),
        ];

        let rounds = vec![];

        let algorithm = SwissPairingAlgorithm::default();

        let first_round = algorithm.next_pairings(&players, &rounds);
        assert_eq!(first_round.len(), 2);
        assert!(first_round[1].get_opponent().is_none());

        algorithm.round_ended(vec![
            // game 1
            (&first_round[0], Result::Win),
            (&first_round[0], Result::Bye),
            // game 2
            (&first_round[0], Result::Draw),
            (&first_round[0], Result::Bye),
        ]);

        // TODO: validate the updated results
    }
}
