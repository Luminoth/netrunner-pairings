//! Netrunner pairings

#![allow(dead_code)]

mod pairing;
mod player;
mod round;

use std::cmp::Ordering;
use std::collections::HashMap;

use rand::{seq::SliceRandom, thread_rng};

pub use pairing::*;
pub use player::*;
pub use round::*;

/// Swiss style player stats
#[derive(Debug)]
struct SwissStats {
    score: u64,
    rounds: u64,
}

impl SwissStats {
    fn average_points_per_rounds(&self) -> f32 {
        if self.rounds == 0 {
            return 0.0;
        }
        self.score as f32 / self.rounds as f32
    }

    fn strength_of_schedule(&self, player: &Player, rounds: impl AsRef<[Round]>) -> f32 {
        let rounds = rounds.as_ref();
        if rounds.is_empty() {
            return 0.0;
        }

        let mut total = 0.0;
        for round in rounds {
            for pairing in round.get_pairings() {
                if let Some(_opponent) = pairing.get_player_opponent(player) {
                    // TODO: add the opponents average_points_per_rounds to the total
                    total += 0.0;
                }
            }
        }

        total / rounds.len() as f32
    }

    fn extended_strength_of_schedule(&self, player: &Player, rounds: impl AsRef<[Round]>) -> f32 {
        let rounds = rounds.as_ref();

        let mut opponent_count = 0;
        let mut total = 0.0;
        for round in rounds {
            for pairing in round.get_pairings() {
                if let Some(_opponent) = pairing.get_player_opponent(player) {
                    // TODO: add the opponents strength_of_schedule to the total
                    total += 0.0;

                    opponent_count += 1;
                }
            }
        }

        if opponent_count == 0 {
            return 0.0;
        }
        total / opponent_count as f32
    }
}

/// Single-sided swiss player stats
#[derive(Debug)]
struct SingleSwissStats {
    score: u64,
}

#[derive(Debug)]
#[non_exhaustive]
enum PairingsAlgorithm {
    /// Swiss style pairings
    ///
    /// https://en.wikipedia.org/wiki/Swiss-system_tournament
    /// https://images-cdn.fantasyflightgames.com/ffg_content/organized-play/support/op-flyer-booklet.pdf
    /// https://images-cdn.fantasyflightgames.com/ffg_content/android-netrunner/support/FAQ/Android-Netrunner%20Tournament%20Rules.pdf
    /// https://nullsignal.games/wp-content/uploads/2022/11/Null_Signal_Organized_Play_Policies_v1.5.pdf
    Swiss(HashMap<String, SwissStats>),

    /// Single-sided Swiss pairings
    ///
    /// https://stimhack.com/single-sided-swiss-how-it-works-by-ysengrin/
    SingleSwiss(HashMap<String, SingleSwissStats>),
}

impl PairingsAlgorithm {
    /// Creates a new Swiss style pairings
    #[inline]
    fn new_swiss() -> Self {
        Self::Swiss(HashMap::new())
    }

    /// Creates a new Single-sided Swiss pairings
    #[inline]
    fn new_single_swiss() -> Self {
        Self::SingleSwiss(HashMap::new())
    }

    #[inline]
    fn get_total_rounds(&self, player_count: usize) -> usize {
        match self {
            Self::Swiss(_) => match player_count {
                0..=1 => panic!("not enough players"),
                2..=9 => 3,
                10..=32 => 4,
                33..=56 => 5,
                57..=80 => 6,
                81..=128 => 7,
                129..=192 => 7,
                193..=256 => 8,
                _ => 9,
            },
            Self::SingleSwiss(_) => match player_count {
                0..=1 => panic!("not enough players"),
                _ => todo!(),
            },
        }
    }

    #[inline]
    fn get_top_cut(&self, player_count: usize) -> Option<usize> {
        match self {
            Self::Swiss(_) => match player_count {
                0..=1 => panic!("not enough players"),
                2..=15 => None,
                16..=24 => Some(4),
                25..=128 => Some(8),
                _ => Some(16),
            },
            Self::SingleSwiss(_) => match player_count {
                0..=1 => panic!("not enough players"),
                _ => todo!(),
            },
        }
    }

