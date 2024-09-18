use crate::app::input::lock_and_text_input::LockAndTextInput;
use crate::app::App;
use iced::widget::{pick_list, row, text};
use iced::Element;
use std::fmt::Display;

#[derive(Debug)]
pub struct DropDownList<T> {
    label: Option<String>,
    pub list: Vec<T>,
    pub selected: Option<T>,
}

#[derive(Debug, Clone)]
pub enum DropDownListMessage<T> {
    Selected(T),
}

impl<T> DropDownList<T> {
    pub fn new<S: Into<String>>(label: Option<S>, list: Vec<T>) -> Self {
        Self {
            label: label.map(|x| x.into()),
            list,
            selected: None,
        }
    }

    pub fn update(&mut self, message: DropDownListMessage<T>) {
        let DropDownListMessage::Selected(item) = message;
        self.selected = Some(item);
    }
}

impl<'a, T> From<&'a DropDownList<T>> for Element<'a, DropDownListMessage<T>>
where
    T: PartialEq + Clone + Display,
{
    fn from(value: &'a DropDownList<T>) -> Self {
        let row = match &value.label {
            Some(label) => row!(text(label.as_str()).width(LockAndTextInput::CHECKBOX_WIDTH)),
            None => row!(),
        };
        let row = row.push(pick_list(
            value.list.as_slice(),
            value.selected.as_ref(),
            DropDownListMessage::Selected,
        ));
        row.spacing(App::SPACING).into()
    }
}
