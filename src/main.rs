//! Solves noughts and crosses

// "noughts_and_crosses" is long, so give it a shorter name.
use noughts_and_crosses as xoxo;

use std::time::Instant;

const FINAL_RESULT: (xoxo::Status, usize) = xoxo::solve(xoxo::Board::new());

fn main() {
    let now = Instant::now();

    let (result, games) = FINAL_RESULT; //xoxo::solve(xoxo::Board::new());
    let result = match result {
        xoxo::Status::Win => "win for the first player",
        xoxo::Status::Draw => "draw",
        xoxo::Status::Loss => "win for the second player",
    };

    println!(
        "Analysed {games} games in {} microseconds",
        now.elapsed().as_micros()
    );

    println!("Noughts and crosses is a {result} with perfect play");
}
