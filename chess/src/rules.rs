/*
           x
   +-------->
   |   (0)
   |  8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖
   |  7 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
   |  6 . . . . . . . .
   |  5 . . . . . . . .
 y V  4 . . . . . . . .
      3 . . . . . . . .
      2 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟
      1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ (63)
        a b c d e f g h

*/

use crate::pieces::{Pieces, Piece};
use crate::coordinate::Coordinate;
use crate::board::Board;

pub struct Rules { }

impl Rules {

    pub fn is_basic_movement_valid (board: &Board, from: &Coordinate, to: &Coordinate) -> bool {
        let from_index = from.as_index();
        let to_index = to.as_index();
        if from_index == to_index { return false; }
        let setup = board.get_current_setup();
        let from_piece = &setup[from_index];
        let to_piece = &setup[to_index];
        let (from_x, from_y) = from.as_x_y();
        let (to_x, to_y) = to.as_x_y();

        match from_piece.piece_type {
            Pieces::WhitePawn => {
                if to_piece.is_empty() {
                    // white pawn can move to an empty place:
                    //     - if it is in the same column and in the next row
                    //     - if it is in the same column and in row 3, the pawn is in row 1, and the place in between is empty
                    if from_x != to_x { return false; } // cannot step onto an empty field not in the same column
                    if from_y >= to_y { return false; } // can only step to a row with larger index
                    if from_y + 1 == to_y { return true; }
                    if (from_y != 1) || (to_y != 3) { return false; }
                    if setup[Coordinate::convert_coordinates_to_index(from_x, from_y + 1)].piece_type == Pieces::Empty { return true; }
                    return false
                }
                else if to_piece.is_black() {
                    // move diagonally and capture another piece
                    if to_y == from_y + 1 {
                        if (from_x != 7) && (to_x == from_x + 1) { return true; }
                        if (from_x != 0) && (to_x == from_x - 1) { return true; }
                    }
                    return false;
                }
                // white
                false
            },
            Pieces::WhiteRook => {
                if to_piece.is_white() { return false; } // cannot take white piece
                if from_x == to_x { return Rules::is_clear_vertical(board, from_x, from_y, to_y); }
                if from_y == to_y { return Rules::is_clear_horizontal(board, from_y, from_x, to_x); }
                false
            },
            Pieces::WhiteKnight => {
                if to_piece.is_white() { return false; } // cannot take white piece
                let dx = (from_x as i8 - to_x as i8).abs();
                let dy = (from_y as i8 - to_y as i8).abs();
                dx * dy == 2
            },
            Pieces::WhiteBishop => {
                if to_piece.is_white() { return false; } // cannot take white piece
                if (from_x as i8 - to_x as i8).abs() == (from_y as i8 - to_y as i8).abs() {
                    return Rules::is_clear_diagonal(board, from_x, from_y, to_x, to_y);
                }
                false
            },
            Pieces::WhiteQueen => {
                if to_piece.is_white() { return false; } // cannot take white piece
                if from_x == to_x { return Rules::is_clear_vertical(board, from_x, from_y, to_y); }
                if from_y == to_y { return Rules::is_clear_horizontal(board, from_y, from_x, to_x); }
                if (from_x as i8 - to_x as i8).abs() == (from_y as i8 - to_y as i8).abs() {
                    return Rules::is_clear_diagonal(board, from_x, from_y, to_x, to_y);
                }
                false
            },
            Pieces::WhiteKing => {
                if to_piece.is_white() { return false; } // cannot take white piece
                let dx = (from_x as i8 - to_x as i8).abs();
                let dy = (from_y as i8 - to_y as i8).abs();
                dx <= 1 && dy <= 1
            },
            Pieces::BlackPawn => {
                if to_piece.is_empty() {
                    // black pawn can move to an empty place:
                    //     - if it is in the same column and in the next row
                    //     - if it is in the same column and in row 4, the pawn is in row 6, and the place in between is empty
                    if from_x != to_x { return false; } // cannot step onto an empty field not in the same column
                    if from_y <= to_y { return false; } // can only step to a row with smaller index
                    if from_y - 1 == to_y { return true; }
                    if (from_y != 6) || (to_y != 4) { return false; }
                    if setup[Coordinate::convert_coordinates_to_index(from_x, from_y - 1)].piece_type == Pieces::Empty { return true; }
                    return false
                }
                else if to_piece.is_white() {
                    // move diagonally and capture another piece
                    if to_y == from_y - 1 {
                        if (from_x != 7) && (to_x == from_x + 1) { return true; }
                        if (from_x != 0) && (to_x == from_x - 1) { return true; }
                    }
                    return false;
                }
                // black
                false
            },
            Pieces::BlackRook => {
                if to_piece.is_black() { return false; } // cannot take black piece
                if from_x == to_x { return Rules::is_clear_vertical(board, from_x, from_y, to_y); }
                if from_y == to_y { return Rules::is_clear_horizontal(board, from_y, from_x, to_x); }
                false
            },
            Pieces::BlackKnight => {
                if to_piece.is_black() { return false; } // cannot take black piece
                let dx = (from_x as i8 - to_x as i8).abs();
                let dy = (from_y as i8 - to_y as i8).abs();
                dx * dy == 2
            },
            Pieces::BlackBishop => {
                if to_piece.is_black() { return false; } // cannot take black piece
                if (from_x as i8 - to_x as i8).abs() == (from_y as i8 - to_y as i8).abs() {
                    return Rules::is_clear_diagonal(board, from_x, from_y, to_x, to_y);
                }
                false
            },
            Pieces::BlackQueen => {
                if to_piece.is_black() { return false; } // cannot take black piece
                if from_x == to_x { return Rules::is_clear_vertical(board, from_x, from_y, to_y); }
                if from_y == to_y { return Rules::is_clear_horizontal(board, from_y, from_x, to_x); }
                if (from_x as i8 - to_x as i8).abs() == (from_y as i8 - to_y as i8).abs() {
                    return Rules::is_clear_diagonal(board, from_x, from_y, to_x, to_y);
                }
                false
            },
            Pieces::BlackKing => {
                if to_piece.is_black() { return false; } // cannot take black piece
                let dx = (from_x as i8 - to_x as i8).abs();
                let dy = (from_y as i8 - to_y as i8).abs();
                dx <= 1 && dy <= 1
            }
            _=> false
        }
    }

