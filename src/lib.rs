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

    fn combined(&self) -> u16 {
        self.player | self.opponent
    }

    fn remaining_bits(&self) -> u16 {
        Self::FULL & !self.combined()
    }

    fn with_move_bit(&self, position: u16) -> Self {
        debug_assert!(
            position & Self::FULL == position,
            "not a valid board bitmap"
        );
        debug_assert!(
            position.count_ones() == 1,
            "not a valid single-move board bitmap"
        );
        debug_assert!(self.combined() & position == 0, "position is already taken");

        Self {
            player: self.opponent,
            opponent: self.player | position,
        }
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

    let mut bits = board.remaining_bits();
    if bits == 0 {
        return (Status::Draw, 1);
    }
    let mut best_result = Status::Loss;
    let mut games = 0;
    while bits != 0 {
        let low_bit = bits & !(bits - 1);
        bits &= !low_bit;
        let opponent_board = board.with_move_bit(low_bit);
        let (result, n) = solve(opponent_board);
        games += n;
        best_result = best_result.max(result.complement());
        if best_result == Status::Win {
            break;
        }
    }
    (best_result, games)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_lost() {
        let mut board = Board::new();

        for position in 0..=6 {
            assert!(!board.has_lost());
            board = board.with_move_bit(1u16 << position);
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
            assert!(board.remaining_bits() != 0);
            board = board.with_move_bit(1u16 << position);
        }

        assert!(board.remaining_bits() == 0);
    }
}
