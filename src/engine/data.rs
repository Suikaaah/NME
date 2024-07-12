#[derive(Default, Clone)]
pub struct Variable<T> {
  pub value : i32,
  pub locked: bool,
      _pd   : std::marker::PhantomData<T>
}

impl<T: From<i32> + Into<i32>> Variable<T> {
  pub fn read(&self) -> T {
    T::from(self.value)
  }

  pub fn write(&mut self, f: fn(&mut T)) {
    let mut temp = self.read();
    f(&mut temp);
    self.value = temp.into();
  }
}

pub const SKILL_SLOT_COUNT: usize = 16;

#[derive(Default, Clone)]
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
  pub fn skill_availability(&self) -> usize {
    self.skills.iter()
      .position(|x| x.value == 0)
      .unwrap_or(SKILL_SLOT_COUNT) as usize
  }
}