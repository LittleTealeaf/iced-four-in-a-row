use game::Game;
use iced::{
    executor,
    widget::{button, column, row, text},
    Application, Color, Command, Length, Settings, Theme,
};

mod game;

fn main() -> iced::Result {
    FourInARow::run(Settings {
        ..Settings::default()
    })
}

#[derive(Debug, Clone)]
enum Message {
    SetWidth(isize),
    SetHeight(isize),
    SetWinLength(isize),
    StartGame,
}

enum FourInARow {
    Settings(GameSettings),
    Playing(Game),
}

struct GameSettings {
    width: isize,
    height: isize,
    win_length: isize,
}

impl Application for FourInARow {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self::Settings(GameSettings {
                width: 6,
                height: 6,
                win_length: 4,
            }),
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Four In A Row")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match self {
            Self::Settings(settings) => match message {
                Message::SetWidth(width) => {
                    settings.width = width;
                    Command::none()
                }
                Message::SetHeight(height) => {
                    settings.height = height;
                    Command::none()
                }
                Message::SetWinLength(win_length) => {
                    settings.win_length = win_length;
                    Command::none()
                }
                _ => Command::none(),
            },
            Self::Playing(game) => match message {
                _ => Command::none(),
            },
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        match self {
            Self::Settings(settings) => {
                let title = text("Four in a Row")
                    .width(Length::Fill)
                    .size(100)
                    .style(Color::from([0.5, 0.5, 0.5]))
                    .horizontal_alignment(iced::alignment::Horizontal::Center)
                    .into();

                let width = row![
                    text("Width"),
                    text(settings.width),
                    column(vec![
                        button("+")
                            .on_press(Message::SetWidth(settings.width + 1))
                            .into(),
                        button("-")
                            .on_press(Message::SetWidth(settings.width - 1))
                            .into()
                    ])
                ]
                .into();

                let height = row![
                    text("Height"),
                    text(settings.height),
                    column(vec![
                        button("+")
                            .on_press(Message::SetHeight(settings.height + 1))
                            .into(),
                        button("-")
                            .on_press(Message::SetHeight(settings.height - 1))
                            .into()
                    ])
                ]
                .into();

                let win_length = row![
                    text("Win Length"),
                    text(settings.win_length),
                    column(vec![
                        button("+")
                            .on_press(Message::SetWinLength(settings.win_length + 1))
                            .into(),
                        button("-")
                            .on_press(Message::SetWinLength(settings.win_length - 1))
                            .into()
                    ])
                ]
                .into();

                column(vec![title, width, height, win_length]).into()
            }
            Self::Playing(game) => {
                todo!()
            }
        }
    }
}
