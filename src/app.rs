pub mod gamemode;
pub mod input;
pub mod message;
pub mod misc;
pub mod scene;
pub mod state;

use anyhow::anyhow;
use gamemode::GAMEMODE;
use iced::{executor, window, Application, Command, Element, Size, Subscription, Theme};
use input::lock_and_text_input::LockAndTextInputMessage;
use message::{Message, MessageField};
use misc::FancyUnwrap;
use scene::{setup::Setup, Scene};
use state::State;
use std::{
    borrow::{Borrow, BorrowMut},
    fs::File,
    io::{Read, Write},
    time::Duration,
};

#[derive(Debug)]
pub struct App {
    scene: Scene,
}

impl App {
    pub const SPACING: u16 = 12;
    pub const WINDOW_SIZE_SETUP: Size = Size::new(500., 200.);
    pub const WINDOW_SIZE_RUNNING: Size = Size::new(600., 580.);
    const SUB_INTERVAL: Duration = Duration::from_millis(100);
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        const WINDOW_NAME_TXT: &str = "window_name.txt";
        const DEFAULT_WINDOW_NAME: &str = "Shin Megami Tensei - Nocturne";

        let window_name = match File::open(WINDOW_NAME_TXT) {
            Err(error) => {
                misc::message_box("Error", format!("unable to open window_name.txt ({error})"));
                let mut new = File::create(WINDOW_NAME_TXT).fancy_unwrap();
                new.write(DEFAULT_WINDOW_NAME.as_bytes()).fancy_unwrap();
                DEFAULT_WINDOW_NAME.to_string()
            }
            Ok(mut file) => {
                let mut buf = String::new();
                file.read_to_string(&mut buf).fancy_unwrap();
                buf.trim().to_string()
            }
        };

        (
            App {
                scene: Scene::Setup(Setup {
                    window_name,
                    ..Default::default()
                }),
            },
            Command::none(),
        )
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn title(&self) -> String {
        String::from("Nocturne Memory Editor 1.6")
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(Self::SUB_INTERVAL).map(|_| Message::Subscription)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match &mut self.scene {
            Scene::Setup(setup) => match message {
                Message::SetupWindowName(input) => {
                    setup.window_name = input;
                }
                Message::SetupEEOffset(input) => {
                    setup.ee_offset = input;
                }
                Message::SetupGamemode(gamemode) => {
                    let mut lock = GAMEMODE.lock().fancy_unwrap();
                    lock.borrow_mut().overwrite(gamemode);
                }
                Message::SetupConfirm => match State::new(&setup.window_name, &setup.ee_offset) {
                    Ok(state) => {
                        self.scene = Scene::Running(Box::new(state));
                        return window::resize(window::Id::MAIN, Self::WINDOW_SIZE_RUNNING);
                    }
                    Err(error) => {
                        misc::message_box("Error", error.to_string());
                    }
                },
                _ => {}
            },
            Scene::Running(state) => match message {
                Message::Subscription => {
                    let process = state.process;
                    state.demi_fiend.randomize(&mut state.rng, process);
                    state
                        .party
                        .iter_mut()
                        .for_each(|demon| demon.randomize(&mut state.rng, process));
                    state.bulk_update();
                }
                Message::RunningDemon(demon) => {
                    state.demon_selected = demon;
                }
                Message::RunningLock(field, locked) => {
                    let demon = state.get_demon_mut();

                    macro_rules! update {
                        ($e: expr) => {
                            $e.input.update(LockAndTextInputMessage::Toggled(locked))
                        };
                    }

                    use MessageField::*;
                    match field {
                        Macca => update!(state.macca),
                        Hp => update!(demon.hp),
                        MaxHp => update!(demon.max_hp),
                        Mp => update!(demon.mp),
                        MaxMp => update!(demon.max_mp),
                        Exp => update!(demon.exp),
                        Level => update!(demon.level),
                        St => update!(demon.st),
                        Ma => update!(demon.ma),
                        Vi => update!(demon.vi),
                        Ag => update!(demon.ag),
                        Lu => update!(demon.lu),
                    }
                }
                Message::RunningInput(field, input) => {
                    let process = state.process;
                    let demon = state.get_demon_mut();

                    macro_rules! update {
                        ($e: expr) => {{
                            $e.input.update(LockAndTextInputMessage::Input(input));

                            if let Ok(value) = $e.input.text.parse() {
                                $e.set_target(value);
                                $e.write(process).fancy_unwrap();
                            }
                        }};
                    }

                    use MessageField::*;
                    match field {
                        Macca => update!(state.macca),
                        Hp => update!(demon.hp),
                        MaxHp => update!(demon.max_hp),
                        Mp => update!(demon.mp),
                        MaxMp => update!(demon.max_mp),
                        Exp => update!(demon.exp),
                        Level => update!(demon.level),
                        St => update!(demon.st),
                        Ma => update!(demon.ma),
                        Vi => update!(demon.vi),
                        Ag => update!(demon.ag),
                        Lu => update!(demon.lu),
                    }
                }
                Message::RunningId(demon) => {
                    let process = state.process;
                    let id = &mut state.get_demon_mut().id;
                    id.set_target(demon.id);
                    id.write(process).fancy_unwrap();
                }
                Message::RunningSkill(position, skill) => {
                    let process = state.process;
                    let demon = state.get_demon_mut();
                    let dst = demon
                        .skills
                        .get_mut(position)
                        .ok_or_else(|| anyhow!("skill position out of bounds"))
                        .fancy_unwrap();
                    dst.set_target(skill.id);
                    dst.write(process).fancy_unwrap();
                    demon.fix_skill_count(process);
                }
                Message::RunningRandomize(active) => {
                    state.get_demon_mut().randomize = active;
                }
                _ => {}
            },
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        match &self.scene {
            Scene::Setup(setup) => setup.into(),
            Scene::Running(state) => {
                let state: &State = state.borrow();
                state.into()
            }
        }
    }
}
