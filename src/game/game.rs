use std::{
    collections::HashMap,
    ops::{Add, Mul},
};

use super::{computer, Bot, Difficulty, GameState, GameTrait, Player, Point, Strategy};

const DIRECTIONS: [Point; 4] = [
    Point { x: 1, y: 0 },
    Point { x: -1, y: 1 },
    Point { x: 0, y: 1 },
    Point { x: 1, y: 1 },
];

pub struct Game {
    board: HashMap<Point, Player>,
    width: isize,
    height: isize,
    goal: isize,
    players: Vec<PlayerType>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum PlayerType {
    User,
    Computer(Bot),
}

impl PlayerType {
    pub fn get_bot(&self) -> Option<&Bot> {
        match self {
            Self::User => None,
            Self::Computer(bot) => Some(bot),
        }
    }

    pub fn set_difficulty(self, difficulty: Difficulty) -> Self {
        match self {
            Self::User => self,
            Self::Computer(Bot(_, strategy)) => Self::Computer(Bot(difficulty, strategy)),
        }
    }

    pub fn set_strategy(self, strategy: Strategy) -> Self {
        match self {
            Self::User => self,
            Self::Computer(Bot(difficulty, _)) => Self::Computer(Bot(difficulty, strategy)),
        }
    }
}

impl Game {
    pub fn new(
        width: isize,
        height: isize,
        goal: isize,
        players: Vec<PlayerType>,
    ) -> Result<Self, NewGameError> {
        if players.len() < 2 {
            Err(NewGameError::PlayersMustBeAtLeast2)
        } else if width < 2 {
            Err(NewGameError::WidthMustBeAtLeast2)
        } else if height < 2 {
            Err(NewGameError::HeightMustBeAtLeast2)
        } else if goal >= height {
            Err(NewGameError::GoalMustBeLessThanHeight)
        } else if goal >= width {
            Err(NewGameError::GoalMustBeLessThanWidth)
        } else {
            Ok(Self {
                board: HashMap::new(),
                width,
                height,
                goal,
                players,
            })
        }
    }
}

#[derive(Debug)]
pub enum NewGameError {
    WidthMustBeAtLeast2,
    HeightMustBeAtLeast2,
    GoalMustBeLessThanWidth,
    GoalMustBeLessThanHeight,
    PlayersMustBeAtLeast2,
}

impl GameTrait for Game {
    fn clear(&mut self) {
        self.board.clear();
    }

    fn get_tile(&self, point: &Point) -> Result<Option<Player>, super::InvalidPointError> {
        if point.x < 0 {
            Err(super::InvalidPointError::XTooSmall)
        } else if point.x >= self.width {
            Err(super::InvalidPointError::XTooLarge)
        } else if point.y < 0 {
            Err(super::InvalidPointError::YTooSmall)
        } else if point.y >= self.height {
            Err(super::InvalidPointError::YTooLarge)
        } else {
            Ok(self.board.get(point).and_then(|player| Some(*player)))
        }
    }

    fn get_board(&self) -> Vec<Vec<Option<Player>>> {
        (0..self.height)
            .map(|y| {
                (0..self.width)
                    .map(|x| self.get_tile(&Point::new(x, y)).unwrap())
                    .collect()
            })
            .collect()
    }

    fn play_move(&mut self, point: Point) -> Result<(), super::PlayMoveError> {
        match self.get_gamestate() {
            GameState::PlayerMove(player) => match self.get_tile(&point)? {
                Some(tile_player) => Err(super::PlayMoveError::PointIsPopulated(tile_player)),
                None => {
                    self.board.insert(point, player);
                    Ok(())
                }
            },
            state => Err(super::PlayMoveError::InvalidGameState(state)),
        }?;
        while let GameState::PlayerMove(player) = self.get_gamestate() {
            if let PlayerType::Computer(difficulty) = self.players[player] {
                let computer_move = self.get_computer_move(difficulty).unwrap();
                self.board.insert(computer_move, player);
            } else {
                break;
            }
        }
        Ok(())
    }

    fn get_gamestate(&self) -> GameState {
        for (point, player) in self.board.iter() {
            for dpoint in DIRECTIONS {
                for i in 1..self.goal {
                    match self.get_tile(&point.add(dpoint.mul(i))) {
                        Ok(Some(other_player)) => {
                            if !player.eq(&other_player) {
                                break;
                            }
                            if i == self.goal - 1 {
                                return GameState::PlayerWon(*player);
                            }
                        }
                        _ => {
                            break;
                        }
                    }
                }
            }
        }

        if self.board.len() < (self.width * self.height) as usize {
            GameState::PlayerMove(self.get_current_player())
        } else {
            GameState::Draw
        }
    }

    fn get_current_player(&self) -> Player {
        self.board.len() % self.players.len()
    }

    fn get_width(&self) -> isize {
        self.width
    }

    fn get_height(&self) -> isize {
        self.height
    }

    fn get_goal(&self) -> isize {
        self.goal
    }

    fn get_player_count(&self) -> usize {
        self.players.len()
    }
}
