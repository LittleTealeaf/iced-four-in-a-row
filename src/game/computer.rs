use std::slice::EscapeAscii;

use rand::seq::SliceRandom;

use crate::game::GameTrait;

use super::{Game, Player, Point};

const DIRECTIONS: [Point; 4] = [
    Point { x: 1, y: 0 },
    Point { x: -1, y: 1 },
    Point { x: 0, y: 1 },
    Point { x: 1, y: 1 },
];

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Bot(pub Difficulty, pub Strategy);

impl Bot {
    pub fn get_difficulty(&self) -> Difficulty {
        self.0
    }

    pub fn get_strategy(&self) -> Strategy {
        self.1
    }
}

impl Default for Bot {
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
    Insane,
}

impl Difficulty {
    pub const ALL: [Difficulty; 4] = [Self::Easy, Self::Normal, Self::Hard, Self::Insane];
}

impl ToString for Difficulty {
    fn to_string(&self) -> String {
        String::from(match self {
            Self::Easy => "Easy",
            Self::Normal => "Normal",
            Self::Hard => "Hard",
            Self::Insane => "Insane",
        })
    }
}

impl Default for Difficulty {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Strategy {
    Neutral,
    Offensive,
    Defensive,
}

impl Strategy {
    pub const ALL: [Strategy; 3] = [Self::Neutral, Self::Offensive, Self::Defensive];
}

impl ToString for Strategy {
    fn to_string(&self) -> String {
        String::from(match self {
            Self::Neutral => "Neutral",
            Self::Offensive => "Offensive",
            Self::Defensive => "Defensive",
        })
    }
}

impl Default for Strategy {
    fn default() -> Self {
        Self::Neutral
    }
}

struct ComputerWeights {
    computer: u64,
    opponent: u64,
    empty: u64,
    populated: u64,
    streak_computer: u64,
    streak_opponent: u64,
}

impl From<Bot> for ComputerWeights {
    fn from(value: Bot) -> Self {
        match value {
            Bot(Difficulty::Easy, Strategy::Neutral) => Self {
                computer: 1,
                opponent: 1,
                empty: 0,
                populated: 0,
                streak_computer: 1,
                streak_opponent: 1,
            },
            Bot(Difficulty::Easy, Strategy::Defensive) => Self {
                computer: 1,
                opponent: 1,
                empty: 0,
                populated: 1,
                streak_computer: 1,
                streak_opponent: 1,
            },
            Bot(Difficulty::Easy, Strategy::Offensive) => Self {
                computer: 1,
                opponent: 1,
                empty: 1,
                populated: 0,
                streak_computer: 1,
                streak_opponent: 1,
            },
            Bot(Difficulty::Normal, Strategy::Neutral) => Self {
                computer: 1,
                opponent: 1,
                empty: 1,
                populated: 1,
                streak_computer: 2,
                streak_opponent: 2,
            },
            Bot(Difficulty::Normal, Strategy::Defensive) => Self {
                computer: 4,
                opponent: 5,
                empty: 1,
                populated: 1,
                streak_computer: 2,
                streak_opponent: 1,
            },
            Bot(Difficulty::Normal, Strategy::Offensive) => Self {
                computer: 5,
                opponent: 4,
                empty: 1,
                populated: 1,
                streak_computer: 1,
                streak_opponent: 2,
            },
            Bot(Difficulty::Hard, Strategy::Neutral) => Self {
                computer: 4,
                opponent: 5,
                empty: 1,
                populated: 2,
                streak_computer: 2,
                streak_opponent: 2,
            },
            Bot(Difficulty::Hard, Strategy::Defensive) => Self {
                computer: 2,
                opponent: 3,
                empty: 1,
                populated: 2,
                streak_computer: 2,
                streak_opponent: 2,
            },
            Bot(Difficulty::Hard, Strategy::Offensive) => Self {
                computer: 3,
                opponent: 2,
                empty: 1,
                populated: 2,
                streak_computer: 2,
                streak_opponent: 2,
            },
            Bot(Difficulty::Insane, Strategy::Neutral) => Self {
                computer: 1,
                opponent: 2,
                empty: 1,
                populated: 3,
                streak_computer: 5,
                streak_opponent: 6,
            },
            Bot(Difficulty::Insane, Strategy::Defensive) => Self {
                computer: 1,
                opponent: 4,
                empty: 1,
                populated: 3,
                streak_computer: 2,
                streak_opponent: 4,
            },
            Bot(Difficulty::Insane, Strategy::Offensive) => Self {
                computer: 3,
                opponent: 2,
                empty: 1,
                populated: 3,
                streak_computer: 4,
                streak_opponent: 3,
            },
        }
    }
}

impl Game {
    pub fn get_computer_move(&self, bot: Bot) -> Option<Point> {
        let weights = bot.into();
        let computer = self.get_current_player();
        let evals: Vec<_> = (0..self.get_height())
            .map(|y| (0..self.get_width()).map(move |x| Point::new(x, y)))
            .flatten()
            .filter(|point| self.get_tile(point).unwrap().is_none())
            .map(|point| (point, self.evaluate_location(point, computer, &weights)))
            .collect();

        let max_evaluation = *evals.iter().map(|(_, i)| i).max().unwrap_or(&0);

        let max_moves: Vec<_> = evals
            .into_iter()
            .filter_map(|(point, eval)| {
                if eval == max_evaluation {
                    Some(point)
                } else {
                    None
                }
            })
            .collect();

        Some(*max_moves.choose(&mut rand::thread_rng())?)
    }

    fn evaluate_location(&self, point: Point, computer: usize, weights: &ComputerWeights) -> u64 {
        let mut eval = 0;
        let goal = self.get_goal();
        let player_count = self.get_player_count();

        for direction in DIRECTIONS {
            let mut initial_empty = 0;
            let mut empty = vec![0; player_count];
            let mut count = vec![0; player_count];

            for d in [-1, 1] {
                let mut step = ScannerStep::InitialEmpty;
                for i in 1..(self.get_goal()) {
                    match self.get_tile(&(point + (direction * d * i))) {
                        Ok(tile) => match step {
                            ScannerStep::InitialEmpty => {
                                if let Some(tile_player) = tile {
                                    step = ScannerStep::Player(tile_player);
                                    count[tile_player] += 1;
                                } else {
                                    initial_empty += 1;
                                }
                            }
                            ScannerStep::Player(player) => {
                                if let Some(tile_player) = tile {
                                    if tile_player == player {
                                        count[player] += 1;
                                    } else {
                                        break;
                                    }
                                } else {
                                    empty[player] += 1;
                                }
                            }
                        },
                        Err(_) => {
                            break;
                        }
                    }
                }
            }

            for player in 0..player_count {
                let empty = initial_empty + empty[player];
                let count = count[player];

                if empty as isize + count as isize >= goal - 1 {
                    let overall_weight = if player == computer {
                        weights.computer
                    } else {
                        weights.opponent
                    };
                    let streak_weight = if player == computer {
                        weights.streak_computer
                    } else {
                        weights.streak_opponent
                    };

                    eval += overall_weight
                        * (weights.empty * empty
                            + (count * weights.populated * streak_weight.pow(count as u32)));
                }
            }
        }

        eval
    }
}

enum ScannerStep {
    InitialEmpty,
    Player(Player),
}
