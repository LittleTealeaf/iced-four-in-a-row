use iced::{
    theme,
    widget::{button, column, container, row},
    Color, Length, Theme,
};

use crate::{
    game::{Game, GameTrait, Point},
    Message,
};

#[derive(Debug, Clone)]
pub enum GameMessage {
    PlayMove(Point),
}

impl From<GameMessage> for Message {
    fn from(value: GameMessage) -> Self {
        Message::GameMessage(value)
    }
}

impl Game {
    pub fn view(&self) -> iced::Element<'_, Message, iced::Renderer<Theme>> {
        let grid = column(
            self.get_board()
                .into_iter()
                .enumerate()
                .map(|(y, row_values)| {
                    row(row_values
                        .into_iter()
                        .enumerate()
                        .map(|(x, tile)| {
                            let button = button("").width(Length::Fill);
                            match tile {
                                Some(player) => {
                                    button.style(theme::Button::Custom(Box::new(match player {
                                        0 => ButtonColor(Color::from_rgb(1f32, 0f32, 0f32)),
                                        1 => ButtonColor(Color::from_rgb(0f32, 1f32, 0f32)),
                                        2 => ButtonColor(Color::from_rgb(0f32, 0f32, 1f32)),
                                        3 => ButtonColor(Color::from_rgb(1f32, 1f32, 0f32)),
                                        4 => ButtonColor(Color::from_rgb(1f32, 0f32, 1f32)),
                                        5 => ButtonColor(Color::from_rgb(0f32, 1f32, 1f32)),
                                        6 => ButtonColor(Color::from_rgb(0.5f32, 0f32, 0f32)),
                                        7 => ButtonColor(Color::from_rgb(0f32, 0.5f32, 0f32)),
                                        8 => ButtonColor(Color::from_rgb(0f32, 0f32, 0.5f32)),
                                        9 => ButtonColor(Color::from_rgb(0.5f32, 0.5f32, 0f32)),
                                        _ => ButtonColor(Color::from_rgb(1f32, 1f32, 1f32)),
                                    })))
                                }
                                None => button
                                    .style(theme::Button::Custom(Box::new(ButtonColor(
                                        Color::from_rgb(0.5f32, 0.5f32, 0.5f32),
                                    ))))
                                    .on_press(
                                        GameMessage::PlayMove(Point::new(x as isize, y as isize))
                                            .into(),
                                    ),
                            }
                            .into()
                        })
                        .collect())
                    .height(Length::Fill)
                    .spacing(1)
                    .into()
                })
                .collect(),
        )
        .spacing(1)
        .into();

        let content = column(vec![grid]);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    pub fn handle_message(&mut self, message: Message) {
        if let Message::GameMessage(message) = message {
            match message {
                GameMessage::PlayMove(point) => {
                    self.play_move(point);
                }
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
