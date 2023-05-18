use iced::{
    theme,
    widget::{button, column, container, radio, row, text, text_input, Space},
    Element, Length, Renderer, Theme,
};

use crate::{
    game::{Game, PlayerType, Difficulty},
    Message,
};

#[derive(Clone)]
pub struct GameSettings {
    width: isize,
    height: isize,
    goal: isize,
    players: Vec<PlayerType>,
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    SetWidth(isize),
    ParseWidth(String),
    SetHeight(isize),
    ParseHeight(String),
    SetGoal(isize),
    ParseGoal(String),
    RemovePlayer(usize),
    AddPlayer,
    SetPlayerType(usize, PlayerType),
}

impl From<SettingsMessage> for Message {
    fn from(value: SettingsMessage) -> Self {
        Message::GameSettingsMessage(value)
    }
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            width: 6,
            height: 6,
            goal: 4,
            players: vec![
                PlayerType::User,
                PlayerType::Computer(crate::game::Difficulty::Normal),
            ],
        }
    }
}

impl GameSettings {
    pub fn to_game(&self) -> Game {
        Game::new(self.width, self.height, self.goal, self.players.clone()).unwrap()
    }

    pub fn view(&self) -> iced::Element<'_, Message, iced::Renderer<Theme>> {
        let title = text("Game Settings")
            .size(50)
            .horizontal_alignment(iced::alignment::Horizontal::Center)
            .into();

        let numerical_input_values = row![
            column(vec![
                text("Width")
                    .horizontal_alignment(iced::alignment::Horizontal::Center)
                    .into(),
                numerical_input(
                    text_input("", self.width.to_string().as_str())
                        .on_input(|value| SettingsMessage::ParseWidth(value).into()),
                    if self.width > self.goal {
                        Some(SettingsMessage::SetWidth(self.width - 1).into())
                    } else {
                        None
                    },
                    Some(SettingsMessage::SetWidth(self.width + 1).into())
                )
                .into()
            ])
            .align_items(iced::Alignment::Center)
            .width(Length::Fixed(100.0)),
            column(vec![
                text("Height")
                    .horizontal_alignment(iced::alignment::Horizontal::Center)
                    .into(),
                numerical_input(
                    text_input("", self.height.to_string().as_str())
                        .on_input(|value| SettingsMessage::ParseHeight(value).into()),
                    if self.height > self.goal {
                        Some(SettingsMessage::SetHeight(self.height - 1).into())
                    } else {
                        None
                    },
                    Some(SettingsMessage::SetHeight(self.height + 1).into())
                )
                .into()
            ])
            .align_items(iced::Alignment::Center)
            .width(Length::Fixed(100.0)),
            column(vec![
                text("Goal")
                    .horizontal_alignment(iced::alignment::Horizontal::Center)
                    .into(),
                numerical_input(
                    text_input("", self.goal.to_string().as_str())
                        .on_input(|value| SettingsMessage::ParseGoal(value).into()),
                    Some(SettingsMessage::SetGoal(self.goal - 1).into()),
                    if self.goal < isize::min(self.height, self.width) {
                        Some(SettingsMessage::SetGoal(self.goal + 1).into())
                    } else {
                        None
                    }
                )
                .into()
            ])
            .align_items(iced::Alignment::Center)
            .width(Length::Fixed(100.0))
        ]
        .spacing(50)
        .into();

        let player_title = text("Players").size(30).into();

        let players = column(
            self.players
                .iter()
                .enumerate()
                .map(|(i, player)| {
                    let set_player_type = |value| SettingsMessage::SetPlayerType(i, value).into();
                    row(vec![
                        radio("User", PlayerType::User, Some(*player), set_player_type).into(),
                        row(
                            Difficulty::all().into_iter().map(|difficulty| {
                                radio(
                                    difficulty.to_string(),
                                    PlayerType::Computer(difficulty),
                                    Some(*player),
                                    set_player_type,
                                )
                                .into()
                            }).collect()
                        ).into(),
                        Space::new(10, 0).into(),
                        button("Delete")
                            .on_press(SettingsMessage::RemovePlayer(i).into())
                            .style(theme::Button::Text)
                            .into(),
                    ])
                    .spacing(20)
                    .into()
                })
                .collect(),
        )
        .into();

        let add_player = {
            let button = button(text("Add Player").size(25)).style(theme::Button::Text);
            if self.players.len() < 10 {
                button.on_press(SettingsMessage::AddPlayer.into())
            } else {
                button
            }
        }
        .into();

        let bottom_space = Space::new(0, Length::Fill).into();

        let play_game = button(text("Start Game").size(30))
            .on_press(Message::StartGame)
            .into();

        let content = column(vec![
            title,
            numerical_input_values,
            player_title,
            players,
            add_player,
            bottom_space,
            play_game,
        ])
        .align_items(iced::Alignment::Center)
        .width(Length::Fill)
        .padding(20)
        .spacing(10);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    pub fn handle_message(&mut self, message: Message) {
        if let Message::GameSettingsMessage(message) = message {
            match message {
                SettingsMessage::SetWidth(width) => {
                    self.width = width;
                }
                SettingsMessage::SetHeight(height) => {
                    self.height = height;
                }
                SettingsMessage::SetGoal(goal) => {
                    self.goal = goal;
                }
                SettingsMessage::RemovePlayer(index) => {
                    self.players.remove(index);
                }
                SettingsMessage::AddPlayer => self.players.push(PlayerType::User),
                SettingsMessage::SetPlayerType(index, player_type) => {
                    self.players[index] = player_type;
                }
                SettingsMessage::ParseWidth(value) => {
                    if let Ok(width) = value.parse() {
                        self.width = width;
                    }
                }
                SettingsMessage::ParseHeight(value) => {
                    if let Ok(height) = value.parse() {
                        self.height = height;
                    }
                }
                SettingsMessage::ParseGoal(goal) => {
                    if let Ok(goal) = goal.parse() {
                        self.goal = goal;
                    }
                }
            }
        }
    }
}

fn numerical_input<'a, Message: Clone + 'a, T: Into<Element<'a, Message, Renderer>>>(
    value: T,
    on_decrement: Option<Message>,
    on_increment: Option<Message>,
) -> Element<'a, Message, Renderer> {
    row![
        {
            let button = button("-").style(theme::Button::Text);
            match on_decrement {
                Some(message) => button.on_press(message),
                None => button,
            }
        },
        value.into(),
        {
            let button = button("+").style(theme::Button::Text);
            match on_increment {
                Some(message) => button.on_press(message),
                None => button,
            }
        }
    ]
    .align_items(iced::Alignment::Center)
    .width(Length::Shrink)
    .height(Length::Shrink)
    .spacing(5)
    .into()
}
