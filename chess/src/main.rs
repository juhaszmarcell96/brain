pub mod pieces;
pub mod board;

use std::io::{self, Write};
use regex::Regex;

fn main() {
    let mut chessboard = board::Board::new();
    chessboard.draw();
    chessboard.select('b', 1);
    chessboard.to('c', 3);
    chessboard.draw();
    chessboard.select('d', 7);
    chessboard.to('d', 5);
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
            chessboard.to(to_col, to_row);
            chessboard.draw();
        }
        else {
            println!("invalid input");
        }
    }
}
