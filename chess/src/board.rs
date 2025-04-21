/*

8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖
7 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
6 . . . . . . . .
5 . . . . . . . .
4 . . . . . . . .
3 . . . . . . . .
2 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟
1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
  a b c d e f g h

*/

use crate::pieces::{Pieces, Piece};
use crate::coordinate::Coordinate;
use crate::rules::Rules;

macro_rules! debug_log {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            println!($($arg)*);
        }
    };
}

#[derive(PartialEq)]
enum Turn {
    White,
    Black
}

pub struct Board {
    pieces: [Piece; 64],
    ignore_turn: bool,
    turn: Turn
}

impl Board {
    pub fn new () -> Self {
        Self {
            pieces: [
                Piece::new(Pieces::WhiteRook, 0, 0),
                Piece::new(Pieces::WhiteKnight, 1, 0),
                Piece::new(Pieces::WhiteBishop, 2, 0),
                Piece::new(Pieces::WhiteQueen, 3, 0),
                Piece::new(Pieces::WhiteKing, 4, 0),
                Piece::new(Pieces::WhiteBishop, 5, 0),
                Piece::new(Pieces::WhiteKnight, 6, 0),
                Piece::new(Pieces::WhiteRook, 7, 0),
                Piece::new(Pieces::WhitePawn, 0, 1),
                Piece::new(Pieces::WhitePawn, 1, 1),
                Piece::new(Pieces::WhitePawn, 2, 1),
                Piece::new(Pieces::WhitePawn, 3, 1),
                Piece::new(Pieces::WhitePawn, 4, 1),
                Piece::new(Pieces::WhitePawn, 5, 1),
                Piece::new(Pieces::WhitePawn, 6, 1),
                Piece::new(Pieces::WhitePawn, 7, 1),
                Piece::new(Pieces::Empty, 0, 2),
                Piece::new(Pieces::Empty, 1, 2),
                Piece::new(Pieces::Empty, 2, 2),
                Piece::new(Pieces::Empty, 3, 2),
                Piece::new(Pieces::Empty, 4, 2),
                Piece::new(Pieces::Empty, 5, 2),
                Piece::new(Pieces::Empty, 6, 2),
                Piece::new(Pieces::Empty, 7, 2),
                Piece::new(Pieces::Empty, 0, 3),
                Piece::new(Pieces::Empty, 1, 3),
                Piece::new(Pieces::Empty, 2, 3),
                Piece::new(Pieces::Empty, 3, 3),
                Piece::new(Pieces::Empty, 4, 3),
                Piece::new(Pieces::Empty, 5, 3),
                Piece::new(Pieces::Empty, 6, 3),
                Piece::new(Pieces::Empty, 7, 3),
                Piece::new(Pieces::Empty, 0, 4),
                Piece::new(Pieces::Empty, 1, 4),
                Piece::new(Pieces::Empty, 2, 4),
                Piece::new(Pieces::Empty, 3, 4),
                Piece::new(Pieces::Empty, 4, 4),
                Piece::new(Pieces::Empty, 5, 4),
                Piece::new(Pieces::Empty, 6, 4),
                Piece::new(Pieces::Empty, 7, 4),
                Piece::new(Pieces::Empty, 0, 5),
                Piece::new(Pieces::Empty, 1, 5),
                Piece::new(Pieces::Empty, 2, 5),
                Piece::new(Pieces::Empty, 3, 5),
                Piece::new(Pieces::Empty, 4, 5),
                Piece::new(Pieces::Empty, 5, 5),
                Piece::new(Pieces::Empty, 6, 5),
                Piece::new(Pieces::Empty, 7, 5),
                Piece::new(Pieces::BlackPawn, 0, 6),
                Piece::new(Pieces::BlackPawn, 1, 6),
                Piece::new(Pieces::BlackPawn, 2, 6),
                Piece::new(Pieces::BlackPawn, 3, 6),
                Piece::new(Pieces::BlackPawn, 4, 6),
                Piece::new(Pieces::BlackPawn, 5, 6),
                Piece::new(Pieces::BlackPawn, 6, 6),
                Piece::new(Pieces::BlackPawn, 7, 6),
                Piece::new(Pieces::BlackRook, 0, 7),
                Piece::new(Pieces::BlackKnight, 1, 7),
                Piece::new(Pieces::BlackBishop, 2, 7),
                Piece::new(Pieces::BlackQueen, 3, 7),
                Piece::new(Pieces::BlackKing, 4, 7),
                Piece::new(Pieces::BlackBishop, 5, 7),
                Piece::new(Pieces::BlackKnight, 6, 7),
                Piece::new(Pieces::BlackRook, 7, 7)
            ],
            ignore_turn: false,
            turn: Turn::White
        }
    }

