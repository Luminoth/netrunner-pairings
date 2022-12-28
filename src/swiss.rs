//! Swiss pairings
//!
//! https://images-cdn.fantasyflightgames.com/ffg_content/organized-play/support/op-flyer-booklet.pdf
//! https://images-cdn.fantasyflightgames.com/ffg_content/android-netrunner/support/FAQ/Android-Netrunner%20Tournament%20Rules.pdf
//! https://nullsignal.games/wp-content/uploads/2022/11/Null_Signal_Organized_Play_Policies_v1.5.pdf

use std::collections::HashSet;

use crate::{Pairing, Player};

/// Swiss pairing
#[derive(Debug)]
pub struct SwissPairing;

impl Pairing for SwissPairing {
    fn next_pair(
        _players: impl AsRef<[Player]>,
        _pairings: HashSet<(Player, Option<Player>)>,
    ) -> Vec<(Player, Option<Player>)> {
        vec![]
    }
}
