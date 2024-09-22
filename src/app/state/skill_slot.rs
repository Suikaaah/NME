use crate::app::{misc, state::stats::Stats};

#[derive(Debug, Clone, Copy)]
pub struct SkillSlot(pub usize);

impl SkillSlot {
    pub fn offset(&self) -> u64 {
        const SLOT_0: u64 = 0x0000_0036;
        const STEP: u64 = 0x0000_0002;

        let SkillSlot(index) = *self;

        if index < Stats::SKILL_SLOT_SIZE {
            SLOT_0 + STEP * index as u64
        } else {
            misc::panic_window("skill position out of bounds");
        }
    }
}
