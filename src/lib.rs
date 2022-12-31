//! Netrunner pairings

#![allow(dead_code)]

mod pairing;
mod player;
mod round;
mod single_swiss;
mod swiss;

pub use pairing::*;
pub use player::*;
pub use round::*;
pub use single_swiss::*;
pub use swiss::*;

/// Pairing algorithm interface
pub trait PairingAlgorithm: Default {
    /// Gets the number of rounds needed for the given player count
    ///
    /// # Panics
    ///
    /// Panics if player_count is less than 2
    fn get_total_rounds(&self, player_count: usize) -> usize;

    /// Gets the top cut number of players given the player count
    fn get_top_cut(&self, player_count: usize) -> Option<usize>;

    /// Determine the next pairing of the given players
    fn next_pairings(
        &self,
        players: impl AsRef<[Player]>,
        rounds: impl AsRef<[Round]>,
    ) -> Vec<Pairing>;

    /// Update internal state with round results
    fn round_ended<'a>(&self, results: impl AsRef<[(&'a Pairing, Result)]>);
}

/// Pairings manager
#[derive(Debug, Default)]
pub struct PairingsManager<T>
where
    T: PairingAlgorithm,
{
    algorithm: T,

    rounds: Vec<Round>,
}

impl<T> PairingsManager<T>
where
    T: PairingAlgorithm,
{
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
    #[inline]
    pub fn next_round(&mut self, players: impl AsRef<[Player]>) -> Vec<Pairing> {
        let pairings = self.algorithm.next_pairings(players, &self.rounds);

        self.rounds.push(Round::new(pairings.clone()));

        pairings
    }

    /// Update internal state with round results
    #[inline]
    pub fn round_ended<'a>(&mut self, results: impl AsRef<[(&'a Pairing, Result)]>) {
        self.rounds.last_mut().unwrap().round_ended(&results);

        self.algorithm.round_ended(results)
    }
}
