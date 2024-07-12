#[derive(Default)]
pub struct Variable<T> {
  pub value : i32,
  pub locked: bool,
      _t    : std::marker::PhantomData<T>
}

impl<T: TryFrom<i32> + TryInto<i32>> Variable<T> {
  pub fn read(&self) -> Option<T> {
    T::try_from(self.value).ok()
  }

  pub fn read_if_locked(&self) -> Option<T> {
    if self.locked {
      self.read()
    } else {
      None
    }
  }

  pub fn write(&mut self, f: impl Fn(&mut T) -> Result<(), String>)
    -> Result<(), String> {
    if let Some(mut x) = self.read() {
      f(&mut x)?;
      if let Some(y) = x.try_into().ok() {
        self.value = y;
      }
    }

    Ok(())
  }

  pub fn write_if_unlocked(&mut self, f: impl Fn(&mut T) -> Result<(), String>)
    -> Result<(), String> {
    if !self.locked {
      self.write(f)?;
    }

    Ok(())
  }
}

pub const SKILL_SLOT_COUNT: usize = 16;

#[derive(Default)]
pub struct Data {
  pub macca : Variable<u32>,
  pub hp    : Variable<u16>,
  pub max_hp: Variable<u16>,
  pub mp    : Variable<u16>,
  pub max_mp: Variable<u16>,
  pub exp   : Variable<u32>,
  pub level : Variable<u8>,
  pub st    : Variable<u8>,
  pub ma    : Variable<u8>,
  pub vi    : Variable<u8>,
  pub ag    : Variable<u8>,
  pub lu    : Variable<u8>,
  pub skills: [Variable<u16>; SKILL_SLOT_COUNT]
}

impl Data {
  pub fn skill_availability(&self) -> u8 {
    self.skills.iter()
      .position(|x| x.value == 0)
      .unwrap_or(SKILL_SLOT_COUNT) as u8
  }
}