    pub fn get_current_setup (&self) -> &[Piece; 64] {
        &self.pieces
    }

    pub fn draw (&self) {
        for i in 0..8 {
            print!("{} ", 8 - i);
            for e in 0..8 {
                if ((i % 2 == 0) && (e % 2 == 1)) || ((i % 2 == 1) && (e % 2 == 0)) {
                    print!("\x1b[47;30m{} \x1b[0m",  self.pieces[i * 8 + e].symbol());
                }
                else {
                    print!("\x1b[100;30m{} \x1b[0m",  self.pieces[i * 8 + e].symbol());
                }
            }
            print!("\n");
        }
        print!("  a b c d e f g h\n\n");
    }

    pub fn teleport (&mut self, from_col: char, from_row: u8, to_col: char, to_row: u8) {
        let (_, _, from_index) = Coordinate::convert_coordinates(from_col, from_row);
        let (_, _, to_index) = Coordinate::convert_coordinates(to_col, to_row);
        if from_index == to_index { return; }
        self.pieces[to_index].piece_type = self.pieces[from_index].piece_type;
        self.pieces[to_index].moved = true;
        self.pieces[from_index].piece_type = Pieces::Empty;
        self.pieces[from_index].moved = false;
        // do not flip turn
    }

    pub fn make_a_move (&mut self, from_col: char, from_row: u8, to_col: char, to_row: u8) -> bool {
        let from_coordinate = Coordinate::from_row_col(from_col, from_row);
        let to_coordinate = Coordinate::from_row_col(to_col, to_row);
        let from_index = from_coordinate.as_index();
        let to_index = to_coordinate.as_index();
        // moving a piece to the same place is invalid
        if from_index == to_index {
            debug_log!("cannot move to the same place");
            return false;
        }
        let from_piece = &self.pieces[from_index];
        let to_piece = &self.pieces[to_index];
        // an empty piece cannot be moved
        if from_piece.is_empty() { return false; }
        // consider the turn, e.g. white piece cannot be moved in a black turn
        if !self.ignore_turn {
            if (self.turn == Turn::White) && from_piece.is_black() {
                debug_log!("cannot move black piece in white turn");
                return false;
            }
            if (self.turn == Turn::Black) && from_piece.is_white() {
                debug_log!("cannot move white piece in black turn");
                return false;
            }
        }
        let (from_x, from_y) = from_coordinate.as_x_y();
        let (to_x, to_y) = from_coordinate.as_x_y();
        debug_log!("moving from [{};{}] to [{};{}]", from_x, from_y, to_x, to_y);
        // check if the basic movement is valid
        if !Rules::is_basic_movement_valid(self, &from_coordinate, &to_coordinate) { return false; }

        // everything checks out -> make the move
        self.pieces[to_index].piece_type = self.pieces[from_index].piece_type;
        self.pieces[to_index].moved = true;
        self.pieces[from_index].piece_type = Pieces::Empty;
        self.pieces[from_index].moved = false;
        // flip the turn -> assume correct usage (move was checked with 'can_move_to' beforehand
        self.flip_turn();
        return true;
    }

    fn flip_turn (&mut self) {
        if self.turn == Turn::White { self.turn = Turn::Black; }
        else { self.turn = Turn::White; }
    }

    pub fn set_ignore_turn (&mut self, ignore: bool) {
        self.ignore_turn = ignore;
    }

}