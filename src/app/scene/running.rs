use crate::app::input::{
    drop_down_list::DropDownListMessage, lock_and_text_input::LockAndTextInput,
    lock_and_text_input::LockAndTextInputMessage,
};
use crate::app::message::{Message, MessageField};
use crate::app::state::demon_slot::DemonSlot;
use crate::app::state::State;
use crate::app::App;
use iced::widget::{checkbox, column, pick_list, row, space::Space, text};
use iced::Element;

impl<'a> From<&'a State> for Element<'a, Message> {
    fn from(value: &'a State) -> Self {
        const DEMON_SLOT_LIST: &[DemonSlot] = &[
            DemonSlot::DemiFiend,
            DemonSlot::Party(0),
            DemonSlot::Party(1),
            DemonSlot::Party(2),
            DemonSlot::Party(3),
            DemonSlot::Party(4),
            DemonSlot::Party(5),
            DemonSlot::Party(6),
            DemonSlot::Party(7),
            DemonSlot::Party(8),
            DemonSlot::Party(9),
            DemonSlot::Party(10),
            DemonSlot::Party(11),
        ];

        let demon = value.get_demon();

        use LockAndTextInputMessage::*;
        macro_rules! map_message {
            ($e: expr, $field: ident) => {
                Element::from(&$e.input).map(|message| match message {
                    Toggled(locked) => Message::RunningLock(MessageField::$field, locked),
                    Input(input) => Message::RunningInput(MessageField::$field, input),
                })
            };
            ($e: expr, $position: expr) => {
                Element::from(&$e.input).map(|message| {
                    let DropDownListMessage::Selected(selected) = message;
                    Message::RunningSkill($position, selected)
                })
            };
        }
        row!(
            column!(
                row!(
                    text("Target").width(LockAndTextInput::CHECKBOX_WIDTH),
                    pick_list(DEMON_SLOT_LIST, Some(value.demon_selected), |demon| {
                        Message::RunningDemon(demon)
                    }),
                )
                .spacing(App::SPACING),
                Space::new(0, App::SPACING * 2),
                Element::from(&demon.id.input).map(|message| {
                    let DropDownListMessage::Selected(selected) = message;
                    Message::RunningId(selected)
                }),
                Space::new(0, App::SPACING),
                checkbox("Randomize", demon.randomize).on_toggle(Message::RunningRandomize),
                row!(
                    map_message!(demon.skills[0], 0),
                    map_message!(demon.skills[4], 4),
                )
                .spacing(App::SPACING),
                row!(
                    map_message!(demon.skills[1], 1),
                    map_message!(demon.skills[5], 5),
                )
                .spacing(App::SPACING),
                row!(
                    map_message!(demon.skills[2], 2),
                    map_message!(demon.skills[6], 6),
                )
                .spacing(App::SPACING),
                row!(
                    map_message!(demon.skills[3], 3),
                    map_message!(demon.skills[7], 7),
                )
                .spacing(App::SPACING),
                Space::new(0, App::SPACING),
                row!(
                    map_message!(demon.skills[8], 8),
                    map_message!(demon.skills[12], 12),
                )
                .spacing(App::SPACING),
                row!(
                    map_message!(demon.skills[9], 9),
                    map_message!(demon.skills[13], 13),
                )
                .spacing(App::SPACING),
                row!(
                    map_message!(demon.skills[10], 10),
                    map_message!(demon.skills[14], 14),
                )
                .spacing(App::SPACING),
                row!(
                    map_message!(demon.skills[11], 11),
                    map_message!(demon.skills[15], 15),
                )
                .spacing(App::SPACING),
            )
            .spacing(App::SPACING),
            column!(
                map_message!(value.macca, Macca),
                Space::new(0, App::SPACING * 2),
                map_message!(demon.hp, Hp),
                map_message!(demon.max_hp, MaxHp),
                map_message!(demon.mp, Mp),
                map_message!(demon.max_mp, MaxMp),
                map_message!(demon.exp, Exp),
                map_message!(demon.level, Level),
                map_message!(demon.st, St),
                map_message!(demon.ma, Ma),
                map_message!(demon.vi, Vi),
                map_message!(demon.ag, Ag),
                map_message!(demon.lu, Lu),
            )
            .spacing(App::SPACING),
        )
        .spacing(App::SPACING)
        .padding([App::SPACING, App::SPACING])
        .into()
    }
}
