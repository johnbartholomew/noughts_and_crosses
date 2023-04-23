/// Represents the final status of a game
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Status {
    Loss,
    Draw,
    Win,
}

// Internally we use a signed int so that we can negate it to get the 'other' player's win state.
const LOSS: i32 = -1;
const WIN: i32 = 1;
const DRAW: i32 = 0;

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

fn solve_inner(board: Board) -> (i32, usize) {
    debug_assert!(
        !board.has_won(),
        "We already won so we should not be trying more moves."
    );
    if board.has_lost() {
        return (LOSS, 1);
    }

    let mut bits = board.remaining_bits();
    if bits == 0 {
        return (DRAW, 1);
    }
    let mut best_result = -1;
    let mut games = 0;
    while bits != 0 {
        let low_bit = bits & !(bits - 1);
        bits &= !low_bit;
        let opponent_board = board.with_move_bit(low_bit);
        let (result, n) = solve_inner(opponent_board);
        games += n;
        // Negate the opponent's result to get our result.
        best_result = best_result.max(-result);
        if best_result == WIN {
            break;
        }
    }
    (best_result, games)
}

/// Returns whether the game is a win, draw, or loss for the current player
/// starting from the specified board position
pub fn solve(board: Board) -> (Status, usize) {
    let (result, n) = solve_inner(board);
    let result = match result {
        LOSS => Status::Loss,
        DRAW => Status::Draw,
        WIN => Status::Win,
        _ => unreachable!(),
    };
    (result, n)
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
