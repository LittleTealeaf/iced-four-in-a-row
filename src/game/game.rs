pub type Board = Vec<Vec<Option<Player>>>;

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (-1, 1), (0, 1), (1, 1)];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Player {
    A,
    B,
}

impl ToString for Player {
    fn to_string(&self) -> String {
        match self {
            Self::A => "A".to_string(),
            Self::B => "B".to_string()
        }
    }
}

impl Player {
    pub fn next_turn(&self) -> Self {
        match self {
            Self::A => Self::B,
            Self::B => Self::A,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GameState {
    InProgress,
    PlayerWon(Player),
    Draw,
}

pub struct Game {
    board: Board,
    height: isize,
    width: isize,
    win_length: isize,
    current_turn: Player,
}

pub trait GameTrait {
    fn get_current_player(&self) -> Player;
    fn get_board(&self) -> Board;
    fn play_move(&mut self, x: usize, y: usize) -> Result<(), PlayMoveError>;
    fn get_tile(&self, x: isize, y: isize) -> Result<&Option<Player>, InvalidPosition>;
    fn get_gamestate(&self) -> GameState;
    fn get_width(&self) -> isize;
    fn get_height(&self) -> isize;
    fn get_win_length(&self) -> isize;
}

impl GameTrait for Game {
    fn get_current_player(&self) -> Player {
        self.current_turn
    }

    fn get_board(&self) -> Board {
        self.board.clone()
    }

    fn play_move(&mut self, x: usize, y: usize) -> Result<(), PlayMoveError> {
        let tile = self
            .board
            .get(y)
            .ok_or(PlayMoveError::InvalidPosition(InvalidPosition::InvalidRow))?
            .get(x)
            .ok_or(PlayMoveError::InvalidPosition(
                InvalidPosition::InvalidColumn,
            ))?;

        if let Some(player) = tile {
            return Err(PlayMoveError::TilePresent(*player));
        }

        self.board
            .get_mut(y)
            .ok_or(PlayMoveError::InvalidPosition(InvalidPosition::InvalidRow))?[x] =
            Some(self.current_turn);

        self.current_turn = self.current_turn.next_turn();

        Ok(())
    }

    fn get_tile(&self, x: isize, y: isize) -> Result<&Option<Player>, InvalidPosition> {
        if x < 0 {
            Err(InvalidPosition::InvalidColumn)
        } else if y < 0 {
            Err(InvalidPosition::InvalidRow)
        } else {
            Ok(self
                .board
                .get(x as usize)
                .ok_or(InvalidPosition::InvalidRow)?
                .get(y as usize)
                .ok_or(InvalidPosition::InvalidColumn)?)
        }
    }

    fn get_gamestate(&self) -> GameState {
        let mut is_playable = false;

        for row in 0..self.height {
            for col in 0..self.width {
                match self.get_tile(col, row).unwrap() {
                    Some(player) => {
                        for (dx, dy) in DIRECTIONS {
                            for i in 1..self.win_length {
                                match self.get_tile(col + dx * i, row + dy * i) {
                                    Ok(Some(plr)) => {
                                        if !plr.eq(player) {
                                            break;
                                        }
                                        if i == self.win_length - 1 {
                                            return GameState::PlayerWon(*player);
                                        }
                                    }
                                    Ok(None) | Err(_) => {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    None => {
                        is_playable = true;
                    }
                }
            }
        }

        if is_playable {
            GameState::InProgress
        } else {
            GameState::Draw
        }
    }

    fn get_width(&self) -> isize {
        self.width
    }

    fn get_height(&self) -> isize {
        self.height
    }

    fn get_win_length(&self) -> isize {
        self.win_length
    }
}

impl Game {
    pub fn new(width: isize, height: isize, win_length: isize) -> Result<Self, NewGameError> {
        if height < 0 {
            return Err(NewGameError::InvalidHeight);
        }
        if width < 0 {
            return Err(NewGameError::InvalidWidth);
        }
        if win_length > height || win_length > width {
            return Err(NewGameError::InvalidWinLength);
        }

        let mut board = Vec::with_capacity(height as usize);
        for _ in 0..height {
            board.push(vec![None; width as usize]);
        }

        Ok(Self {
            board,
            height,
            width,
            win_length,
            current_turn: Player::A,
        })
    }


}

impl Default for Game {
    fn default() -> Self {
        Self::new(6, 6, 4).unwrap()
    }
}

#[derive(Debug)]
pub enum NewGameError {
    InvalidWidth,
    InvalidHeight,
    InvalidWinLength,
}

#[derive(Debug)]
pub enum PlayMoveError {
    InvalidPosition(InvalidPosition),
    TilePresent(Player),
}

#[derive(Debug)]
pub enum InvalidPosition {
    InvalidRow,
    InvalidColumn,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_game_has_proper_dimensions() {
        let width = 10;
        let height = 20;

        let game = Game::new(width, height, 6).unwrap();
        assert_eq!(height as usize, game.board.len());
        for row in game.board {
            assert_eq!(width as usize, row.len());
        }
    }

    #[test]
    fn new_game_is_in_progress() {
        let game = Game::default();

        assert_eq!(GameState::InProgress, game.get_gamestate());
    }
}
