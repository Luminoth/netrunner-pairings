mod single_swiss;
mod swiss;

pub use single_swiss::*;
pub use swiss::*;

#[derive(Debug)]
pub struct Player {
    name: String,
}

impl Player {
    #[inline]
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    #[inline]
    pub fn name(&self) -> &String {
        &self.name
    }
}

pub trait Pairing {
    fn next_pair(players: impl AsRef<[Player]>) -> Vec<(Player, Option<Player>)>;
}
