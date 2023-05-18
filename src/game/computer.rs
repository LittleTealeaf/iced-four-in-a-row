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
pub enum Difficulty {
    Random,
    Normal,
    NormalOffensive,
    NormalDefensive,
    Hard,
    HardOffensive,
    HardDefensive,
    Insane,
    InsaneOffensive,
    InsaneDefensive,
}

impl ToString for Difficulty {
    fn to_string(&self) -> String {
        String::from(match self {
            Difficulty::Random => "Random",
            Difficulty::Normal => "Norm",
            Difficulty::NormalOffensive => "Norm Off",
            Difficulty::NormalDefensive => "Norm Def",
            Difficulty::Hard => "Hard",
            Difficulty::HardOffensive => "Hard Off",
            Difficulty::HardDefensive => "Hard Def",
            Difficulty::Insane => "Insane",
            Difficulty::InsaneOffensive => "Insane Off",
            Difficulty::InsaneDefensive => "Insane Def",
        })
    }
}

impl Difficulty {
    pub fn all() -> [Difficulty; 10] {
        [
            Difficulty::Random,
            Difficulty::Normal,
            Difficulty::NormalOffensive,
            Difficulty::NormalDefensive,
            Difficulty::Hard,
            Difficulty::HardOffensive,
            Difficulty::HardDefensive,
            Difficulty::Insane,
            Difficulty::InsaneOffensive,
            Difficulty::InsaneDefensive,
        ]
    }
}

impl From<Difficulty> for ComputerWeights {
    fn from(difficulty: Difficulty) -> Self {
        match difficulty {
            Difficulty::Random => ComputerWeights { computer:  0, opponent: 0, empty: 0, populated: 0, streak_computer: 1, streak_opponent: 1 },
            Difficulty::Normal => ComputerWeights { computer: 1, opponent: 1, empty: 1, populated: 1, streak_computer: 1, streak_opponent: 1 },
            Difficulty::NormalOffensive => ComputerWeights { computer: 3, opponent: 2, empty: 1, populated: 1, streak_computer: 1, streak_opponent: 1 },
            Difficulty::NormalDefensive => ComputerWeights { computer: 2, opponent: 3, empty: 1, populated: 1, streak_computer: 1, streak_opponent: 1 },
            Difficulty::Hard => ComputerWeights { computer: 1, opponent: 1, empty: 1, populated: 2, streak_computer: 1, streak_opponent: 1 },
            Difficulty::HardOffensive => ComputerWeights { computer: 3, opponent: 2, empty: 1, populated: 2, streak_computer: 1, streak_opponent: 1 },
            Difficulty::HardDefensive => ComputerWeights { computer: 2, opponent: 3, empty: 1, populated: 2, streak_computer: 1, streak_opponent: 1 },
            Difficulty::Insane => ComputerWeights { computer: 1, opponent: 1, empty: 1, populated: 2, streak_computer: 2, streak_opponent: 2 },
            Difficulty::InsaneOffensive => ComputerWeights { computer: 3, opponent: 2, empty: 1, populated: 2, streak_computer: 4, streak_opponent: 3 },
            Difficulty::InsaneDefensive => ComputerWeights { computer: 2, opponent: 3, empty: 1, populated: 2, streak_computer: 3, streak_opponent: 4 },
        }
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

impl Game {
    pub fn get_computer_move(&self, difficulty: Difficulty) -> Option<Point> {
        let weights = difficulty.into();
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
