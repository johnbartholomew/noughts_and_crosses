//! A solver for the game Noughts and Crosses, also known as Tic-Tac-Toe.

mod board {
    // STYLE NOTE:
    // A (sub-)module for 'board' means that visibility can be controlled,
    // so that a clean and safe interface can be exposed for 'solve' to use.

    /// Represents the noughts and crosses board.
    #[derive(Default, Clone, PartialEq, Eq)]
    pub struct Board {
        player: u16,
        opponent: u16,
    }

    impl std::fmt::Display for Board {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let play_count = (self.player | self.opponent).count_ones();
            let mut buf = *b"   \n   \n   ";
            let (ex, oh) = if play_count & 1 == 1 {
                (self.opponent, self.player)
            } else {
                (self.player, self.opponent)
            };
            for i in 0..9 {
                let row = i / 3;
                let row = &mut buf[(4 * row)..];
                let col = i % 3;
                let b = 1u16 << i;
                if ex & b == b {
                    row[col] = b'X';
                } else if oh & b == b {
                    row[col] = b'O';
                }
            }
            // SAFETY: The string is UTF-8 (actually ASCII) by construction since it only uses ASCII characters.
            f.write_str(std::str::from_utf8(&buf).unwrap())
        }
    }

    impl std::fmt::Debug for Board {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if f.alternate() {
                write!(
                    f,
                    "Board{{ player: {:#09b}, opponent: {:#09b} }}",
                    self.player, self.opponent
                )
            } else {
                write!(f, "Board({:#09b}, {:#09b})", self.player, self.opponent)
            }
        }
    }

    /// The binary representation of the lines of three on the board
    const LINE_MASKS: [u16; 8] = [
        0b000000111,
        0b000111000,
        0b111000000,
        0b001001001,
        0b010010010,
        0b100100100,
        0b100010001,
        0b001010100,
    ];

    // STYLE NOTE:
    // When I have an embedded block of constant/static data I like to validate
    // properties of it with one or more tests. This helps ensure I haven't made
    // a silly copy-paste mistake that might not be easy to spot in the data itself.
    #[test]
    fn test_line_masks() {
        // Some properties of the set of lines of 3.
        let lines_sorted = {
            let mut ls = LINE_MASKS.clone();
            ls.sort_unstable();
            ls
        };

        assert!(
            lines_sorted.windows(2).all(|w| w[0] < w[1]),
            "Valid lines should be distinct."
        );
        assert!(
            lines_sorted.into_iter().all(|x| x.count_ones() == 3),
            "Valid lines should each have exactly 3 bits set."
        );
    }

    // STYLE NOTE:
    // I extracted this out of 'board' because I wanted to be able to use it
    // easily on either 'player' or 'opponent'. It doesn't need 'both' halves
    // of the board and pulling it out of the Board implementation makes that
    // a little clearer. It can be tested separately as well.
    fn has_won(places: u16) -> bool {
        LINE_MASKS.iter().any(|&line| places & line == line)
    }

    #[test]
    fn test_has_won() {
        // Spot check has_won against some obvious cases.
        assert!(!has_won(0u16));
        assert!(has_won(0b111111111));
        assert!(has_won(0b111000000));
        assert!(has_won(0b000000111));
        assert!(!has_won(0b101000101));
        assert!(has_won(0b101010101));
    }

    // STYLE NOTE:
    // This is a pretty simple bit twiddling hack; putting it into a function
    // allows it to be tested easily (and since it's only u16 it can be tested
    // exhaustively very cheaply).
    const fn lowest_bit16(bits: u16) -> u16 {
        if bits != 0 {
            bits & !(bits - 1)
        } else {
            0
        }
    }

    // STYLE NOTE:
    // When the state space is small I like exhaustive tests, as long as it's
    // actually _testing_ something. In this case I have an "alternative" implementation
    // of lowest_bit16, using the trailing_zeros() method (standard library method on u16).
    #[test]
    fn test_lowest_bit16_exhaustive() {
        assert_eq!(lowest_bit16(0), 0);
        for n in 1..=u16::MAX {
            let lo = lowest_bit16(n);
            assert_eq!(lo, 1u16 << n.trailing_zeros());
        }
    }

    /// An Iterator of valid next moves constructed from a board.
    pub struct Moves {
        base: Board,
        remain: u16,
    }

    impl std::iter::Iterator for Moves {
        type Item = Board;
        fn next(&mut self) -> Option<Self::Item> {
            if self.remain != 0 {
                let b = lowest_bit16(self.remain);
                self.remain &= !b;
                Some(self.base.with_move_bits(b))
            } else {
                None
            }
        }
    }

    #[derive(Debug)]
    pub struct InvalidBoard(&'static str);

    impl std::fmt::Display for InvalidBoard {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "invalid board: {}", self.0)
        }
    }

    #[derive(Debug)]
    pub struct InvalidMove;

    impl Board {
        const FULL: u16 = 0b111111111;

        /// Constructs an empty board.
        pub fn new() -> Self {
            Self::default()
        }

        /// Constructs a new board with the specified play pattern.
        pub fn from_bits(player: u16, opponent: u16) -> Result<Self, InvalidBoard> {
            if player & Self::FULL != player {
                return Err(InvalidBoard("player has excess bits set"));
            }
            if opponent & Self::FULL != opponent {
                return Err(InvalidBoard("opponent has excess bits set"));
            }
            if player ^ opponent != player | opponent {
                return Err(InvalidBoard(
                    "player and opponent have both played the same cell",
                ));
            }
            let player_turns = player.count_ones();
            let opponent_turns = opponent.count_ones();
            if player_turns > opponent_turns {
                return Err(InvalidBoard("player has had too many turns"));
            }
            if opponent_turns > player_turns + 1 {
                return Err(InvalidBoard("opponent has had too many turns"));
            }
            // opponent is always placed the last piece (by construction, see with_move_bits),
            // therefore *only* opponent can ever win! Once the winning move is played,
            // then "opponent" played it. If an odd number of moves have been played
            // in total, then X won, if an even number of moves have been played in total,
            // then O won.
            if has_won(player) {
                return Err(InvalidBoard("opponent had a turn after player won"));
            }
            Ok(Board { player, opponent })
        }

        /// Returns true if the opponent won (we lost).
        pub fn has_lost(&self) -> bool {
            has_won(self.opponent)
        }

        /// Returns an iterator of valid next board positions.
        /// Note that the resulting boards will be "flipped": opponent becomes player, player becomes opponent.
        pub fn moves(&self) -> Moves {
            let remain = Self::FULL & !(self.player | self.opponent);
            Moves {
                base: self.clone(),
                remain,
            }
        }

        /// Attempts to play the move, specified by the index of the cell to play in.
        /// If the move is not valid for any reason, returns Err(InvalidMove), otherwise
        /// returns the new board.
        pub fn with_move(&self, cell: u16) -> Result<Self, InvalidMove> {
            if cell > 8 {
                return Err(InvalidMove);
            }
            let bits = 1u16 << cell;
            // Can only play on blank cells.
            if bits & (self.player | self.opponent) != 0 {
                return Err(InvalidMove);
            }
            // Can't play at all once someone has won.
            if has_won(self.opponent) {
                return Err(InvalidMove);
            }
            Ok(self.with_move_bits(bits))
        }

        // Private because it relies on only accepting 'valid' next moves.
        fn with_move_bits(&self, position: u16) -> Self {
            // with_move_bits is only called from the Moves iterator, and Moves is constructed
            // so that it only picks valid (open) cells to try, so position should always
            // be valid (as checked by this debug_assert) by construction.
            // If with_move_bits was public then the argument should be checked by assert!()
            // or should be handled as a clean error (ie, with a Result type).
            debug_assert!(
                position > 0 && position <= 0b100000000 && position.is_power_of_two(),
                "position is invalid"
            );
            debug_assert!(
                position & (self.player | self.opponent) == 0,
                "position is already taken"
            );

            let ret = Self {
                player: self.opponent,
                opponent: self.player | position,
            };

            // SELF-CHECK: player and opponent don't have any overlapping cells.
            debug_assert!(
                (self.player ^ self.opponent) == (self.player | self.opponent),
                "player and opponent have both played in the same cell"
            );

            // SELF-CHECK: from_bits() could construct the same board.
            debug_assert!(Board::from_bits(self.player, self.opponent).unwrap() == *self);

            ret
        }
    }

    #[test]
    fn test_moves_from_empty() {
        let mut boards: Vec<_> = Board::new().moves().map(|b| b.opponent).collect();
        boards.sort_unstable();
        let want: Vec<_> = (0..9).map(|x| 1u16 << x).collect();
        assert_eq!(want.len(), 9);
        assert_eq!(boards, want);
    }

    #[test]
    fn test_moves_from_full() {
        let full = Board {
            player: 0b101010101,
            opponent: 0b010101010,
        };
        assert!(full.moves().next().is_none());
    }

    #[test]
    fn test_moves() {
        let bb = |player, opponent| Board { player, opponent };
        assert_eq!(bb(0b110000000, 0b001001000).moves().count(), 5);

        // I would spot-check more, but I'm lazy.
    }

    #[test]
    fn test_find_any_win_sequence() {
        use rand::prelude::SliceRandom;

        let mut rng = rand::thread_rng();
        let mut stack = vec![Board::new()];
        let mut move_stack = {
            let mut moves: Vec<_> = stack.last().unwrap().moves().collect();
            moves.shuffle(&mut rng);
            vec![moves.into_iter()]
        };
        'outer: while let Some(moves) = move_stack.last_mut() {
            if let Some(bb) = moves.next() {
                let found = bb.has_lost();
                let mut moves: Vec<_> = bb.moves().collect();
                moves.shuffle(&mut rng);
                move_stack.push(moves.into_iter());
                stack.push(bb);
                if found {
                    break 'outer;
                }
            } else {
                // This one's empty; back off.
                move_stack.pop();
                stack.pop();
            }
        }
        for (i, bb) in stack.iter().enumerate() {
            println!("{}:\n{}\n", i, bb);
        }
    }

    #[test]
    fn test_from_bits() {
        // Things that are not ok:
        let not_ok = |p, o| assert!(Board::from_bits(p, o).is_err());
        not_ok(1, 1); // Both players play cell 0.
        not_ok(1u16 << 6, 1u16 << 6); // Both players play cell 6.
        not_ok(1u16 << 9, 0); // Player plays out-of-range.
        not_ok(0, 1u16 << 9); // Opponent plays out-of-range.
        not_ok(1u16 << 15, 0); // Player plays out-of-range.
        not_ok(0, 1u16 << 15); // Opponent plays out-of-range.
        not_ok(1, 0); // Player has had a turn but opponent has not.
        not_ok(0b001000011, 0b000011000); // Opponent has had 4 turns, player has had 5.

        // Things that are ok:
        let is_ok = |p, o| assert!(Board::from_bits(p, o).is_ok());
        is_ok(0, 1); // Opponent has had a turn but player has not.
        is_ok(0b000011000, 0b001000011); // Opponent has had 3 turns, player has had 2.
        is_ok(0b010011000, 0b001000011); // Opponent has had 3 turns, player has had 3.
        is_ok(0, 0); // Both players have had 0 turns.
        assert_eq!(Board::from_bits(0, 0).unwrap(), Board::new());
    }
}

