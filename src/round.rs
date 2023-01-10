//! Game round related structures

use serde::{Deserialize, Serialize};

use crate::{Pairing, Result};

/// Game round container
#[derive(Debug, Serialize, Deserialize)]
pub struct Round {
    pairings: Vec<Pairing>,
}

impl Round {
    /// Create a new round using the given pairings
    #[inline]
    pub(crate) fn new(pairings: impl Into<Vec<Pairing>>) -> Self {
        Self {
            pairings: pairings.into(),
        }
    }

    /// Get the round's pairings
    #[inline]
    pub(crate) fn get_pairings(&self) -> &Vec<Pairing> {
        &self.pairings
    }

    /// Update internal state with round results
    pub(crate) fn round_ended<'a>(&mut self, results: impl AsRef<[(&'a Pairing, Result)]>) {
        // TODO: this sucks, the round should store the pairings
        // in a way that's easier to update them

        for (result_pairing, result) in results.as_ref() {
            for pairing in &mut self.pairings {
                if **result_pairing == *pairing {
                    pairing.update_result(*result);
                    break;
                }
            }
        }
    }
}