    fn is_clear_horizontal(board: &Board, y: u8, x1: u8, x2: u8) -> bool {
        let setup = board.get_current_setup();
        let (start, end) = if x1 < x2 { (x1 + 1, x2) } else { (x2 + 1, x1) };
        for x in start..end {
            let idx = Coordinate::convert_coordinates_to_index(x, y);
            if setup[idx].piece_type != Pieces::Empty {
                return false;
            }
        }
        true
    }

    fn is_clear_vertical(board: &Board, x: u8, y1: u8, y2: u8) -> bool {
        let setup = board.get_current_setup();
        let (start, end) = if y1 < y2 { (y1 + 1, y2) } else { (y2 + 1, y1) };
        for y in start..end {
            let idx = Coordinate::convert_coordinates_to_index(x, y);
            if setup[idx].piece_type != Pieces::Empty {
                return false;
            }
        }
        true
    }

    fn is_clear_diagonal(board: &Board, from_x: u8, from_y: u8, to_x: u8, to_y: u8) -> bool {
        let setup = board.get_current_setup();
        let dx = if to_x > from_x { 1 } else { -1 };
        let dy = if to_y > from_y { 1 } else { -1 };
        let mut x = from_x as i8 + dx;
        let mut y = from_y as i8 + dy;

        while x != to_x as i8 && y != to_y as i8 {
            let idx = Coordinate::convert_coordinates_to_index(x as u8, y as u8);
            if setup[idx].piece_type != Pieces::Empty {
                return false;
            }
            x += dx;
            y += dy;
        }
        true
    }

}