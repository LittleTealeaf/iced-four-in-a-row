use game::{ComputerGame, Game, GameState, GameTrait, Player};
use iced::{
    executor,
    theme::{self, Button},
    widget::{button, checkbox, column, container, row, text},
    Application, Color, Command, Length, Settings, Size, Theme,
};
use widgets::numerical_input;

mod game;
mod widgets;

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
    SetVersusComputer(bool),
    StartGame,
    PlayMove { x: isize, y: isize },
}

enum FourInARow {
    Settings(GameSettings),
    Playing(Game),
    PlayingComputer(ComputerGame),
}

struct GameSettings {
    width: isize,
    height: isize,
    win_length: isize,
    against_computer: bool,
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
                against_computer: true,
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
                Message::SetVersusComputer(val) => {
                    settings.against_computer = val;
                    Command::none()
                }
                Message::StartGame => {
                    let game =
                        Game::new(settings.width, settings.height, settings.win_length).unwrap();
                    *self = if settings.against_computer {
                        Self::PlayingComputer(ComputerGame::new(game, game::Difficulty::HARD))
                    } else {
                        Self::Playing(game)
                    };
                    Command::none()
                }

                _ => Command::none(),
            },
            Self::Playing(game) => match message {
                Message::PlayMove { x, y } => {
                    game.play_move(x as usize, y as usize);
                    Command::none()
                }
                _ => Command::none(),
            },
            Self::PlayingComputer(game) => match message {
                Message::PlayMove { x, y } => {
                    game.play_move(x as usize, y as usize);
                    Command::none()
                }
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
                    numerical_input(
                        settings.width,
                        Message::SetWidth(settings.width - 1),
                        Message::SetWidth(settings.width + 1)
                    )
                ]
                .align_items(iced::Alignment::Center)
                .spacing(5)
                .into();

                let height = row![
                    text("Height"),
                    numerical_input(
                        settings.height,
                        Message::SetHeight(settings.height - 1),
                        Message::SetHeight(settings.height + 1)
                    )
                ]
                .align_items(iced::Alignment::Center)
                .spacing(5)
                .into();

                let win_length = row![
                    text("Win Length"),
                    numerical_input(
                        settings.win_length,
                        Message::SetWinLength(settings.win_length - 1),
                        Message::SetWinLength(settings.win_length + 1)
                    )
                ]
                .align_items(iced::Alignment::Center)
                .spacing(5)
                .into();

                let versus_computer =
                    checkbox("Versus Computer", settings.against_computer, |value| {
                        Message::SetVersusComputer(value)
                    })
                    .into();

                let start_game = button("Start Game").on_press(Message::StartGame).into();

                column(vec![
                    title,
                    width,
                    height,
                    win_length,
                    versus_computer,
                    start_game,
                ])
                .align_items(iced::Alignment::Center)
                .spacing(5)
                .into()
            }
            Self::Playing(game) | Self::PlayingComputer(ComputerGame { game, .. }) => {
                let gamestate = game.get_gamestate();

                let grid = row(game
                    .get_board()
                    .into_iter()
                    .enumerate()
                    .map(|(y, row)| {
                        column::<Message, iced::Renderer>(
                            row.into_iter()
                                .enumerate()
                                .map(|(x, tile)| {
                                    let mut button = button("")
                                        .width(Length::Fill)
                                        .height(Length::Fill)
                                        .style(theme::Button::Custom(Box::new(match tile {
                                            Some(Player::A) => {
                                                ButtonColor(Color::from_rgb(0f32, 0f32, 1f32))
                                            }
                                            Some(Player::B) => {
                                                ButtonColor(Color::from_rgb(1f32, 0f32, 0f32))
                                            }
                                            None => {
                                                ButtonColor(Color::from_rgb(0.4f32, 0.4f32, 0.4f32))
                                            }
                                        })));

                                    if GameState::InProgress == gamestate {
                                        button = button.on_press(Message::PlayMove {
                                            x: x as isize,
                                            y: y as isize,
                                        });
                                    }
                                    button.into()
                                })
                                .collect(),
                        )
                        .spacing(10)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .into()
                    })
                    .collect())
                .spacing(10)
                .into();

                let content = column(vec![grid])
                    .align_items(iced::Alignment::Center)
                    .width(Length::Fill)
                    .spacing(20);

                container(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .into()
            }
        }
    }
}

struct ButtonColor(iced::Color);

impl button::StyleSheet for ButtonColor {
    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(self.0)),
            ..Default::default()
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(self.0)),
            ..Default::default()
        }
    }

    type Style = Theme;
}

fn test() {
    theme::Button::Custom(Box::new(ButtonColor(Color::from_rgb(1f32, 0f32, 0f32))));
}
