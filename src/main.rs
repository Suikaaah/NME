mod app;

use app::App;
use iced::{window::Position, Application, Result, Settings};

pub fn main() -> Result {
    let window = iced::window::Settings {
        size: App::WINDOW_SIZE_SETUP,
        position: Position::Centered,
        ..Default::default()
    };

    App::run(Settings {
        window,
        ..Default::default()
    })
}
