#[path = "./data.rs"]
#[allow(unused)]
mod data;

pub fn skill_address(idx: usize) -> Result<u64, String> {
  if idx < data::SKILL_SLOT_COUNT {
    Ok(0x21248F8E + 2 * (idx as u64))
  } else {
    Err("Invalid skill index".to_string())
  }
}

pub const fn macca()  -> u64 { 0x21248F48 }
pub const fn hp()     -> u64 { 0x21248F5A }
pub const fn max_hp() -> u64 { 0x21248F5C }
pub const fn mp()     -> u64 { 0x21248F5E }
pub const fn max_mp() -> u64 { 0x21248F60 }
pub const fn exp()    -> u64 { 0x21248F64 }
pub const fn level()  -> u64 { 0x21248F68 }
pub const fn st()     -> u64 { 0x21248F6A }
pub const fn ma()     -> u64 { 0x21248F6C }
pub const fn vi()     -> u64 { 0x21248F6D }
pub const fn ag()     -> u64 { 0x21248F6E }
pub const fn lu()     -> u64 { 0x21248F6F }

pub const fn skill_availability() -> u64 { 0x21248F8C }