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
    turn: Turn
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
            turn: Turn::White
        }
    }

    pub fn draw (&self) {
        print!("8 {} {} {} {} {} {} {} {}\n", self.pieces[ 0].symbol(), self.pieces[ 1].symbol(), self.pieces[ 2].symbol(), self.pieces[ 3].symbol(), self.pieces[ 4].symbol(), self.pieces[ 5].symbol(), self.pieces[ 6].symbol(), self.pieces[ 7].symbol());
        print!("7 {} {} {} {} {} {} {} {}\n", self.pieces[ 8].symbol(), self.pieces[ 9].symbol(), self.pieces[10].symbol(), self.pieces[11].symbol(), self.pieces[12].symbol(), self.pieces[13].symbol(), self.pieces[14].symbol(), self.pieces[15].symbol());
        print!("6 {} {} {} {} {} {} {} {}\n", self.pieces[16].symbol(), self.pieces[17].symbol(), self.pieces[18].symbol(), self.pieces[19].symbol(), self.pieces[20].symbol(), self.pieces[21].symbol(), self.pieces[22].symbol(), self.pieces[23].symbol());
        print!("5 {} {} {} {} {} {} {} {}\n", self.pieces[24].symbol(), self.pieces[25].symbol(), self.pieces[26].symbol(), self.pieces[27].symbol(), self.pieces[28].symbol(), self.pieces[29].symbol(), self.pieces[30].symbol(), self.pieces[31].symbol());
        print!("4 {} {} {} {} {} {} {} {}\n", self.pieces[32].symbol(), self.pieces[33].symbol(), self.pieces[34].symbol(), self.pieces[35].symbol(), self.pieces[36].symbol(), self.pieces[37].symbol(), self.pieces[38].symbol(), self.pieces[38].symbol());
        print!("3 {} {} {} {} {} {} {} {}\n", self.pieces[40].symbol(), self.pieces[41].symbol(), self.pieces[42].symbol(), self.pieces[43].symbol(), self.pieces[44].symbol(), self.pieces[45].symbol(), self.pieces[46].symbol(), self.pieces[47].symbol());
        print!("2 {} {} {} {} {} {} {} {}\n", self.pieces[48].symbol(), self.pieces[49].symbol(), self.pieces[50].symbol(), self.pieces[51].symbol(), self.pieces[52].symbol(), self.pieces[53].symbol(), self.pieces[54].symbol(), self.pieces[55].symbol());
        print!("1 {} {} {} {} {} {} {} {}\n", self.pieces[56].symbol(), self.pieces[57].symbol(), self.pieces[58].symbol(), self.pieces[59].symbol(), self.pieces[60].symbol(), self.pieces[61].symbol(), self.pieces[62].symbol(), self.pieces[63].symbol());
        print!("  a b c d e f g h\n");
    }
}