pub use board::Board;

type StatusInt = i32;

// Internally we use a signed int so that we can negate it to get the 'other' player's win state.
const LOSS: StatusInt = -1;
const WIN: StatusInt = 1;
const DRAW: StatusInt = 0;

fn solve_inner(board: &Board) -> (StatusInt, usize) {
    if board.has_lost() {
        return (LOSS, 1);
    }

    let mut best_result = -1;
    let mut games = 0;
    for opponent_board in board.moves() {
        let (result, n) = solve_inner(&opponent_board);
        games += n;
        // Negate the opponent's result to get our result.
        best_result = best_result.max(-result);
        if best_result == WIN {
            break;
        }
    }
    if games == 0 {
        (DRAW, 1)
    } else {
        (best_result, games)
    }
}

/// Represents the final status of a game
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Status {
    Loss,
    Draw,
    Win,
}

/// Returns whether the game is a win, draw, or loss for the current player starting from the
/// specified board position, and the count of boards examined not including the input board.
pub fn solve(board: &Board) -> (Status, usize) {
    let (result, n) = solve_inner(board);
    let result = match result {
        LOSS => Status::Loss,
        DRAW => Status::Draw,
        WIN => Status::Win,
        _ => unreachable!(),
    };
    (result, n)
}

#[test]
fn test_solve_from_empty() {
    assert_eq!(solve(&Board::new()), (Status::Draw, 38856));
}
