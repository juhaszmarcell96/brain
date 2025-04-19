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

/*

♚ -> white king   -> -6
♛ -> white queen  -> -5
♝ -> white bishop -> -4
♞ -> white knight -> -3
♜ -> white rook   -> -2
♟ -> white pawn   -> -1
. -> empty field  ->  0
♙ -> black pawn   ->  1
♖ -> black rook   ->  2
♘ -> black knight ->  3
♗ -> black bishop ->  4
♕ -> black queen  ->  5
♔ -> black king   ->  6

*/

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Pieces {
    WhitePawn,
    WhiteRook,
    WhiteKnight,
    WhiteBishop,
    WhiteQueen,
    WhiteKing,
    Empty,
    BlackPawn,
    BlackRook,
    BlackKnight,
    BlackBishop,
    BlackQueen,
    BlackKing
}

impl Pieces {
    fn value(&self) -> i8 {
        match self {
            Pieces::WhiteKing   => -6,
            Pieces::WhiteQueen  => -5,
            Pieces::WhiteBishop => -4,
            Pieces::WhiteKnight => -3,
            Pieces::WhiteRook   => -2,
            Pieces::WhitePawn   => -1,
            Pieces::Empty       =>  0,
            Pieces::BlackPawn   =>  1,
            Pieces::BlackRook   =>  2,
            Pieces::BlackKnight =>  3,
            Pieces::BlackBishop =>  4,
            Pieces::BlackQueen  =>  5,
            Pieces::BlackKing   =>  6
        }
    }
    fn symbol(&self) -> char {
        match self {
            Pieces::WhiteKing   => '♔',
            Pieces::WhiteQueen  => '♕',
            Pieces::WhiteBishop => '♗',
            Pieces::WhiteKnight => '♘',
            Pieces::WhiteRook   => '♖',
            Pieces::WhitePawn   => '♙',
            Pieces::Empty       => ' ',
            Pieces::BlackPawn   => '♟',
            Pieces::BlackRook   => '♜',
            Pieces::BlackKnight => '♞',
            Pieces::BlackBishop => '♝',
            Pieces::BlackQueen  => '♛',
            Pieces::BlackKing   => '♚'
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub piece_type: Pieces,
    pub moved: bool,
    x: u8,
    y: u8
}

impl Piece {
    pub fn new (t: Pieces, x: u8, y: u8) -> Self {
        Self {
            piece_type: t,
            moved: false,
            x,
            y
        }
    }

    pub fn get_x (&self) -> u8 { self.x }
    pub fn get_y (&self) -> u8 { self.y }

    pub fn draw (&self) {
        print!("{}", self.piece_type.symbol());
    }

    pub fn value (&self) -> i8 {
        self.piece_type.value()
    }

    pub fn symbol (&self) -> char {
        self.piece_type.symbol()
    }
    
    pub fn is_black (&self) -> bool {
        self.piece_type == Pieces::BlackPawn ||
        self.piece_type == Pieces::BlackRook ||
        self.piece_type == Pieces::BlackKnight ||
        self.piece_type == Pieces::BlackBishop ||
        self.piece_type == Pieces::BlackQueen ||
        self.piece_type == Pieces::BlackKing
    }
    
    pub fn is_white (&self) -> bool {
        self.piece_type == Pieces::WhitePawn ||
        self.piece_type == Pieces::WhiteRook ||
        self.piece_type == Pieces::WhiteKnight ||
        self.piece_type == Pieces::WhiteBishop ||
        self.piece_type == Pieces::WhiteQueen ||
        self.piece_type == Pieces::WhiteKing
    }
    
    pub fn is_empty (&self) -> bool {
        self.piece_type == Pieces::Empty
    }

    pub fn capture (&mut self) {
        self.piece_type = Pieces::Empty;
    }
}
