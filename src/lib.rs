/// Represents the final status of a game
pub enum Status {
    Win,
    Draw,
    Loss,
}

/// Represents the noughts and crosses board
pub struct Board {
    player: u16,
    opponent: u16,
    combined: u16,
}

impl Board {
    /// The binary representation of a full board
    const FULL: u16 = 0b111111111;

    /// The binary representation of the lines of three on the board
    const LINES: [u16; 8] = [
        0b000000111,
        0b000111000,
        0b111000000,
        0b001001001,
        0b010010010,
        0b100100100,
        0b100010001,
        0b001010100,
    ];

    /// Constructs a new instances representing an empty board
    pub fn new() -> Self {
        Self {
            player: 0,
            opponent: 0,
            combined: 0,
        }
    }

    /// Returns whether the current player has lost
    fn has_lost(&self) -> bool {
        for line in Self::LINES {
            if self.opponent & line == line {
                return true;
            }
        }

        false
    }

    /// Returns whether there are any moves available
    fn has_moves(&self) -> bool {
        self.combined != Self::FULL
    }

    /// Returns an instance for the opponent after the specified move has been
    /// made, or an error if the move is invalid
    fn with_move(&self, position: u16) -> Result<Self, &'static str> {
        let position = 1 << position;

        if self.combined & position != 0 {
            return Err("Invalid move");
        }

        Ok(Self {
            player: self.opponent,
            opponent: self.player | position,
            combined: self.player | self.opponent | position,
        })
    }
}

/// Returns whether the game is a win, draw, or loss for the current player
/// starting from the specified board position
pub fn solve(board: Board, games: &mut u32) -> Status {
    if !board.has_moves() {
        *games += 1;
        return Status::Draw;
    }

    // start by assuming the player will lose
    let mut result = Status::Loss;

    for position in 0..=8 {
        if let Ok(opponent_board) = board.with_move(position) {
            // the player has won, so return early
            if opponent_board.has_lost() {
                *games += 1;
                return Status::Win;
            }

            match solve(opponent_board, games) {
                // if the opponent can win, continue looking for a better result
                Status::Win => (),

                // the player can draw, but continue looking for a better result
                Status::Draw => result = Status::Draw,

                // the player can win, so we can return early
                Status::Loss => return Status::Win,
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_move() {
        assert!(Board::new().with_move(0).unwrap().with_move(0).is_err())
    }

    #[test]
    fn has_lost() {
        let mut board = Board::new();

        for position in 0..=6 {
            assert!(!board.has_lost());
            board = board.with_move(position).unwrap();
        }

        // At this point the board is:
        //
        // xox
        // oxo
        // x..
        assert!(board.has_lost());
    }

    #[test]
    fn has_moves() {
        let mut board = Board::new();

        for position in 0..=8 {
            assert!(board.has_moves());
            board = board.with_move(position).unwrap();
        }

        assert!(!board.has_moves());
    }
}