    fn sort_players(&self, players: &mut Vec<Player>, rounds: impl AsRef<[Round]>) {
        let rounds = rounds.as_ref();

        // first round is always random pairing
        if rounds.is_empty() {
            players.shuffle(&mut thread_rng());
            return;
        }

        match self {
            Self::Swiss(stats) => players.sort_by(|x, y| {
                let xs = stats.get(x.get_id()).unwrap();
                let ys = stats.get(y.get_id()).unwrap();

                // order by score first
                let score = xs.score.cmp(&ys.score);
                if score != Ordering::Equal {
                    return score;
                }

                // break ties by sos
                let xsos = xs.strength_of_schedule(x, rounds);
                let ysos = ys.strength_of_schedule(y, rounds);

                let sos = xsos.partial_cmp(&ysos).unwrap();
                if sos != Ordering::Equal {
                    return sos;
                }

                // break further ties by extended sos
                let xesos = xs.extended_strength_of_schedule(x, rounds);
                let yesos = ys.extended_strength_of_schedule(y, rounds);

                let esos = xesos.partial_cmp(&yesos).unwrap();
                if esos != Ordering::Equal {
                    return esos;
                }

                // finally, randomize
                // TODO:
                Ordering::Equal
            }),
            Self::SingleSwiss(_) => todo!(),
        }
    }

    fn next_pairings(&self, mut players: Vec<Player>, rounds: impl AsRef<[Round]>) -> Vec<Pairing> {
        self.sort_players(&mut players, &rounds);

        let mut pairings = vec![];
        match self {
            Self::Swiss(_) => {
                // TODO: if we have odd players, pop off the lowest player as the bye

                // TODO: this isn't right? we need to rank and group players first?
                for pairing in players.chunks(2) {
                    pairings.push(Pairing::from_slice(pairing));
                }
            }
            Self::SingleSwiss(_) => todo!(),
        }

        pairings
    }

    fn round_ended<'a>(&mut self, _results: impl AsRef<[(&'a Pairing, Result)]>) {
        match self {
            Self::Swiss(_) => {
                // TODO: ensure the results make sense (each player has 2 games and the pairing results make sense)

                // TODO: update each player's stats

                todo!()
            }
            Self::SingleSwiss(_) => todo!(),
        }
    }
}

/// Pairings
#[derive(Debug)]
pub struct Pairings {
    algorithm: PairingsAlgorithm,
    rounds: Vec<Round>,
}

impl Pairings {
    /// Creates a new Swiss style pairings
    #[inline]
    pub fn new_swiss() -> Self {
        Self {
            algorithm: PairingsAlgorithm::new_swiss(),
            rounds: vec![],
        }
    }

    /// Creates a new Single-sided Swiss pairings
    #[inline]
    pub fn new_single_swiss() -> Self {
        Self {
            algorithm: PairingsAlgorithm::new_single_swiss(),
            rounds: vec![],
        }
    }

    /// Gets the number of rounds needed for the given player count
    ///
    /// # Panics
    ///
    /// Panics if player_count is less than 2
    #[inline]
    pub fn get_total_rounds(&self, player_count: usize) -> usize {
        self.algorithm.get_total_rounds(player_count)
    }

    /// Gets the top cut number of players given the player count
    #[inline]
    pub fn get_top_cut(&self, player_count: usize) -> Option<usize> {
        self.algorithm.get_top_cut(player_count)
    }

    /// Gets the current round number
    #[inline]
    pub fn get_current_round(&self) -> usize {
        self.rounds.len() + 1
    }

    /// Determine the next pairing of the given players
    pub fn next_round(&mut self, players: impl AsRef<[Player]>) -> Vec<Pairing> {
        let players = players.as_ref().to_owned();
        let pairings = self.algorithm.next_pairings(players, &self.rounds);

        self.rounds.push(Round::new(pairings.clone()));

        pairings
    }

