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
    turn: Turn,
    selected: Option<usize>
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
            turn: Turn::White,
            selected: None
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

    pub fn select (&mut self, col: char, row: u8) {
        let (_, _, index) = Coordinate::convert_coordinates(col, row);
        self.selected = Some(index);
    }

    pub fn move_to (&mut self, col: char, row: u8) {
        let (_, _, to_index) = Coordinate::convert_coordinates(col, row);
        let from_index = self.selected.expect("no piece is selected");
        self.selected = None;
        if from_index == to_index { return; }
        self.pieces[to_index].piece_type = self.pieces[from_index].piece_type;
        self.pieces[to_index].moved = true;
        self.pieces[from_index].piece_type = Pieces::Empty;
        self.pieces[from_index].moved = false;
        // flip the turn -> assume correct usage (move was checked with 'can_move_to' beforehand
        self.flip_turn();

    }

    pub fn make_a_move (&mut self, from_col: char, from_row: u8, to_col: char, to_row: u8) -> bool {
        let from_coordinate = Coordinate::from_row_col(from_col, from_row);
        let to_coordinate = Coordinate::from_row_col(to_col, to_row);
        let from_index = from_coordinate.as_index();
        let to_index = from_coordinate.as_index();
        // moving a piece to the same place is invalid
        if from_index == to_index { return false; }
        let from_piece = &self.pieces[from_index];
        let to_piece = &self.pieces[to_index];
        // an empty piece cannot be moved
        if from_piece.is_empty() { return false; }
        // consider the turn, e.g. white piece cannot be moved in a black turn
        if !self.ignore_turn {
            if (self.turn == Turn::White) && from_piece.is_black() { return false; }
            if (self.turn == Turn::Black) && from_piece.is_white() { return false; }
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

    pub fn can_move_to (&self, col: char, row: u8) -> bool {
        let from_index = self.selected.expect("no piece is selected");
        let to_coordinate = Coordinate::from_row_col(col, row);
        let to_index = to_coordinate.as_index();
        if from_index == to_index { return false; }
        let from_piece = &self.pieces[from_index];
        let to_piece = &self.pieces[to_index];
        let from_x = from_piece.get_x();
        let from_y = from_piece.get_y();
        let (to_x, to_y) = to_coordinate.as_x_y();

        debug_log!("moving from [{};{}] to [{};{}]", from_x, from_y, to_x, to_y);

        if from_piece.is_empty() { return false; }
        if !self.ignore_turn {
            if (self.turn == Turn::White) && from_piece.is_black() { return false; }
            if (self.turn == Turn::Black) && from_piece.is_white() { return false; }
        }

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
                    if self.pieces[Coordinate::convert_coordinates_to_index(from_x, from_y + 1)].piece_type == Pieces::Empty { return true; }
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
                if from_x == to_x { return self.is_clear_vertical(from_x, from_y, to_y); }
                if from_y == to_y { return self.is_clear_horizontal(from_y, from_x, to_x); }
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
                    return self.is_clear_diagonal(from_x, from_y, to_x, to_y);
                }
                false
            },
            Pieces::WhiteQueen => {
                if to_piece.is_white() { return false; } // cannot take white piece
                if from_x == to_x { return self.is_clear_vertical(from_x, from_y, to_y); }
                if from_y == to_y { return self.is_clear_horizontal(from_y, from_x, to_x); }
                if (from_x as i8 - to_x as i8).abs() == (from_y as i8 - to_y as i8).abs() {
                    return self.is_clear_diagonal(from_x, from_y, to_x, to_y);
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
                    if self.pieces[Coordinate::convert_coordinates_to_index(from_x, from_y - 1)].piece_type == Pieces::Empty { return true; }
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
                if from_x == to_x { return self.is_clear_vertical(from_x, from_y, to_y); }
                if from_y == to_y { return self.is_clear_horizontal(from_y, from_x, to_x); }
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
                    return self.is_clear_diagonal(from_x, from_y, to_x, to_y);
                }
                false
            },
            Pieces::BlackQueen => {
                if to_piece.is_black() { return false; } // cannot take black piece
                if from_x == to_x { return self.is_clear_vertical(from_x, from_y, to_y); }
                if from_y == to_y { return self.is_clear_horizontal(from_y, from_x, to_x); }
                if (from_x as i8 - to_x as i8).abs() == (from_y as i8 - to_y as i8).abs() {
                    return self.is_clear_diagonal(from_x, from_y, to_x, to_y);
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

    fn is_clear_horizontal(&self, y: u8, x1: u8, x2: u8) -> bool {
        let (start, end) = if x1 < x2 { (x1 + 1, x2) } else { (x2 + 1, x1) };
        for x in start..end {
            let idx = Coordinate::convert_coordinates_to_index(x, y);
            if self.pieces[idx].piece_type != Pieces::Empty {
                return false;
            }
        }
        true
    }

    fn is_clear_vertical(&self, x: u8, y1: u8, y2: u8) -> bool {
        let (start, end) = if y1 < y2 { (y1 + 1, y2) } else { (y2 + 1, y1) };
        for y in start..end {
            let idx = Coordinate::convert_coordinates_to_index(x, y);
            if self.pieces[idx].piece_type != Pieces::Empty {
                return false;
            }
        }
        true
    }

    fn is_clear_diagonal(&self, from_x: u8, from_y: u8, to_x: u8, to_y: u8) -> bool {
        let dx = if to_x > from_x { 1 } else { -1 };
        let dy = if to_y > from_y { 1 } else { -1 };
        let mut x = from_x as i8 + dx;
        let mut y = from_y as i8 + dy;

        while x != to_x as i8 && y != to_y as i8 {
            let idx = Coordinate::convert_coordinates_to_index(x as u8, y as u8);
            if self.pieces[idx].piece_type != Pieces::Empty {
                return false;
            }
            x += dx;
            y += dy;
        }
        true
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn white_pawn_move_test () {
        let mut board = Board::new();
        board.set_ignore_turn(true);
        board.select('e', 7);
        for col in "abcdefgh".chars() {
            for row in 1..8 {
                //debug_log!("col {} row {}", col, row);
                if col == 'e' && row == 6 { assert!(board.can_move_to(col, row)); } // single step
                else if col == 'e' && row == 5 { assert!(board.can_move_to(col, row)); } // double step
                else { assert!(!board.can_move_to(col, row)); }
            }
        }
        board.move_to('e', 5);
        board.select('e', 5);
        for col in "abcdefgh".chars() {
            for row in 1..8 {
                //debug_log!("col {} row {}", col, row);
                if col == 'e' && row == 4 { assert!(board.can_move_to(col, row)); } // single step
                else { assert!(!board.can_move_to(col, row)); }
            }
        }
        board.move_to('e', 3);
        board.select('e', 3);
        for col in "abcdefgh".chars() {
            for row in 1..8 {
                //debug_log!("col {} row {}", col, row);
                if col == 'd' && row == 2 { assert!(board.can_move_to(col, row)); } // single step
                else if col == 'f' && row == 2 { assert!(board.can_move_to(col, row)); } // double step
                else { assert!(!board.can_move_to(col, row)); }
            }
        }
    }
    
    #[test]
    fn white_rook_move_test () {
        let mut board = Board::new();
        board.set_ignore_turn(true);
        board.select('a', 8);
        for col in "abcdefgh".chars() {
            for row in 1..8 {
                // cannot move anywhere from the initial position
                assert!(!board.can_move_to(col, row));
            }
        }
        //8   ♘ ♗ ♕ ♔   ♘ ♖ 
        //7 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙ 
        //6                 
        //5   ♞   ♖     ♗   
        //4                 
        //3                 
        //2 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟ 
        //1 ♜   ♝ ♛ ♚ ♝ ♞ ♜ 
        //  a b c d e f g h
        board.move_to('d', 5);
        board.teleport('b', 1, 'b', 5);
        board.teleport('f', 8, 'g', 5);
        board.select('d', 5);
        for col in "abcdefgh".chars() {
            for row in 1..8 {
                if col == 'd' && row == 6 { assert!(board.can_move_to(col, row)); }
                else if col == 'd' && row == 4 { assert!(board.can_move_to(col, row)); }
                else if col == 'd' && row == 3 { assert!(board.can_move_to(col, row)); }
                else if col == 'd' && row == 2 { assert!(board.can_move_to(col, row)); }
                else if col == 'b' && row == 5 { assert!(board.can_move_to(col, row)); }
                else if col == 'c' && row == 5 { assert!(board.can_move_to(col, row)); }
                else if col == 'e' && row == 5 { assert!(board.can_move_to(col, row)); }
                else if col == 'f' && row == 5 { assert!(board.can_move_to(col, row)); }
                else { assert!(!board.can_move_to(col, row)); }
            }
        }
    }
    
    #[test]
    fn white_knight_move_test () {
        let mut board = Board::new();
        board.set_ignore_turn(true);
        board.select('b', 8);
        for col in "abcdefgh".chars() {
            for row in 1..8 {
                if col == 'a' && row == 6 { assert!(board.can_move_to(col, row)); } // single step
                else if col == 'c' && row == 6 { assert!(board.can_move_to(col, row)); } // double step
                else { assert!(!board.can_move_to(col, row)); }
            }
        }
        //8 ♖   ♗ ♕ ♔ ♗ ♘ ♖ 
        //7 ♙ ♙ ♙ ♙ ♙   ♙ ♙ 
        //6                 
        //5                 
        //4           ♙     
        //3       ♘         
        //2 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟ 
        //1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ 
        //  a b c d e f g h
        board.move_to('d', 3);
        board.teleport('f', 7, 'f', 4);
        board.select('d', 3);
        for col in "abcdefgh".chars() {
            for row in 1..8 {
                if col == 'c' && row == 1 { assert!(board.can_move_to(col, row)); }
                else if col == 'e' && row == 1 { assert!(board.can_move_to(col, row)); }
                else if col == 'b' && row == 2 { assert!(board.can_move_to(col, row)); }
                else if col == 'f' && row == 2 { assert!(board.can_move_to(col, row)); }
                else if col == 'b' && row == 4 { assert!(board.can_move_to(col, row)); }
                else if col == 'c' && row == 5 { assert!(board.can_move_to(col, row)); }
                else if col == 'e' && row == 5 { assert!(board.can_move_to(col, row)); }
                else { assert!(!board.can_move_to(col, row)); }
            }
        }
    }
    
    #[test]
    fn white_bishop_move_test () {
        let mut board = Board::new();
        board.set_ignore_turn(true);
        board.select('c', 8);
        for col in "abcdefgh".chars() {
            for row in 1..8 {
                // cannot move anywhere from the initial position
                assert!(!board.can_move_to(col, row));
            }
        }
        //8 ♖ ♘   ♕ ♔ ♗ ♘ ♖ 
        //7 ♙ ♙ ♙ ♙ ♙   ♙ ♙ 
        //6           ♙     
        //5                 
        //4       ♗         
        //3   ♟             
        //2 ♟   ♟ ♟ ♟ ♟ ♟ ♟ 
        //1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ 
        //  a b c d e f g h
        board.move_to('d', 4);
        board.teleport('f', 7, 'f', 6);
        board.teleport('b', 2, 'b', 3);
        board.select('d', 4);
        for col in "abcdefgh".chars() {
            for row in 1..8 {
                if col == 'c' && row == 5 { assert!(board.can_move_to(col, row)); }
                else if col == 'b' && row == 6 { assert!(board.can_move_to(col, row)); }
                else if col == 'e' && row == 5 { assert!(board.can_move_to(col, row)); }
                else if col == 'c' && row == 3 { assert!(board.can_move_to(col, row)); }
                else if col == 'b' && row == 2 { assert!(board.can_move_to(col, row)); }
                else if col == 'a' && row == 1 { assert!(board.can_move_to(col, row)); }
                else if col == 'e' && row == 3 { assert!(board.can_move_to(col, row)); }
                else if col == 'f' && row == 2 { assert!(board.can_move_to(col, row)); }
                else { assert!(!board.can_move_to(col, row)); }
            }
        }
    }
    
    #[test]
    fn white_queen_move_test () {
        let mut board = Board::new();
        board.set_ignore_turn(true);
        board.select('d', 8);
        for col in "abcdefgh".chars() {
            for row in 1..8 {
                // cannot move anywhere from the initial position
                assert!(!board.can_move_to(col, row));
            }
        }
        //8 ♖ ♘ ♗   ♔ ♗ ♘ ♖ 
        //7 ♙ ♙ ♙ ♙ ♙   ♙   
        //6           ♙     
        //5                 
        //4       ♕       ♙ 
        //3   ♟             
        //2 ♟   ♟ ♟ ♟ ♟ ♟ ♟ 
        //1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ 
        //  a b c d e f g h
        board.move_to('d', 4);
        board.teleport('f', 7, 'f', 6);
        board.teleport('b', 2, 'b', 3);
        board.teleport('h', 7, 'h', 4);
        board.select('d', 4);
        for col in "abcdefgh".chars() {
            for row in 1..8 {
                if col == 'c' && row == 5 { assert!(board.can_move_to(col, row)); }
                else if col == 'b' && row == 6 { assert!(board.can_move_to(col, row)); }
                else if col == 'e' && row == 5 { assert!(board.can_move_to(col, row)); }
                else if col == 'c' && row == 3 { assert!(board.can_move_to(col, row)); }
                else if col == 'b' && row == 2 { assert!(board.can_move_to(col, row)); }
                else if col == 'a' && row == 1 { assert!(board.can_move_to(col, row)); }
                else if col == 'e' && row == 3 { assert!(board.can_move_to(col, row)); }
                else if col == 'f' && row == 2 { assert!(board.can_move_to(col, row)); }
                else if col == 'a' && row == 4 { assert!(board.can_move_to(col, row)); }
                else if col == 'b' && row == 4 { assert!(board.can_move_to(col, row)); }
                else if col == 'c' && row == 4 { assert!(board.can_move_to(col, row)); }
                else if col == 'e' && row == 4 { assert!(board.can_move_to(col, row)); }
                else if col == 'f' && row == 4 { assert!(board.can_move_to(col, row)); }
                else if col == 'g' && row == 4 { assert!(board.can_move_to(col, row)); }
                else if col == 'd' && row == 6 { assert!(board.can_move_to(col, row)); }
                else if col == 'd' && row == 5 { assert!(board.can_move_to(col, row)); }
                else if col == 'd' && row == 3 { assert!(board.can_move_to(col, row)); }
                else if col == 'd' && row == 2 { assert!(board.can_move_to(col, row)); }
                else { assert!(!board.can_move_to(col, row)); }
            }
        }
    }
    
    #[test]
    fn white_king_move_test () {
        let mut board = Board::new();
        board.set_ignore_turn(true);
        board.select('e', 8);
        for col in "abcdefgh".chars() {
            for row in 1..8 {
                // cannot move anywhere from the initial position
                assert!(!board.can_move_to(col, row));
            }
        }
        //8 ♖ ♘ ♗ ♕   ♗ ♘ ♖ 
        //7 ♙ ♙ ♙ ♙ ♙   ♙ ♙ 
        //6                 
        //5           ♙     
        //4         ♔       
        //3         ♟       
        //2 ♟ ♟ ♟ ♟   ♟ ♟ ♟ 
        //1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ 
        //  a b c d e f g h
        board.move_to('e', 4);
        board.teleport('e', 2, 'e', 3);
        board.teleport('f', 7, 'f', 5);
        board.select('e', 4);
        for col in "abcdefgh".chars() {
            for row in 1..8 {
                if col == 'e' && row == 5 { assert!(board.can_move_to(col, row)); }
                else if col == 'd' && row == 5 { assert!(board.can_move_to(col, row)); }
                else if col == 'd' && row == 4 { assert!(board.can_move_to(col, row)); }
                else if col == 'd' && row == 3 { assert!(board.can_move_to(col, row)); }
                else if col == 'e' && row == 3 { assert!(board.can_move_to(col, row)); }
                else if col == 'f' && row == 3 { assert!(board.can_move_to(col, row)); }
                else if col == 'f' && row == 4 { assert!(board.can_move_to(col, row)); }
                else { assert!(!board.can_move_to(col, row)); }
            }
        }
    }
}
