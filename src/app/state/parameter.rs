use crate::app::state::{demon_slot::DemonSlot, skill_slot::SkillSlot};

#[derive(Debug, Clone, Copy)]
pub enum Parameter {
    Macca,
    Id(DemonSlot),
    Hp(DemonSlot),
    MaxHp(DemonSlot),
    Mp(DemonSlot),
    MaxMp(DemonSlot),
    Exp(DemonSlot),
    Level(DemonSlot),
    St(DemonSlot),
    Ma(DemonSlot),
    Vi(DemonSlot),
    Ag(DemonSlot),
    Lu(DemonSlot),
    SkillCount(DemonSlot),
    Skill(DemonSlot, SkillSlot),
}

impl Parameter {
    pub fn address(&self, ee_offset: u64) -> u64 {
        use Parameter::*;

        let pcsx2_1_6_0_address = match *self {
            Macca => 0x2124_8F48,
            Id(demon) => demon.offset(),
            Hp(demon) => demon.offset() + 0x0000_0002,
            MaxHp(demon) => demon.offset() + 0x0000_0004,
            Mp(demon) => demon.offset() + 0x0000_0006,
            MaxMp(demon) => demon.offset() + 0x0000_0008,
            Exp(demon) => demon.offset() + 0x0000_000C,
            Level(demon) => demon.offset() + 0x0000_0010,
            St(demon) => demon.offset() + 0x0000_0012,
            Ma(demon) => demon.offset() + 0x0000_0014,
            Vi(demon) => demon.offset() + 0x0000_0015,
            Ag(demon) => demon.offset() + 0x0000_0016,
            Lu(demon) => demon.offset() + 0x0000_0017,
            SkillCount(demon) => demon.offset() + 0x0000_0034,
            Skill(demon, skill) => demon.offset() + skill.offset(),
        };

        pcsx2_1_6_0_address + ee_offset - 0x2000_0000
    }
}
