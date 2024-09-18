use std::borrow::Borrow;

use crate::app::{
    gamemode::{Gamemode, GAMEMODE},
    input::{drop_down_list::DropDownList, lock_and_text_input::LockAndTextInput},
    misc::FancyUnwrap,
    state::{
        demon::{Demon, DEMON_LIST},
        demon_slot::DemonSlot,
        parameter::Parameter,
        skill::{Skill, SKILL_LIST},
        skill_slot::SkillSlot,
        variable::{Update, Variable},
    },
};
use anyhow::anyhow;
use rand::{rngs::ThreadRng, seq::SliceRandom};
use windows::Win32::Foundation::HANDLE;

#[derive(Debug)]
pub struct Stats {
    pub id: Variable<u16, DropDownList<Demon>>,
    pub hp: Variable<u16, LockAndTextInput>,
    pub max_hp: Variable<u16, LockAndTextInput>,
    pub mp: Variable<u16, LockAndTextInput>,
    pub max_mp: Variable<u16, LockAndTextInput>,
    pub exp: Variable<u32, LockAndTextInput>,
    pub level: Variable<u8, LockAndTextInput>,
    pub st: Variable<u8, LockAndTextInput>,
    pub ma: Variable<u8, LockAndTextInput>,
    pub vi: Variable<u8, LockAndTextInput>,
    pub ag: Variable<u8, LockAndTextInput>,
    pub lu: Variable<u8, LockAndTextInput>,
    pub skill_count: Variable<u8, ()>,
    pub skills: [Variable<u16, DropDownList<Skill>>; Self::SKILL_SLOT_SIZE],
    pub randomize: bool,
}

impl Stats {
    pub const SKILL_SLOT_SIZE: usize = 16;

    pub fn new(slot: DemonSlot, ee_offset: u64) -> Self {
        let demon_list = {
            let mut list = DEMON_LIST.to_vec();
            list.sort_by_key(|demon| demon.name);
            list
        };

        let skill_list = {
            let mut list = SKILL_LIST.to_vec();
            let lock = GAMEMODE.lock().fancy_unwrap();
            let gamemode: &Gamemode = lock.borrow();
            list.sort_by_key(|skill| match *gamemode {
                Gamemode::Vanilla => skill.name_vanilla,
                Gamemode::Hardtype => skill.name_hardtype,
            });
            list
        };

        macro_rules! lati {
            ($field: ident, $label: expr) => {
                Variable::new(
                    Parameter::$field(slot).address(ee_offset),
                    LockAndTextInput::new($label),
                )
            };
        }

        Self {
            id: Variable::new(
                Parameter::Id(slot).address(ee_offset),
                DropDownList::new(Some("ID"), demon_list),
            ),
            hp: lati!(Hp, "HP"),
            max_hp: lati!(MaxHp, "Max HP"),
            mp: lati!(Mp, "MP"),
            max_mp: lati!(MaxMp, "Max MP"),
            exp: lati!(Exp, "EXP"),
            level: lati!(Level, "Level"),
            st: lati!(St, "St"),
            ma: lati!(Ma, "Ma"),
            vi: lati!(Vi, "Vi"),
            ag: lati!(Ag, "Ag"),
            lu: lati!(Lu, "Lu"),
            skill_count: Variable::new(Parameter::SkillCount(slot).address(ee_offset), ()),
            skills: (0..Self::SKILL_SLOT_SIZE)
                .map(|index| {
                    Variable::new(
                        Parameter::Skill(slot, SkillSlot(index)).address(ee_offset),
                        DropDownList::new::<&'static str>(None, skill_list.clone()),
                    )
                })
                .collect::<Vec<_>>()
                .try_into()
                .fancy_unwrap(),
            randomize: false,
        }
    }

    pub fn bulk_update(&mut self, process: HANDLE) {
        self.id.update(process);
        self.hp.update(process);
        self.max_hp.update(process);
        self.mp.update(process);
        self.max_mp.update(process);
        self.exp.update(process);
        self.level.update(process);
        self.st.update(process);
        self.ma.update(process);
        self.vi.update(process);
        self.ag.update(process);
        self.lu.update(process);
        self.skills
            .iter_mut()
            .for_each(|skill| skill.update(process));
    }

    pub fn fix_skill_count(&mut self, process: HANDLE) {
        let dst = &mut self.skill_count;
        dst.set_target(
            self.skills
                .iter_mut()
                .position(|skill| {
                    skill.read(process).fancy_unwrap();
                    skill.get_read() == 0
                })
                .unwrap_or(Stats::SKILL_SLOT_SIZE) as u8,
        );
        dst.write(process).fancy_unwrap();
    }

    pub fn randomize(&mut self, rng: &mut ThreadRng, process: HANDLE) {
        const RANDOMIZE_SLOTS: usize = 8;

        if self.randomize {
            let skills = {
                let mut list = SKILL_LIST[1..].to_vec();
                list.shuffle(rng);
                list
            };
            skills
                .into_iter()
                .take(RANDOMIZE_SLOTS)
                .enumerate()
                .for_each(|(index, skill)| {
                    let dst = self
                        .skills
                        .get_mut(index)
                        .ok_or_else(|| anyhow!("skill index out of bound"))
                        .fancy_unwrap();
                    dst.set_target(skill.id);
                    dst.write(process).fancy_unwrap();
                });
            self.fix_skill_count(process);
        }
    }
}
