use std::collections::HashMap;

use rand::seq::SliceRandom; // 0.7.2

use super::{Board, Game, GameState, GameTrait, InvalidPosition, PlayMoveError, Player};

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (-1, 1), (0, 1), (1, 1)];

pub struct ComputerWeights {
    computer: isize,
    player: isize,
    empty: isize,
    populated: isize,
    streak_computer: isize,
    streak_player: isize,
}

pub enum Difficulty {
    EASY,
    NORMAL,
    HARD,
}

impl From<Difficulty> for ComputerWeights {
    fn from(difficulty: Difficulty) -> Self {
        match difficulty {
            Difficulty::EASY => ComputerWeights {
                computer: 1,
                player: 1,
                empty: 0,
                populated: 0,
                streak_computer: 1,
                streak_player: 1,
            },
            Difficulty::NORMAL => ComputerWeights {
                computer: 1,
                player: 2,
                empty: 1,
                populated: 2,
                streak_computer: 2,
                streak_player: 2,
            },
            Difficulty::HARD => ComputerWeights {
                computer: 3,
                player: 4,
                empty: 1,
                populated: 5,
                streak_computer: 8,
                streak_player: 10,
            },
        }
    }
}

pub struct ComputerGame {
    pub game: Game,
    weights: ComputerWeights,
    computer: Player,
    player: Player,
}

impl ComputerGame {
    pub fn new(game: Game, difficulty: Difficulty) -> Self {
        let player = game.get_current_player();
        let computer = player.next_turn();
        let weights = difficulty.into();

        Self {
            game,
            weights,
            player,
            computer,
        }
    }

    fn get_computer_move(&self) -> (isize, isize) {
        // self.game.get_board().into_iter().flat_map(|row| row.into_iter().enumerate().)

        let vals = self
            .game
            .get_board()
            .into_iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.into_iter()
                    .enumerate()
                    .filter_map(move |(x, tile)| match tile {
                        Some(_) => None,
                        None => Some((
                            x,
                            y,
                            self.evaluate_location(x as isize, y as isize).unwrap(),
                        )),
                    })
            })
            .collect::<Vec<_>>();
        let max_evaluation = *vals.iter().map(|(_, _, i)| i).max().unwrap_or(&0);
        println!("Evaluation: {}", max_evaluation);
        // let vals = vals.into_iter().filter(|(_,_,i)| i == max_evaluation).collect::<Vec<_>>();
        let moves = vals
            .into_iter()
            .filter_map(|(x, y, eval)| {
                if eval == max_evaluation {
                    Some((x, y))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let (x, y) = moves.choose(&mut rand::thread_rng()).unwrap();

        (*x as isize, *y as isize)
    }

    fn evaluate_location(&self, x: isize, y: isize) -> Result<isize, InvalidPosition> {
        let ComputerGame { game, weights, .. } = self;

        let mut eval = 0;

        for (dx, dy) in DIRECTIONS {
            let mut empty_player = 0;
            let mut empty_computer = 0;
            let mut count_player = 0;
            let mut count_computer = 0;

            for d in [-1, 1] {
                for i in 1..(game.get_win_length()) {
                    match game.get_tile(x + dx * i * d, y + dy * i * d) {
                        Ok(Some(player)) => {
                            if player.eq(&self.player) {
                                count_player += 1;
                            } else {
                                break;
                            }
                        }
                        Ok(None) => {
                            empty_player += 1;
                        }
                        Err(_) => {
                            break;
                        }
                    }
                }
                for i in 1..=(game.get_win_length()) {
                    match game.get_tile(x + dx * i * d, y + dy * i * d) {
                        Ok(Some(player)) => {
                            if player.eq(&self.computer) {
                                count_computer += 1;
                            } else {
                                break;
                            }
                        }
                        Ok(None) => {
                            empty_computer += 1;
                        }
                        Err(_) => {
                            break;
                        }
                    }
                }
            }


            if empty_player + count_player as isize >= game.get_win_length() - 1 {
                eval += weights.player
                    * (empty_player * weights.empty
                        + (count_player as isize) * weights.populated)
                    * weights.streak_player.pow(count_player);
            }

            if empty_computer + count_computer as isize >= game.get_win_length() - 1 {
                eval += weights.computer
                * (empty_computer * weights.empty
                    + (count_computer as isize) * weights.populated)
                * weights.streak_computer.pow(count_computer);
            }
        }

        Ok(eval)
    }
}

impl GameTrait for ComputerGame {
    fn get_current_player(&self) -> Player {
        self.game.get_current_player()
    }

    fn get_board(&self) -> Board {
        self.game.get_board()
    }

    fn play_move(&mut self, x: usize, y: usize) -> Result<(), PlayMoveError> {
        self.game.play_move(x, y)?;

        if let GameState::InProgress = self.game.get_gamestate() {
            let (x, y) = self.get_computer_move();
            self.game.play_move(x as usize, y as usize)?;
        }

        Ok(())
    }

    fn get_gamestate(&self) -> GameState {
        self.game.get_gamestate()
    }

    fn get_width(&self) -> isize {
        self.game.get_width()
    }

    fn get_height(&self) -> isize {
        self.game.get_height()
    }

    fn get_win_length(&self) -> isize {
        self.game.get_win_length()
    }

    fn get_tile(&self, x: isize, y: isize) -> Result<&Option<Player>, InvalidPosition> {
        self.game.get_tile(x, y)
    }
}
