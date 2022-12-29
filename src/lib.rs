//! Netrunner pairings

#![allow(dead_code)]

mod pairing;
mod player;
mod single_swiss;
mod swiss;

use std::cell::RefCell;
use std::collections::HashSet;

pub use pairing::*;
pub use player::*;
pub use single_swiss::*;
pub use swiss::*;

/// Round results
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Results {
    Win,
    Draw,
    Loss,
    Bye,
}

/// Pairing algorithm interface
pub trait PairingAlgorithm {
    /// Creates a new pairing algorithm
    fn new(players: impl AsRef<[PlayerHandle]>) -> Self;

    /// Determine the next pairing of the given players
    fn next_pairings(&self, pairings: &HashSet<Pairing>, round: usize) -> Vec<Pairing>;

    /// Update internal state with round results
    fn round_ended(&self, results: impl AsRef<[(Player, Results)]>);
}

/// Pairings manager
pub struct PairingsManager<T>
where
    T: PairingAlgorithm,
{
    algorithm: T,

    players: Vec<PlayerHandle>,
    pairings: HashSet<Pairing>,
    current_round: RefCell<usize>,
}

impl<T> PairingsManager<T>
where
    T: PairingAlgorithm,
{
    /// Creates a new pairings manager
    pub fn new(players: impl Into<Vec<Player>>) -> Self {
        let players = players
            .into()
            .drain(..)
            .map(PlayerHandle::new)
            .collect::<Vec<_>>();
        let algorithm = T::new(&players);

        Self {
            algorithm,
            players,
            pairings: HashSet::new(),
            current_round: RefCell::new(0),
        }
    }

    /// Gets the current round number
    #[inline]
    pub fn get_current_round(&self) -> usize {
        *self.current_round.borrow()
    }

    /// Determine the next pairing of the given players
    #[inline]
    pub fn next_pairings(&self) -> Vec<Pairing> {
        *self.current_round.borrow_mut() += 1;

        self.algorithm
            .next_pairings(&self.pairings, self.get_current_round())
    }

    /// Update internal state with round results
    #[inline]
    pub fn round_ended(&self, results: impl AsRef<[(Player, Results)]>) {
        self.algorithm.round_ended(results)
    }
}
