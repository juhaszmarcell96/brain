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

enum Turn {
    White,
    Black
}

pub struct Board {
    pieces: [Piece; 64],
    turn: Turn,
    selected: Option<usize>
}

impl Board {
    pub fn new () -> Self {
        Self {
            pieces: [
                Piece::new(Pieces::BlackRook, 0, 0),
                Piece::new(Pieces::BlackKnight, 1, 0),
                Piece::new(Pieces::BlackBishop, 2, 0),
                Piece::new(Pieces::BlackQueen, 3, 0),
                Piece::new(Pieces::BlackKing, 4, 0),
                Piece::new(Pieces::BlackBishop, 5, 0),
                Piece::new(Pieces::BlackKnight, 6, 0),
                Piece::new(Pieces::BlackRook, 7, 0),
                Piece::new(Pieces::BlackPawn, 0, 1),
                Piece::new(Pieces::BlackPawn, 1, 1),
                Piece::new(Pieces::BlackPawn, 2, 1),
                Piece::new(Pieces::BlackPawn, 3, 1),
                Piece::new(Pieces::BlackPawn, 4, 1),
                Piece::new(Pieces::BlackPawn, 5, 1),
                Piece::new(Pieces::BlackPawn, 6, 1),
                Piece::new(Pieces::BlackPawn, 7, 1),
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
                Piece::new(Pieces::WhitePawn, 0, 6),
                Piece::new(Pieces::WhitePawn, 1, 6),
                Piece::new(Pieces::WhitePawn, 2, 6),
                Piece::new(Pieces::WhitePawn, 3, 6),
                Piece::new(Pieces::WhitePawn, 4, 6),
                Piece::new(Pieces::WhitePawn, 5, 6),
                Piece::new(Pieces::WhitePawn, 6, 6),
                Piece::new(Pieces::WhitePawn, 7, 6),
                Piece::new(Pieces::WhiteRook, 0, 7),
                Piece::new(Pieces::WhiteKnight, 1, 7),
                Piece::new(Pieces::WhiteBishop, 2, 7),
                Piece::new(Pieces::WhiteQueen, 3, 7),
                Piece::new(Pieces::WhiteKing, 4, 7),
                Piece::new(Pieces::WhiteBishop, 5, 7),
                Piece::new(Pieces::WhiteKnight, 6, 7),
                Piece::new(Pieces::WhiteRook, 7, 7)
            ],
            turn: Turn::White,
            selected: None
        }
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

    pub fn convert_coordinates (col: char, row: u8) -> (u8, u8, usize) {
        if row < 1 { panic!("row too small"); }
        if row > 8 { panic!("row too large"); }
        let col_int = match col {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => panic!("invalid column"),
        };
        (col_int, 8 - row, ((8 - row) as usize) * 8 + (col_int as usize))
    }

    pub fn select (&mut self, col: char, row: u8) {
        if row < 1 { panic!("row too small"); }
        if row > 8 { panic!("row too large"); }
        let col_int = match col {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => panic!("invalid column"),
        };
        let index = ((8 - row) as usize) * 8 + col_int;
        self.selected = Some(index);
    }

    pub fn to (&mut self, col: char, row: u8) {
        if self.selected.is_none() { panic!("nothing is selected"); }
        if row < 1 { panic!("row too small"); }
        if row > 8 { panic!("row too large"); }
        let col_int = match col {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => panic!("invalid column"),
        };
        let index = ((8 - row) as usize) * 8 + col_int;
        let from = self.selected.expect("no piece is selected");
        if from != index {
            let (a, b) = self.pieces.split_at_mut(std::cmp::max(from, index));
            if from < index {
                a[from].move_to(&mut b[0]);
            } else {
                b[0].move_to(&mut a[index]);
            }
        }
        self.selected = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn coordinate_conversion_test () {
        assert_eq!(Board::convert_coordinates('a', 8), (0, 0, 0));
        assert_eq!(Board::convert_coordinates('h', 1), (7, 7, 63));
        assert_eq!(Board::convert_coordinates('c', 3), (2, 5, 42));
    }
}
