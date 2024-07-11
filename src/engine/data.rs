pub type MaccaType = u32;
pub type HpType    = u16;
pub type MaxHpType = u16;
pub type MpType    = u16;
pub type MaxMpType = u16;
pub type ExpType   = u32;
pub type LevelType = u8;
pub type StType    = u8;
pub type MaType    = u8;
pub type ViType    = u8;
pub type AgType    = u8;
pub type LuType    = u8;
pub type SkillType = u16;

pub const SKILL_SLOT_COUNT: usize = 16;

#[derive(Default)]
pub struct Data {
  pub macca : i32,
  pub hp    : i32,
  pub max_hp: i32,
  pub mp    : i32,
  pub max_mp: i32,
  pub exp   : i32,
  pub level : i32,
  pub st    : i32,
  pub ma    : i32,
  pub vi    : i32,
  pub ag    : i32,
  pub lu    : i32,
  pub skills: [i32; SKILL_SLOT_COUNT]
}

impl Data {
  pub fn skill_availability(&self) -> u8 {
    self.skills.iter()
      .position(|&x| x == 0)
      .unwrap_or(SKILL_SLOT_COUNT) as u8
  }
}

#[derive(Default)]
pub struct Lock {
  pub macca : bool,
  pub hp    : bool,
  pub max_hp: bool,
  pub mp    : bool,
  pub max_mp: bool,
  pub exp   : bool,
  pub level : bool,
  pub st    : bool,
  pub ma    : bool,
  pub vi    : bool,
  pub ag    : bool,
  pub lu    : bool
}