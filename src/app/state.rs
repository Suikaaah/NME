pub mod demon;
pub mod demon_slot;
pub mod parameter;
pub mod skill;
pub mod skill_slot;
pub mod stats;
pub mod variable;

use crate::app::{
    input::lock_and_text_input::LockAndTextInput,
    misc::{self, FancyUnwrap},
};
use anyhow::{anyhow, Result};
use demon_slot::DemonSlot;
use parameter::Parameter;
use rand::{rngs::ThreadRng, thread_rng};
use stats::Stats;
use variable::Update;
use variable::Variable;
use windows::Win32::Foundation::HANDLE;

#[derive(Debug)]
#[allow(unused)]
pub struct State {
    pub process: HANDLE,
    pub macca: Variable<u32, LockAndTextInput>,
    pub demi_fiend: Stats,
    pub party: [Stats; Self::PARTY_SIZE],
    pub demon_selected: DemonSlot,
    pub rng: ThreadRng,
}

impl State {
    pub const PARTY_SIZE: usize = 12;

    pub fn new<W, E>(window_name: W, ee_offset_str: E) -> Result<Self>
    where
        W: AsRef<str>,
        E: AsRef<str>,
    {
        let ee_offset = misc::parse_address(ee_offset_str)?;

        Ok(Self {
            process: misc::get_process(window_name)?,
            macca: Variable::new(
                Parameter::Macca.address(ee_offset),
                LockAndTextInput::new("Macca"),
            ),
            demi_fiend: Stats::new(DemonSlot::DemiFiend, ee_offset),
            party: (0..Self::PARTY_SIZE)
                .map(|index| Stats::new(DemonSlot::Party(index), ee_offset))
                .collect::<Vec<Stats>>()
                .try_into()
                .fancy_unwrap(),
            demon_selected: DemonSlot::DemiFiend,
            rng: thread_rng(),
        })
    }

    pub fn get_demon(&self) -> &Stats {
        match self.demon_selected {
            DemonSlot::DemiFiend => &self.demi_fiend,
            DemonSlot::Party(index) => self
                .party
                .get(index)
                .ok_or_else(|| anyhow!("party index out of bound"))
                .fancy_unwrap(),
        }
    }

    pub fn get_demon_mut(&mut self) -> &mut Stats {
        match self.demon_selected {
            DemonSlot::DemiFiend => &mut self.demi_fiend,
            DemonSlot::Party(index) => self
                .party
                .get_mut(index)
                .ok_or_else(|| anyhow!("party index out of bound"))
                .fancy_unwrap(),
        }
    }

    pub fn bulk_update(&mut self) {
        self.macca.update(self.process);

        self.demi_fiend.bulk_update(self.process);
        self.party
            .iter_mut()
            .for_each(|stats| stats.bulk_update(self.process));
    }
}