    /// Update internal state with round results
    pub fn round_ended<'a>(&mut self, results: impl AsRef<[(&'a Pairing, Result)]>) {
        self.rounds.last_mut().unwrap().round_ended(&results);

        self.algorithm.round_ended(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn invalid_round_count_0_players() {
        let pairings = Pairings::new_swiss();

        pairings.get_total_rounds(0);
    }

    #[test]
    #[should_panic]
    fn invalid_round_count_1_player() {
        let pairings = Pairings::new_swiss();

        pairings.get_total_rounds(1);
    }

    #[test]
    fn round_counts() {
        let pairings = Pairings::new_swiss();

        assert_eq!(pairings.get_total_rounds(2), 3);
        assert_eq!(pairings.get_total_rounds(9), 3);

        assert_eq!(pairings.get_total_rounds(10), 4);
        assert_eq!(pairings.get_total_rounds(15), 4);
        assert_eq!(pairings.get_total_rounds(16), 4);
        assert_eq!(pairings.get_total_rounds(24), 4);
        assert_eq!(pairings.get_total_rounds(25), 4);
        assert_eq!(pairings.get_total_rounds(32), 4);

        assert_eq!(pairings.get_total_rounds(33), 5);
        assert_eq!(pairings.get_total_rounds(56), 5);

        assert_eq!(pairings.get_total_rounds(57), 6);
        assert_eq!(pairings.get_total_rounds(80), 6);

        assert_eq!(pairings.get_total_rounds(81), 7);
        assert_eq!(pairings.get_total_rounds(128), 7);
        assert_eq!(pairings.get_total_rounds(129), 7);
        assert_eq!(pairings.get_total_rounds(192), 7);

        assert_eq!(pairings.get_total_rounds(193), 8);
        assert_eq!(pairings.get_total_rounds(256), 8);

        assert_eq!(pairings.get_total_rounds(257), 9);
        assert_eq!(pairings.get_total_rounds(usize::MAX), 9);
    }

    #[test]
    #[should_panic]
    fn invalid_top_cut_0_players() {
        let pairings = Pairings::new_swiss();

        pairings.get_top_cut(0);
    }

    #[test]
    #[should_panic]
    fn invalid_top_cut_1_player() {
        let pairings = Pairings::new_swiss();

        pairings.get_top_cut(1);
    }

    #[test]
    fn top_cut() {
        let pairings = Pairings::new_swiss();

        assert_eq!(pairings.get_top_cut(2), None);
        assert_eq!(pairings.get_top_cut(9), None);
        assert_eq!(pairings.get_top_cut(10), None);
        assert_eq!(pairings.get_top_cut(15), None);

        assert_eq!(pairings.get_top_cut(16), Some(4));
        assert_eq!(pairings.get_top_cut(24), Some(4));

        assert_eq!(pairings.get_top_cut(25), Some(8));
        assert_eq!(pairings.get_top_cut(32), Some(8));
        assert_eq!(pairings.get_top_cut(33), Some(8));
        assert_eq!(pairings.get_top_cut(56), Some(8));
        assert_eq!(pairings.get_top_cut(57), Some(8));
        assert_eq!(pairings.get_top_cut(80), Some(8));
        assert_eq!(pairings.get_top_cut(81), Some(8));
        assert_eq!(pairings.get_top_cut(128), Some(8));

        assert_eq!(pairings.get_top_cut(129), Some(16));
        assert_eq!(pairings.get_top_cut(192), Some(16));
        assert_eq!(pairings.get_top_cut(193), Some(16));
        assert_eq!(pairings.get_top_cut(256), Some(16));
        assert_eq!(pairings.get_top_cut(257), Some(16));
        assert_eq!(pairings.get_top_cut(usize::MAX), Some(16));
    }

    #[test]
    fn two_players_first_round() {
        let players = vec![
            Player::new("afirst", "alast", None),
            Player::new("bfirst", "blast", None),
        ];

        let mut pairings = Pairings::new_swiss();

        let first_round = pairings.next_round(&players);
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

        pairings.round_ended(vec![
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

        let mut pairings = Pairings::new_swiss();

        let first_round = pairings.next_round(&players);
        assert_eq!(first_round.len(), 2);
        assert!(first_round[1].get_opponent().is_none());

        pairings.round_ended(vec![
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
