use crate::app::{misc, state::State};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DemonSlot {
    DemiFiend,
    Party(usize),
}

impl Display for DemonSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use DemonSlot::*;

        match *self {
            DemiFiend => write!(f, "Demi-Fiend"),
            Party(index) => write!(f, "Party #{index}"),
        }
    }
}

impl DemonSlot {
    pub fn offset(&self) -> u64 {
        use DemonSlot::*;

        const PARTY_0: u64 = 0x2124_9308;
        const STEP: u64 = 0x0000_00EC;

        match *self {
            DemiFiend => 0x2124_8F58,
            Party(index) if index < State::PARTY_SIZE => PARTY_0 + STEP * index as u64,
            _ => {
                misc::panic_window("party index out of bounds");
            }
        }
    }
}
