mod game;
mod computer;

use std::ops::{Add, Mul};

pub use game::*;
pub use computer::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl Add<Point> for Point {
    fn add(self, rhs: Point) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }

    type Output =  Point;
}

impl Mul<isize> for Point {
    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }

    type Output = Point;
}

impl From<(isize, isize)> for Point {
    fn from((x, y): (isize, isize)) -> Self {
        Self { x, y }
    }
}

pub type Player = usize;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum GameState {
    PlayerMove(Player),
    PlayerWon(Player),
    Draw,
}

pub trait GameTrait {
    fn clear(&mut self);
    fn get_tile(&self, point: &Point) -> Result<Option<Player>, InvalidPointError>;
    fn get_board(&self) -> Vec<Vec<Option<Player>>>;
    fn play_move(&mut self, point: Point) -> Result<(), PlayMoveError>;
    fn get_gamestate(&self) -> GameState;
    fn get_current_player(&self) -> Player;
    fn get_width(&self) -> isize;
    fn get_height(&self) -> isize;
    fn get_goal(&self) -> isize;
    fn get_player_count(&self) -> usize;
}

#[derive(Debug)]
pub enum PlayMoveError {
    InvalidPoint(InvalidPointError),
    PointIsPopulated(Player),
    InvalidGameState(GameState),
}

#[derive(Debug)]
pub enum InvalidPointError {
    XTooSmall,
    XTooLarge,
    YTooSmall,
    YTooLarge,
}

impl From<InvalidPointError> for PlayMoveError {
    fn from(value: InvalidPointError) -> Self {
        PlayMoveError::InvalidPoint(value)
    }
}
