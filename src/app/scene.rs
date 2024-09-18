pub mod running;
pub mod setup;

use crate::app::state::State;
use setup::Setup;

#[derive(Debug)]
pub enum Scene {
    Setup(Setup),
    Running(Box<State>),
}
