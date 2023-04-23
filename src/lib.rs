/// Represents the final status of a game
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Status {
    Loss,
    Draw,
    Win,
}

impl Status {
    fn complement(&self) -> Self {
        match self {
            Status::Loss => Status::Win,
            Status::Draw => Status::Draw,
            Status::Win => Status::Loss,
        }
    }
}

/// Represents the noughts and crosses board
pub struct Board {
    player: u16,
    opponent: u16,
}

#[derive(Debug)]
struct InvalidMove;

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
        }
    }

    /// Returns whether the current player has lost
    fn has_lost(&self) -> bool {
        Self::LINES.iter().any(|&line| self.opponent & line == line)
    }

    fn has_won(&self) -> bool {
        Self {
            player: self.opponent,
            opponent: self.player,
        }
        .has_lost()
    }

    /// Returns whether there are any moves available
    fn has_moves(&self) -> bool {
        self.combined() != Self::FULL
    }

    fn combined(&self) -> u16 {
        self.player | self.opponent
    }

    /// Returns an instance for the opponent after the specified move has been
    /// made, or an error if the move is invalid
    fn with_move(&self, position: u16) -> Result<Self, InvalidMove> {
        assert!(position <= 8, "invalid board cell index");
        let position = 1 << position;

        if self.combined() & position != 0 {
            return Err(InvalidMove);
        }

        Ok(Self {
            player: self.opponent,
            opponent: self.player | position,
        })
    }
}

/// Returns whether the game is a win, draw, or loss for the current player
/// starting from the specified board position
pub fn solve(board: Board) -> (Status, usize) {
    debug_assert!(
        !board.has_won(),
        "We already won so we should not be trying more moves."
    );
    if board.has_lost() {
        return (Status::Loss, 1);
    }
    if !board.has_moves() {
        return (Status::Draw, 1);
    }

    // start by assuming the player will lose
    let mut best_result = Status::Loss;
    let mut games = 0;
    for position in 0..=8 {
        if let Ok(opponent_board) = board.with_move(position) {
            let (result, n) = solve(opponent_board);
            games += n;
            best_result = best_result.max(result.complement());
            if best_result == Status::Win {
                break;
            }
        }
    }
    (best_result, games)
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
