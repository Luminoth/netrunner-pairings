use crate::{Pairing, Player};

#[derive(Debug)]
pub struct SingleSwissPairing {}

impl Pairing for SingleSwissPairing {
    fn next_pair(_players: impl AsRef<[Player]>) -> Vec<(Player, Option<Player>)> {
        vec![]
    }
}
