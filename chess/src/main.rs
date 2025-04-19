pub mod pieces;
pub mod board;
pub mod coordinate;
pub mod movement;
pub mod history;
pub mod rules;

use std::io::{self, Write};
use regex::Regex;

fn main() {
    let mut chessboard = board::Board::new();
    chessboard.draw();

    let re = Regex::new(r"^([a-h])\s?([1-8])\s+to\s+([a-h])\s?([1-8])$").unwrap();
    loop {
        print!("enter move (e.g. 'a 2 to b 3'): ");
        io::stdout().flush().unwrap(); // flush before input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if let Some(caps) = re.captures(input) {
            let from_col = caps[1].chars().next().unwrap();
            let from_row: u8 = caps[2].parse().unwrap();
            let to_col = caps[3].chars().next().unwrap();
            let to_row: u8 = caps[4].parse().unwrap();

            //println!("parsed move: from {}{} to {}{}", from_col, from_row, to_col, to_row);
            chessboard.select(from_col, from_row);
            if !chessboard.can_move_to(to_col, to_row) {
                println!("invalid move");
                continue;
            }
            chessboard.move_to(to_col, to_row);
            chessboard.draw();
        }
        else {
            println!("invalid input");
        }
    }
}
