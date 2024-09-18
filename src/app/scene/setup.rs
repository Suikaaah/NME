use crate::app::gamemode::{Gamemode, GAMEMODE};
use crate::app::misc::FancyUnwrap;
use crate::app::{message::Message, App};
use iced::alignment::Horizontal;
use iced::widget::{button, column, pick_list, row, text, text_input, Space};
use iced::{Alignment, Element, Length};
use std::borrow::Borrow;

#[derive(Debug, Default)]
pub struct Setup {
    pub window_name: String,
    pub ee_offset: String,
}

impl From<&Setup> for Element<'static, Message> {
    fn from(value: &Setup) -> Self {
        const TEXT_WIDTH: u16 = 150;
        const BUTTON_WIDTH: u16 = 100;
        const GAMEMODE_LIST: &[Gamemode] = &[Gamemode::Vanilla, Gamemode::Hardtype];
        let lock = GAMEMODE.lock().fancy_unwrap();
        let gamemode: &Gamemode = lock.borrow();

        column!(
            row!(
                text("Window name").width(TEXT_WIDTH),
                text_input("", value.window_name.as_str()).on_input(Message::SetupWindowName),
            )
            .spacing(App::SPACING),
            row!(
                text("EE Main Memory").width(TEXT_WIDTH),
                text_input("Copy and paste from the console", value.ee_offset.as_str())
                    .on_input(Message::SetupEEOffset),
            )
            .spacing(App::SPACING),
            row!(
                text("Gamemode").width(TEXT_WIDTH),
                pick_list(GAMEMODE_LIST, Some(*gamemode), |gamemode| {
                    Message::SetupGamemode(gamemode)
                }),
            )
            .spacing(App::SPACING),
            row!(
                Space::new(Length::Fill, Length::Fill),
                button(text("Launch").horizontal_alignment(Horizontal::Center))
                    .width(BUTTON_WIDTH)
                    .on_press(Message::SetupConfirm)
            )
            .align_items(Alignment::End)
        )
        .padding([App::SPACING, App::SPACING])
        .spacing(App::SPACING)
        .into()
    }
}
