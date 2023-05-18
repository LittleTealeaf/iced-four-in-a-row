use app::{GameMessage, GameSettings, SettingsMessage};
use game::Game;
use iced::{executor, Application, Command, Settings, Theme};

mod app;
mod game;

fn main() -> iced::Result {
    GameApp::run(Settings::default())
}

pub enum GameApp {
    GameSettings(GameSettings),
    Playing(Game),
}

#[derive(Debug, Clone)]
pub enum Message {
    GameSettingsMessage(SettingsMessage),
    GameMessage(GameMessage),
    StartGame,
}

impl Application for GameApp {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self::GameSettings(GameSettings::default()), Command::none())
    }

    fn title(&self) -> String {
        String::from("Four in a Row")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::StartGame => {
                if let Self::GameSettings(settings) = self {
                    *self = Self::Playing(settings.to_game());
                }
                Command::none()
            }

            message => {
                match self {
                    Self::GameSettings(settings) => settings.handle_message(message),
                    Self::Playing(game) => game.handle_message(message),
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        match self {
            Self::GameSettings(settings) => settings.view(),
            Self::Playing(game) => game.view(),
        }
    }
}
