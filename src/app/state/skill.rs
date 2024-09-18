use anyhow::anyhow;

use crate::app::{
    gamemode::{Gamemode, GAMEMODE},
    misc::FancyUnwrap,
};
use std::{borrow::Borrow, fmt::Display};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Skill {
    pub id: u16,
    pub name_vanilla: &'static str,
    pub name_hardtype: &'static str,
}

impl Skill {
    pub const fn new(id: u16, name_vanilla: &'static str, name_hardtype: &'static str) -> Self {
        Self {
            id,
            name_vanilla,
            name_hardtype,
        }
    }
}

impl Display for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lock = GAMEMODE.lock().fancy_unwrap();
        let gamemode: &Gamemode = lock.borrow();
        let s = match *gamemode {
            Gamemode::Vanilla => self.name_vanilla,
            Gamemode::Hardtype => self.name_hardtype,
        };
        write!(f, "{}", s)
    }
}

impl From<u16> for Skill {
    fn from(value: u16) -> Self {
        *SKILL_LIST
            .get(value as usize)
            .ok_or_else(|| anyhow!("skill id out of bound"))
            .fancy_unwrap()
    }
}

pub const SKILL_LIST: &[Skill] = &[
    Skill::new(0x0000, "Blank", "Blank"),
    Skill::new(0x0001, "Agi", "Agi"),
    Skill::new(0x0002, "Agilao", "Agilao"),
    Skill::new(0x0003, "Agidyne", "Agidyne"),
    Skill::new(0x0004, "Maragi", "Maragi"),
    Skill::new(0x0005, "Maragion", "Maragion"),
    Skill::new(0x0006, "Maragidyne", "Maragidyne"),
    Skill::new(0x0007, "Bufu", "Bufu"),
    Skill::new(0x0008, "Bufula", "Bufula"),
    Skill::new(0x0009, "Bufudyne", "Bufudyne"),
    Skill::new(0x000A, "Mabufu", "Mabufu"),
    Skill::new(0x000B, "Mabufula", "Mabufula"),
    Skill::new(0x000C, "Mabufudyne", "Mabufudyne"),
    Skill::new(0x000D, "Zio", "Zio"),
    Skill::new(0x000E, "Zionga", "Zionga"),
    Skill::new(0x000F, "Ziodyne", "Ziodyne"),
    Skill::new(0x0010, "Mazio", "Mazio"),
    Skill::new(0x0011, "Mazionga", "Mazionga"),
    Skill::new(0x0012, "Maziodyne", "Maziodyne"),
    Skill::new(0x0013, "Zan", "Zan"),
    Skill::new(0x0014, "Zanma", "Zanma"),
    Skill::new(0x0015, "Zandyne", "Zandyne"),
    Skill::new(0x0016, "Mazan", "Mazan"),
    Skill::new(0x0017, "Mazanma", "Mazanma"),
    Skill::new(0x0018, "Mazandyne", "Mazandyne"),
    Skill::new(0x0019, "Megido", "Megido"),
    Skill::new(0x001A, "Megidola", "Megidola"),
    Skill::new(0x001B, "Megidolaon", "Megidolaon"),
    Skill::new(0x001C, "Hama", "Hama"),
    Skill::new(0x001D, "Hamaon", "Hamaon"),
    Skill::new(0x001E, "Mahama", "Mahama"),
    Skill::new(0x001F, "Mahamaon", "Mahamaon"),
    Skill::new(0x0020, "Mudo", "Mudo"),
    Skill::new(0x0021, "Mudoon", "Mudoon"),
    Skill::new(0x0022, "Mamudo", "Mamudo"),
    Skill::new(0x0023, "Mamudoon", "Mamudoon"),
    Skill::new(0x0024, "Dia", "Dia"),
    Skill::new(0x0025, "Diarama", "Diarama"),
    Skill::new(0x0026, "Diarahan", "Diarahan"),
    Skill::new(0x0027, "Media", "Media"),
    Skill::new(0x0028, "Mediarama", "Mediarama"),
    Skill::new(0x0029, "Mediarahan", "Mediarahan"),
    Skill::new(0x002A, "Makatora", "Makatora"),
    Skill::new(0x002B, "Patra", "Patra"),
    Skill::new(0x002C, "Me Patra", "Me Patra"),
    Skill::new(0x002D, "Mutudi", "Mutudi"),
    Skill::new(0x002E, "Posumudi", "Posumudi"),
    Skill::new(0x002F, "Paraladi", "Paraladi"),
    Skill::new(0x0030, "Petradi", "Petradi"),
    Skill::new(0x0031, "Recarm", "Recarm"),
    Skill::new(0x0032, "Samrecarm", "Samrecarm"),
    Skill::new(0x0033, "Recarmdra", "Recarmdra"),
    Skill::new(0x0034, "Tarunda", "Tarunda"),
    Skill::new(0x0035, "Sukunda", "Sukunda"),
    Skill::new(0x0036, "Rakunda", "Rakunda"),
    Skill::new(0x0037, "Makajam", "Makajam"),
    Skill::new(0x0038, "Makajamon", "Makajamon"),
    Skill::new(0x0039, "Dekaja", "Dekaja"),
    Skill::new(0x003A, "Reserve", "Dervish"),
    Skill::new(0x003B, "Shibaboo", "Shibaboo"),
    Skill::new(0x003C, "Dormina", "Dormina"),
    Skill::new(0x003D, "Pulinpa", "Pulinpa"),
    Skill::new(0x003E, "Marin Karin", "Marin Karin"),
    Skill::new(0x003F, "Tentarafoo", "Tentarafoo"),
    Skill::new(0x0040, "Tarukaja", "Tarukaja"),
    Skill::new(0x0041, "Sukukaja", "Sukukaja"),
    Skill::new(0x0042, "Rakukaja", "Rakukaja"),
    Skill::new(0x0043, "Makakaja", "Cadenza"),
    Skill::new(0x0044, "Tetraja", "Tetraja"),
    Skill::new(0x0045, "Makarakarn", "Makarakarn"),
    Skill::new(0x0046, "Tetrakarn", "Tetrakarn"),
    Skill::new(0x0047, "Analyze", "Analyze"),
    Skill::new(0x0048, "Trafuri", "Trafuri"),
    Skill::new(0x0049, "Estoma", "Estoma"),
    Skill::new(0x004A, "Riberama", "Riberama"),
    Skill::new(0x004B, "Liftoma", "Liftoma"),
    Skill::new(0x004C, "Lightoma", "Lightoma"),
    Skill::new(0x004D, "Dekunda", "Dekunda"),
    Skill::new(0x004E, "Soul Drain", "Soul Drain"),
    Skill::new(0x004F, "Pestilence", "Pestilence"),
    Skill::new(0x0050, "0x050", "0x050"),
    Skill::new(0x0051, "0x051", "0x051"),
    Skill::new(0x0052, "0x052", "0x052"),
    Skill::new(0x0053, "0x053", "0x053"),
    Skill::new(0x0054, "0x054", "0x054"),
    Skill::new(0x0055, "0x055", "0x055"),
    Skill::new(0x0056, "0x056", "0x056"),
    Skill::new(0x0057, "0x057", "0x057"),
    Skill::new(0x0058, "0x058", "0x058"),
    Skill::new(0x0059, "0x059", "Judecca Tomb"),
    Skill::new(0x005A, "0x05A", "Poison Arrow"),
    Skill::new(0x005B, "0x05B", "Weeping Heaven"),
    Skill::new(0x005C, "0x05C", "0x05C"),
    Skill::new(0x005D, "0x05D", "0x05D"),
    Skill::new(0x005E, "0x05E", "0x05E"),
    Skill::new(0x005F, "0x05F", "0x05F"),
    Skill::new(0x0060, "Lunge", "Lunge"),
    Skill::new(0x0061, "Hell Thrust", "Hell Thrust"),
    Skill::new(0x0062, "Berserk", "Berserk"),
    Skill::new(0x0063, "Tempest", "Tempest"),
    Skill::new(0x0064, "Hades Blast", "Hades Blast"),
    Skill::new(0x0065, "Heat Wave", "Heat Wave"),
    Skill::new(0x0066, "Blight", "Blight"),
    Skill::new(0x0067, "Brutal Slash", "Brutal Slash"),
    Skill::new(0x0068, "Hassohappa", "Hassohappa"),
    Skill::new(0x0069, "Dark Sword", "Dark Sword"),
    Skill::new(0x006A, "Stasis Blade", "Stasis Blade"),
    Skill::new(0x006B, "Mighty Gust", "Mighty Gust"),
    Skill::new(0x006C, "Deathbound", "Deathbound"),
    Skill::new(0x006D, "Guillotine", "Guillotine"),
    Skill::new(0x006E, "Chaos Blade", "Chaos Blade"),
    Skill::new(0x006F, "Needle Rush", "Needle Rush"),
    Skill::new(0x0070, "Stun Needle", "Stun Needle"),
    Skill::new(0x0071, "Toxic Sting", "Toxic Sting"),
    Skill::new(0x0072, "Arid Needle", "Arid Needle"),
    Skill::new(0x0073, "Sacrifice", "Sacrifice"),
    Skill::new(0x0074, "Kamikaze", "Kamikaze"),
    Skill::new(0x0075, "Feral Bite", "Feral Bite"),
    Skill::new(0x0076, "Venom Bite", "Venom Bite"),
    Skill::new(0x0077, "Charm Bite", "Charm Bite"),
    Skill::new(0x0078, "Stone Bite", "Stone Bite"),
    Skill::new(0x0079, "Stun Bite", "Stun Bite"),
    Skill::new(0x007A, "Hell Fang", "Hell Fang"),
    Skill::new(0x007B, "Feral Claw", "Feral Claw"),
    Skill::new(0x007C, "Venom Claw", "Venom Claw"),
    Skill::new(0x007D, "Stun Claw", "Stun Claw"),
    Skill::new(0x007E, "Iron Claw", "Iron Claw"),
    Skill::new(0x007F, "Godly Light", "Starlight"),
    Skill::new(0x0080, "Reserve", "Skull Cleave"),
    Skill::new(0x0081, "Heat Riser", "Heat Riser"),
    Skill::new(0x0082, "Antichton", "Antichton"),
    Skill::new(0x0083, "Deadly Fury", "Deadly Fury"),
    Skill::new(0x0084, "Reserve", "Fang Breaker"),
    Skill::new(0x0085, "Xeros-Beat", "Javelin Rain"),
    Skill::new(0x0086, "Reserve", "Last Word"),
    Skill::new(0x0087, "Reserve", "Chi Blast"),
    Skill::new(0x0088, "Divine Shot", "Divine Shot"),
    Skill::new(0x0089, "Attack", "Luster Candy"),
    Skill::new(0x008A, "Attack", "Attack"),
    Skill::new(0x008B, "Attack", "Primal Force"),
    Skill::new(0x008C, "Attack", "Attack"),
    Skill::new(0x008D, "Reserve", "Gate of Hell"),
    Skill::new(0x008E, "Reserve", "Mjolnir"),
    Skill::new(0x008F, "Javelin Rain", "Xeros-Beat"),
    Skill::new(0x0090, "Oni-Kagura", "Oni-Kagura"),
    Skill::new(0x0091, "Reserve", "Silent Prayer"),
    Skill::new(0x0092, "Reserve", "Smite"),
    Skill::new(0x0093, "Freikugel", "Freikugel"),
    Skill::new(0x0094, "Reserve", "Cocytus"),
    Skill::new(0x0095, "Reserve", "Panta Rhei"),
    Skill::new(0x0096, "Reserve", "God's Curse"),
    Skill::new(0x0097, "Reserve", "Akasha Arts"),
    Skill::new(0x0098, "Last Resort", "Last Resort"),
    Skill::new(0x0099, "Foul Havoc", "Foul Havoc"),
    Skill::new(0x009A, "Reserve", "Amrita"),
    Skill::new(0x009B, "Earthquake", "Earthquake"),
    Skill::new(0x009C, "0x09C", "Chill"),
    Skill::new(0x009D, "0x09D", "Zap"),
    Skill::new(0x009E, "Holy Melody", "Holy Melody"),
    Skill::new(0x009F, "Evil Melody", "Evil Melody"),
    Skill::new(0x00A0, "Spiral Viper", "Spiral Viper"),
    Skill::new(0x00A1, "Magma Axis", "Magma Axis"),
    Skill::new(0x00A2, "0x0A2", "Sakura Rage"),
    Skill::new(0x00A3, "Gaea Rage", "Gaea Rage"),
    Skill::new(0x00A4, "Counter", "Counter"),
    Skill::new(0x00A5, "Retaliate", "Retaliate"),
    Skill::new(0x00A6, "Avenge", "Avenge"),
    Skill::new(0x00A7, "Extra Hit", "Extra Hit"),
    Skill::new(0x00A8, "Tentacle", "Tentacle"),
    Skill::new(0x00A9, "Attack", "Attack"),
    Skill::new(0x00AA, "Attack", "Attack"),
    Skill::new(0x00AB, "Attack", "Attack"),
    Skill::new(0x00AC, "Attack", "Attack"),
    Skill::new(0x00AD, "Attack", "Attack"),
    Skill::new(0x00AE, "Attack", "Attack"),
    Skill::new(0x00AF, "Attack", "Attack"),
    Skill::new(0x00B0, "Fire Breath", "Fire Breath"),
    Skill::new(0x00B1, "Hellfire", "Hellfire"),
    Skill::new(0x00B2, "Prominence", "Prominence"),
    Skill::new(0x00B3, "Ragnarok", "Ragnarok"),
    Skill::new(0x00B4, "Ice Breath", "Ice Breath"),
    Skill::new(0x00B5, "Glacial Blast", "Glacial Blast"),
    Skill::new(0x00B6, "Shock", "Shock"),
    Skill::new(0x00B7, "Bolt Storm", "Bolt Storm"),
    Skill::new(0x00B8, "Wing Buffet", "Wing Buffet"),
    Skill::new(0x00B9, "Tornado", "Tornado"),
    Skill::new(0x00BA, "Wind Cutter", "Wind Cutter"),
    Skill::new(0x00BB, "Wet Wind", "Vayavya"),
    Skill::new(0x00BC, "Thunderclap", "Thunderclap"),
    Skill::new(0x00BD, "Holy Wrath", "Holy Wrath"),
    Skill::new(0x00BE, "Deathtouch", "Deathtouch"),
    Skill::new(0x00BF, "Mana Drain", "Mana Drain"),
    Skill::new(0x00C0, "Life Drain", "Life Drain"),
    Skill::new(0x00C1, "Violet Flash", "Mjolnir"),
    Skill::new(0x00C2, "Starlight", "Thunder Reign"),
    Skill::new(0x00C3, "Radiance", "Radiance"),
    Skill::new(0x00C4, "Hell Gaze", "Hell Gaze"),
    Skill::new(0x00C5, "Stone Gaze", "Stone Gaze"),
    Skill::new(0x00C6, "Mute Gaze", "Mute Gaze"),
    Skill::new(0x00C7, "Evil Gaze", "Evil Gaze"),
    Skill::new(0x00C8, "Reserve", "Bloodbath"),
    Skill::new(0x00C9, "Bael's Bane", "Bael's Bane"),
    Skill::new(0x00CA, "Toxic Cloud", "Toxic Cloud"),
    Skill::new(0x00CB, "War Cry", "War Cry"),
    Skill::new(0x00CC, "Fog Breath", "Fog Breath"),
    Skill::new(0x00CD, "Taunt", "Taunt"),
    Skill::new(0x00CE, "Debilitate", "Debilitate"),
    Skill::new(0x00CF, "Dismal Tune", "Dismal Tune"),
    Skill::new(0x00D0, "Sol Niger", "Sol Niger"),
    Skill::new(0x00D1, "Stun Gaze", "Stun Gaze"),
    Skill::new(0x00D2, "Lullaby", "Lullaby"),
    Skill::new(0x00D3, "Binding Cry", "Binding Cry"),
    Skill::new(0x00D4, "Eternal Rest", "Eternal Rest"),
    Skill::new(0x00D5, "Sonic Wave", "Sonic Wave"),
    Skill::new(0x00D6, "Sexy Gaze", "Sexy Gaze"),
    Skill::new(0x00D7, "Allure", "Allure"),
    Skill::new(0x00D8, "Panic Voice", "Panic Voice"),
    Skill::new(0x00D9, "Intoxicate", "Intoxicate"),
    Skill::new(0x00DA, "Prayer", "Prayer"),
    Skill::new(0x00DB, "Beast Eye", "Beast Eye"),
    Skill::new(0x00DC, "Dragon Eye", "Dragon Eye"),
    Skill::new(0x00DD, "Infinite Light", "Infinite Light"),
    Skill::new(0x00DE, "Summon", "Carnal Winds"),
    Skill::new(0x00DF, "Beckon Call", "Beckon Call"),
    Skill::new(0x00E0, "Focus", "Focus"),
    Skill::new(0x00E1, "Retreat", "Retreat"),
    Skill::new(0x00E2, "Gathering", "Gathering"),
    Skill::new(0x00E3, "Dark Howl", "Dark Howl"),
    Skill::new(0x00E4, "Reserve", "Phlegethon"),
    Skill::new(0x00E5, "Laughter", "Laughter"),
    Skill::new(0x00E6, "Divine Will", "Divine Will"),
    Skill::new(0x00E7, "Reserve", "Sol Niger"),
    Skill::new(0x00E8, "Foul Union", "Foul Union"),
    Skill::new(0x00E9, "Replicate", "Replicate"),
    Skill::new(0x00EA, "Aurora", "Aurora"),
    Skill::new(0x00EB, "Fire of Sinai", "Fire of Sinai"),
    Skill::new(0x00EC, "Conjuration", "Conjuration"),
    Skill::new(0x00ED, "Reserve", "Reserve"),
    Skill::new(0x00EE, "Reserve", "Reserve"),
    Skill::new(0x00EF, "Reserve", "Reserve"),
    Skill::new(0x00F0, "Reserve", "Reserve"),
    Skill::new(0x00F1, "Vast Light", "Vast Light"),
    Skill::new(0x00F2, "God's Curse", "God's Curse"),
    Skill::new(0x00F3, "Hell's Call", "Hell's Call"),
    Skill::new(0x00F4, "Icy Death", "Niflheim"),
    Skill::new(0x00F5, "Mirage", "Mirage"),
    Skill::new(0x00F6, "Reserve", "Reserve"),
    Skill::new(0x00F7, "Lost Macca!", "Lost Macca!"),
    Skill::new(0x00F8, "Reserve", "Reserve"),
    Skill::new(0x00F9, "Wild Dance", "Wild Dance"),
    Skill::new(0x00FA, "Domination", "Domination"),
    Skill::new(0x00FB, "Samrecarm", "Samrecarm"),
    Skill::new(0x00FC, "Gathering", "Gathering"),
    Skill::new(0x00FD, "Apocalypse", "Apocalypse"),
    Skill::new(0x00FE, "Phase Shift", "Omnipotence"),
    Skill::new(0x00FF, "(Summon)", "(Summon)"),
    Skill::new(0x0100, "Attack", "Attack"),
    Skill::new(0x0101, "Fire of Sinai", "Fire of Sinai"),
    Skill::new(0x0102, "Attack", "Attack"),
    Skill::new(0x0103, "Death Flies", "Death Flies"),
    Skill::new(0x0104, "Death Flies", "Death Flies"),
    Skill::new(0x0105, "Soul Divide", "Soul Divide"),
    Skill::new(0x0106, "E & I", "E & I"),
    Skill::new(0x0107, "Rebellion", "Rebellion"),
    Skill::new(0x0108, "Bullet-Time", "Bullet-Time"),
    Skill::new(0x0109, "Provoke", "Provoke"),
    Skill::new(0x010A, "Stinger", "Stinger"),
    Skill::new(0x010B, "Roundtrip", "Roundtrip"),
    Skill::new(0x010C, "Whirlwind", "Whirlwind"),
    Skill::new(0x010D, "Showtime", "Showtime"),
    Skill::new(0x010E, "Attack", "Attack"),
    Skill::new(0x010F, "Evil Gleam", "Evil Gleam"),
    Skill::new(0x0110, "Root of Evil", "Root of Evil"),
    Skill::new(0x0111, "High King", "High King"),
    Skill::new(0x0112, "Holy Star", "Holy Star"),
    Skill::new(0x0113, "Andalucia", "Andalucia"),
    Skill::new(0x0114, "Red Capote", "Red Capote"),
    Skill::new(0x0115, "Startle", "Startle"),
    Skill::new(0x0116, "Preach", "Preach"),
    Skill::new(0x0117, "Meditation", "Meditation"),
    Skill::new(0x0118, "Terrorblade", "Terrorblade"),
    Skill::new(0x0119, "Hell Spin", "Hell Spin"),
    Skill::new(0x011A, "Hell Exhaust", "Hell Exhaust"),
    Skill::new(0x011B, "Hell Burner", "Scald"),
    Skill::new(0x011C, "Hell Throttle", "Hell Throttle"),
    Skill::new(0x011D, "Death Lust", "Babylon Goblet"),
    Skill::new(0x011E, "Beast Roar", "Death Lust"),
    Skill::new(0x011F, "God's Bow", "God's Bow"),
    Skill::new(0x0120, "Reserve", "Reserve"),
    Skill::new(0x0121, "Reserve", "Reserve"),
    Skill::new(0x0122, "Life Bonus", "Life Bonus"),
    Skill::new(0x0123, "Life Gain", "Life Gain"),
    Skill::new(0x0124, "Life Surge", "Life Surge"),
    Skill::new(0x0125, "Mana Bonus", "Mana Bonus"),
    Skill::new(0x0126, "Mana Gain", "Mana Gain"),
    Skill::new(0x0127, "Mana Surge", "Mana Surge"),
    Skill::new(0x0128, "Fast Retreat", "Fast Retreat"),
    Skill::new(0x0129, "Reserve", "Reserve"),
    Skill::new(0x012A, "Mind's Eye", "Mind's Eye"),
    Skill::new(0x012B, "Might", "Might"),
    Skill::new(0x012C, "Bright Might", "Bright Might"),
    Skill::new(0x012D, "Dark Might", "Dark Might"),
    Skill::new(0x012E, "Drain Attack", "Drain Attack"),
    Skill::new(0x012F, "Reserve", "Reserve"),
    Skill::new(0x0130, "Attack All", "Attack All"),
    Skill::new(0x0131, "Counter", "Counter"),
    Skill::new(0x0132, "Retaliate", "Retaliate"),
    Skill::new(0x0133, "Avenge", "Avenge"),
    Skill::new(0x0134, "Reserve", "Reserve"),
    Skill::new(0x0135, "Fire Boost", "Fire Boost"),
    Skill::new(0x0136, "Ice Boost", "Ice Boost"),
    Skill::new(0x0137, "Elec Boost", "Elec Boost"),
    Skill::new(0x0138, "Force Boost", "Force Boost"),
    Skill::new(0x0139, "Anti-Phys", "Anti-Phys"),
    Skill::new(0x013A, "Anti-Fire", "Anti-Fire"),
    Skill::new(0x013B, "Anti-Ice", "Anti-Ice"),
    Skill::new(0x013C, "Anti-Elec", "Anti-Elec"),
    Skill::new(0x013D, "Anti-Force", "Anti-Force"),
    Skill::new(0x013E, "Anti-Expel", "Anti-Expel"),
    Skill::new(0x013F, "Anti-Death", "Anti-Death"),
    Skill::new(0x0140, "Anti-Curse", "Anti-Curse"),
    Skill::new(0x0141, "Anti-Nerve", "Anti-Nerve"),
    Skill::new(0x0142, "Anti-Mind", "Anti-Mind"),
    Skill::new(0x0143, "Void Phys", "Void Phys"),
    Skill::new(0x0144, "Void Fire", "Void Fire"),
    Skill::new(0x0145, "Void Ice", "Void Ice"),
    Skill::new(0x0146, "Void Elec", "Void Elec"),
    Skill::new(0x0147, "Void Force", "Void Force"),
    Skill::new(0x0148, "Void Expel", "Void Expel"),
    Skill::new(0x0149, "Void Death", "Void Death"),
    Skill::new(0x014A, "Void Curse", "Void Curse"),
    Skill::new(0x014B, "Void Nerve", "Void Nerve"),
    Skill::new(0x014C, "Void Mind", "Void Mind"),
    Skill::new(0x014D, "Phys Drain", "Phys Drain"),
    Skill::new(0x014E, "Fire Drain", "Fire Drain"),
    Skill::new(0x014F, "Ice Drain", "Ice Drain"),
    Skill::new(0x0150, "Elec Drain", "Elec Drain"),
    Skill::new(0x0151, "Force Drain", "Force Drain"),
    Skill::new(0x0152, "Phys Repel", "Phys Repel"),
    Skill::new(0x0153, "Fire Repel", "Fire Repel"),
    Skill::new(0x0154, "Ice Repel", "Ice Repel"),
    Skill::new(0x0155, "Elec Repel", "Elec Repel"),
    Skill::new(0x0156, "Force Repel", "Force Repel"),
    Skill::new(0x0157, "Reserve", "Reserve"),
    Skill::new(0x0158, "Reserve", "Reserve"),
    Skill::new(0x0159, "Endure", "Endure"),
    Skill::new(0x015A, "Life Aid", "Life Aid"),
    Skill::new(0x015B, "Mana Aid", "Mana Aid"),
    Skill::new(0x015C, "Victory Cry", "Victory Cry"),
    Skill::new(0x015D, "Life Refill", "Life Refill"),
    Skill::new(0x015E, "Mana Refill", "Mana Refill"),
    Skill::new(0x015F, "Thief's Lore", "Thief's Lore"),
    Skill::new(0x0160, "Reserve", "Reserve"),
    Skill::new(0x0161, "Lucky Find", "Lucky Find"),
    Skill::new(0x0162, "Watchful", "Watchful"),
    Skill::new(0x0163, "Charisma", "Charisma"),
    Skill::new(0x0164, "Reserve", "Reserve"),
    Skill::new(0x0165, "Pierce", "Pierce"),
    Skill::new(0x0166, "0x166", "0x166"),
    Skill::new(0x0167, "0x167", "0x167"),
    Skill::new(0x0168, "Never Yield", "Never Yield"),
    Skill::new(0x0169, "Son's Oath", "Son's Oath"),
    Skill::new(0x016A, "0x16A", "ic"),
    Skill::new(0x016B, "0x16B", "Anti-Magic"),
    Skill::new(0x016C, "0x16C", "ments"),
    Skill::new(0x016D, "0x16D", "Anti-Ailments"),
    Skill::new(0x016E, "0x16E", "ments"),
    Skill::new(0x016F, "Void Ailments", "Void Ailments"),
    Skill::new(0x0170, "0x170", "Will"),
    Skill::new(0x0171, "0x171", "Unshaken Will"),
    Skill::new(0x0172, "0x172", "nn"),
    Skill::new(0x0173, "0x173", "Laevateinn"),
    Skill::new(0x0174, "0x174", "Odinson"),
    Skill::new(0x0175, "0x175", "0x175"),
    Skill::new(0x0176, "0x176", "ost"),
    Skill::new(0x0177, "0x177", "Magic Boost"),
    Skill::new(0x0178, "0x178", "ter"),
    Skill::new(0x0179, "0x179", "Arms Master"),
    Skill::new(0x017A, "0x17A", "0x17A"),
    Skill::new(0x017B, "0x17B", "0x17B"),
    Skill::new(0x017C, "0x17C", "0x17C"),
    Skill::new(0x017D, "0x17D", "0x17D"),
    Skill::new(0x017E, "0x17E", "0x17E"),
    Skill::new(0x017F, "0x17F", "0x17F"),
    Skill::new(0x0180, "Talk", "Talk"),
    Skill::new(0x0181, "Scout", "Scout"),
    Skill::new(0x0182, "Kidnap", "Kidnap"),
    Skill::new(0x0183, "Seduce", "Seduce"),
    Skill::new(0x0184, "Brainwash", "Brainwash"),
    Skill::new(0x0185, "Reserve", "Reserve"),
    Skill::new(0x0186, "Dark Pledge", "Dark Pledge"),
    Skill::new(0x0187, "Wooing", "Wooing"),
    Skill::new(0x0188, "Beseech", "Beseech"),
    Skill::new(0x0189, "Soul Recruit", "Soul Recruit"),
    Skill::new(0x018A, "Mischief", "Mischief"),
    Skill::new(0x018B, "Death Pact", "Death Pact"),
    Skill::new(0x018C, "Pester", "Pester"),
    Skill::new(0x018D, "Begging", "Begging"),
    Skill::new(0x018E, "Threaten", "Threaten"),
    Skill::new(0x018F, "Stone Hunt", "Stone Hunt"),
    Skill::new(0x0190, "Trade", "Trade"),
    Skill::new(0x0191, "Loan", "Loan"),
    Skill::new(0x0192, "Reserve", "Reserve"),
    Skill::new(0x0193, "Reserve", "Reserve"),
    Skill::new(0x0194, "Reserve", "Reserve"),
    Skill::new(0x0195, "Reserve", "Reserve"),
    Skill::new(0x0196, "Reserve", "Reserve"),
    Skill::new(0x0197, "Reserve", "Reserve"),
    Skill::new(0x0198, "Reserve", "Reserve"),
    Skill::new(0x0199, "Haggle", "Haggle"),
    Skill::new(0x019A, "Arbitration", "Arbitration"),
    Skill::new(0x019B, "Detain", "Detain"),
    Skill::new(0x019C, "Gonnection", "Gonnection"),
    Skill::new(0x019D, "Persuade", "Persuade"),
    Skill::new(0x019E, "Intimidate", "Intimidate"),
    Skill::new(0x019F, "Nag", "Nag"),
    Skill::new(0x01A0, "Reserve", "Reserve"),
    Skill::new(0x01A1, "Reserve", "Reserve"),
    Skill::new(0x01A2, "Maiden Plea", "Maiden Plea"),
    Skill::new(0x01A3, "Wine Party", "Wine Party"),
    Skill::new(0x01A4, "Flatter", "Flatter"),
    Skill::new(0x01A5, "Jive Talk", "Jive Talk"),
    Skill::new(0x01A6, "0x1A6", "0x1A6"),
    Skill::new(0x01A7, "0x1A7", "0x1A7"),
    Skill::new(0x01A8, "0x1A8", "0x1A8"),
    Skill::new(0x01A9, "0x1A9", "0x1A9"),
    Skill::new(0x01AA, "0x1AA", "0x1AA"),
    Skill::new(0x01AB, "0x1AB", "0x1AB"),
    Skill::new(0x01AC, "0x1AC", "0x1AC"),
    Skill::new(0x01AD, "0x1AD", "0x1AD"),
    Skill::new(0x01AE, "0x1AE", "0x1AE"),
    Skill::new(0x01AF, "0x1AF", "0x1AF"),
    Skill::new(0x01B0, "0x1B0", "0x1B0"),
    Skill::new(0x01B1, "0x1B1", "0x1B1"),
    Skill::new(0x01B2, "0x1B2", "0x1B2"),
    Skill::new(0x01B3, "0x1B3", "0x1B3"),
    Skill::new(0x01B4, "0x1B4", "0x1B4"),
    Skill::new(0x01B5, "0x1B5", "0x1B5"),
    Skill::new(0x01B6, "0x1B6", "0x1B6"),
    Skill::new(0x01B7, "0x1B7", "0x1B7"),
    Skill::new(0x01B8, "0x1B8", "0x1B8"),
    Skill::new(0x01B9, "0x1B9", "0x1B9"),
    Skill::new(0x01BA, "0x1BA", "0x1BA"),
    Skill::new(0x01BB, "0x1BB", "0x1BB"),
    Skill::new(0x01BC, "0x1BC", "0x1BC"),
    Skill::new(0x01BD, "0x1BD", "0x1BD"),
    Skill::new(0x01BE, "0x1BE", "0x1BE"),
    Skill::new(0x01BF, "0x1BF", "0x1BF"),
    Skill::new(0x01C0, "0x1C0", "0x1C0"),
    Skill::new(0x01C1, "0x1C1", "0x1C1"),
    Skill::new(0x01C2, "0x1C2", "0x1C2"),
    Skill::new(0x01C3, "0x1C3", "0x1C3"),
    Skill::new(0x01C4, "0x1C4", "0x1C4"),
    Skill::new(0x01C5, "0x1C5", "0x1C5"),
    Skill::new(0x01C6, "0x1C6", "0x1C6"),
    Skill::new(0x01C7, "0x1C7", "0x1C7"),
    Skill::new(0x01C8, "0x1C8", "0x1C8"),
    Skill::new(0x01C9, "0x1C9", "0x1C9"),
    Skill::new(0x01CA, "0x1CA", "0x1CA"),
    Skill::new(0x01CB, "0x1CB", "0x1CB"),
    Skill::new(0x01CC, "0x1CC", "0x1CC"),
    Skill::new(0x01CD, "0x1CD", "0x1CD"),
    Skill::new(0x01CE, "0x1CE", "0x1CE"),
    Skill::new(0x01CF, "0x1CF", "0x1CF"),
    Skill::new(0x01D0, "0x1D0", "0x1D0"),
    Skill::new(0x01D1, "0x1D1", "0x1D1"),
    Skill::new(0x01D2, "0x1D2", "0x1D2"),
    Skill::new(0x01D3, "0x1D3", "0x1D3"),
    Skill::new(0x01D4, "0x1D4", "0x1D4"),
    Skill::new(0x01D5, "0x1D5", "0x1D5"),
    Skill::new(0x01D6, "0x1D6", "0x1D6"),
    Skill::new(0x01D7, "0x1D7", "0x1D7"),
    Skill::new(0x01D8, "0x1D8", "0x1D8"),
    Skill::new(0x01D9, "0x1D9", "0x1D9"),
    Skill::new(0x01DA, "0x1DA", "0x1DA"),
    Skill::new(0x01DB, "0x1DB", "0x1DB"),
    Skill::new(0x01DC, "0x1DC", "0x1DC"),
    Skill::new(0x01DD, "0x1DD", "0x1DD"),
    Skill::new(0x01DE, "0x1DE", "0x1DE"),
    Skill::new(0x01DF, "0x1DF", "0x1DF"),
    Skill::new(0x01E0, "0x1E0", "0x1E0"),
    Skill::new(0x01E1, "0x1E1", "0x1E1"),
    Skill::new(0x01E2, "0x1E2", "0x1E2"),
    Skill::new(0x01E3, "0x1E3", "0x1E3"),
    Skill::new(0x01E4, "0x1E4", "0x1E4"),
    Skill::new(0x01E5, "0x1E5", "0x1E5"),
    Skill::new(0x01E6, "0x1E6", "0x1E6"),
    Skill::new(0x01E7, "0x1E7", "0x1E7"),
    Skill::new(0x01E8, "0x1E8", "0x1E8"),
    Skill::new(0x01E9, "0x1E9", "0x1E9"),
    Skill::new(0x01EA, "0x1EA", "0x1EA"),
    Skill::new(0x01EB, "0x1EB", "0x1EB"),
    Skill::new(0x01EC, "0x1EC", "0x1EC"),
    Skill::new(0x01ED, "0x1ED", "0x1ED"),
    Skill::new(0x01EE, "0x1EE", "0x1EE"),
    Skill::new(0x01EF, "0x1EF", "0x1EF"),
    Skill::new(0x01F0, "Call Angel", "Call Angel"),
    Skill::new(0x01F1, "Call Soldier", "Call Soldier"),
    Skill::new(0x01F2, "Call Souls", "Call Souls"),
    Skill::new(0x01F3, "Call Evil", "Call Evil"),
    Skill::new(0x01F4, "0x1F4", "0x1F4"),
    Skill::new(0x01F5, "0x1F5", "0x1F5"),
    Skill::new(0x01F6, "0x1F6", "0x1F6"),
    Skill::new(0x01F7, "0x1F7", "0x1F7"),
    Skill::new(0x01F8, "0x1F8", "0x1F8"),
    Skill::new(0x01F9, "0x1F9", "0x1F9"),
    Skill::new(0x01FA, "0x1FA", "0x1FA"),
    Skill::new(0x01FB, "0x1FB", "0x1FB"),
    Skill::new(0x01FC, "0x1FC", "0x1FC"),
    Skill::new(0x01FD, "0x1FD", "0x1FD"),
    Skill::new(0x01FE, "0x1FE", "Vile"),
    Skill::new(0x01FF, "0x1FF", "Fairy"),
];