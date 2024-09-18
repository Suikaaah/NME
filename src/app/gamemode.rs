use lazy_static::lazy_static;
use std::{fmt::Display, sync::Mutex};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Gamemode {
    Vanilla,
    Hardtype,
}

impl Gamemode {
    pub fn overwrite(&mut self, new: Self) {
        *self = new;
    }
}

impl Display for Gamemode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Gamemode::Vanilla => "Vanilla",
            Gamemode::Hardtype => "Hardtype",
        };
        write!(f, "{str}")
    }
}

lazy_static! {
    pub static ref GAMEMODE: Mutex<Gamemode> = Mutex::new(Gamemode::Vanilla);
}
