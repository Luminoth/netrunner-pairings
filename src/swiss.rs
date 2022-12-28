use crate::{Pairing, Player};

#[derive(Debug)]
pub struct SwissPairing {}

impl Pairing for SwissPairing {
    fn next_pair(_players: impl AsRef<[Player]>) -> Vec<(Player, Option<Player>)> {
        vec![]
    }
}
