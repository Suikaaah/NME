use crate::app::{
    input::{drop_down_list::DropDownList, lock_and_text_input::LockAndTextInput},
    misc::{self, FancyUnwrap},
};
use anyhow::Result;
use std::{fmt::Display, marker::PhantomData, str::FromStr};
use windows::Win32::Foundation::HANDLE;

#[derive(Debug)]
pub struct Variable<T, I> {
    address: u64,
    value_read: T,
    value_target: T,
    pub input: I,
    _marker: PhantomData<T>,
}

impl<T: Default, I> Variable<T, I> {
    pub fn new(address: u64, input: I) -> Self {
        Self {
            address,
            value_read: Default::default(),
            value_target: Default::default(),
            input,
            _marker: PhantomData,
        }
    }
}

impl<T: Copy, I> Variable<T, I> {
    pub fn get_read(&self) -> T {
        self.value_read
    }
}

impl<T, I> Variable<T, I> {
    pub fn read(&mut self, process: HANDLE) -> Result<()> {
        misc::read(process, &mut self.value_read, self.address)
    }

    pub fn write(&self, process: HANDLE) -> Result<()> {
        misc::write(process, &self.value_target, self.address)
    }

    pub fn set_target(&mut self, value: T) {
        self.value_target = value;
    }
}

pub trait Update {
    fn update(&mut self, process: HANDLE);
}

impl<T: Display + FromStr> Update for Variable<T, LockAndTextInput> {
    fn update(&mut self, process: HANDLE) {
        if self.input.locked {
            self.write(process).fancy_unwrap();
        } else {
            self.read(process).fancy_unwrap();

            self.input.text = self.value_read.to_string();
            if let Ok(value) = self.input.text.parse() {
                self.set_target(value);
            }
        }
    }
}

impl<T, U> Update for Variable<T, DropDownList<U>>
where
    T: Clone + Into<U>,
{
    fn update(&mut self, process: HANDLE) {
        self.read(process).fancy_unwrap();
        self.input.selected = Some(self.value_read.clone().into());
    }
}
