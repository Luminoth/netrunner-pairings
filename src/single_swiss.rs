//! Single-sided Swiss pairings
//!
//! https://stimhack.com/single-sided-swiss-how-it-works-by-ysengrin/

use std::collections::HashSet;

use crate::{Pairing, Player};

/// Single-sided Swiss pairing
#[derive(Debug)]
pub struct SingleSwissPairing;

impl Pairing for SingleSwissPairing {
    fn next_pair(
        _players: impl AsRef<[Player]>,
        _pairings: HashSet<(Player, Option<Player>)>,
    ) -> Vec<(Player, Option<Player>)> {
        vec![]
    }
}
