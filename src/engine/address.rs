#[path = "./data.rs"]
#[allow(unused)]
mod data;

pub fn skill_address(base: u64, idx: usize) -> Result<u64, String> {
    if idx < data::SKILL_SLOT_COUNT {
        let base = 0x21248F8E + base - 0x2000_0000;
        Ok(base + 2 * (idx as u64))
    } else {
        Err("Invalid skill index".to_string())
    }
}

pub fn macca(base: u64) -> u64 {
    0x21248F48 + base - 0x2000_0000
}
pub const fn hp(base: u64) -> u64 {
    0x21248F5A + base - 0x2000_0000
}
pub const fn max_hp(base: u64) -> u64 {
    0x21248F5C + base - 0x2000_0000
}
pub const fn mp(base: u64) -> u64 {
    0x21248F5E + base - 0x2000_0000
}
pub const fn max_mp(base: u64) -> u64 {
    0x21248F60 + base - 0x2000_0000
}
pub const fn exp(base: u64) -> u64 {
    0x21248F64 + base - 0x2000_0000
}
pub const fn level(base: u64) -> u64 {
    0x21248F68 + base - 0x2000_0000
}
pub const fn st(base: u64) -> u64 {
    0x21248F6A + base - 0x2000_0000
}
pub const fn ma(base: u64) -> u64 {
    0x21248F6C + base - 0x2000_0000
}
pub const fn vi(base: u64) -> u64 {
    0x21248F6D + base - 0x2000_0000
}
pub const fn ag(base: u64) -> u64 {
    0x21248F6E + base - 0x2000_0000
}
pub const fn lu(base: u64) -> u64 {
    0x21248F6F + base - 0x2000_0000
}
pub const fn skill_availability(base: u64) -> u64 {
    0x21248F8C + base - 0x2000_0000
}
