use crate::app::{
    gamemode::Gamemode,
    state::{demon::Demon, demon_slot::DemonSlot, skill::Skill},
};

#[derive(Debug, Clone)]
pub enum MessageField {
    Macca,
    Hp,
    MaxHp,
    Mp,
    MaxMp,
    Exp,
    Level,
    St,
    Ma,
    Vi,
    Ag,
    Lu,
}

#[derive(Debug, Clone)]
pub enum Message {
    Subscription,
    SetupWindowName(String),
    SetupEEOffset(String),
    SetupGamemode(Gamemode),
    SetupConfirm,
    RunningDemon(DemonSlot),
    RunningLock(MessageField, bool),
    RunningInput(MessageField, String),
    RunningId(Demon),
    RunningSkill(usize, Skill),
    RunningRandomize(bool),
}
