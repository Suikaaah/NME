use crate::app::App;
use iced::{
    widget::{checkbox, row, text_input},
    Element,
};

#[derive(Debug, Default)]
pub struct LockAndTextInput {
    label: String,
    pub locked: bool,
    pub text: String,
}

#[derive(Debug, Clone)]
pub enum LockAndTextInputMessage {
    Toggled(bool),
    Input(String),
}

impl LockAndTextInput {
    pub const CHECKBOX_WIDTH: u16 = 100;

    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            ..Default::default()
        }
    }

    pub fn update(&mut self, message: LockAndTextInputMessage) {
        match message {
            LockAndTextInputMessage::Toggled(locked) => {
                self.locked = locked;
            }
            LockAndTextInputMessage::Input(input) => {
                self.text = input;
            }
        }
    }
}

impl From<&LockAndTextInput> for Element<'static, LockAndTextInputMessage> {
    fn from(value: &LockAndTextInput) -> Self {
        row!(
            checkbox(value.label.as_str(), value.locked)
                .width(LockAndTextInput::CHECKBOX_WIDTH)
                .on_toggle(LockAndTextInputMessage::Toggled),
            text_input("", value.text.as_str()).on_input(LockAndTextInputMessage::Input)
        )
        .spacing(App::SPACING)
        .into()
    }
}
