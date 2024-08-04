#[path = "./data.rs"]
#[allow(unused)]
mod data;

const EE_BASE  : u64 = 0x0000_7FF6_4000_0000;
const EE_OFFSET: u64 = EE_BASE - 0x2000_0000;

pub fn skill_address(idx: usize) -> Result<u64, String> {
    if idx < data::SKILL_SLOT_COUNT {
        let base = 0x21248F8E + EE_OFFSET;
        Ok(base + 2 * (idx as u64))
    } else {
        Err("Invalid skill index".to_string())
    }
}

pub const fn macca() -> u64 {
    0x21248F48 + EE_OFFSET
}
pub const fn hp() -> u64 {
    0x21248F5A + EE_OFFSET
}
pub const fn max_hp() -> u64 {
    0x21248F5C + EE_OFFSET
}
pub const fn mp() -> u64 {
    0x21248F5E + EE_OFFSET
}
pub const fn max_mp() -> u64 {
    0x21248F60 + EE_OFFSET
}
pub const fn exp() -> u64 {
    0x21248F64 + EE_OFFSET
}
pub const fn level() -> u64 {
    0x21248F68 + EE_OFFSET
}
pub const fn st() -> u64 {
    0x21248F6A + EE_OFFSET
}
pub const fn ma() -> u64 {
    0x21248F6C + EE_OFFSET
}
pub const fn vi() -> u64 {
    0x21248F6D + EE_OFFSET
}
pub const fn ag() -> u64 {
    0x21248F6E + EE_OFFSET
}
pub const fn lu() -> u64 {
    0x21248F6F + EE_OFFSET
}
pub const fn skill_availability() -> u64 {
    0x21248F8C + EE_OFFSET
}
