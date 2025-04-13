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
♙ -> black rook   ->  2
♘ -> black knight ->  3
♗ -> black bishop ->  4
♕ -> black queen  ->  5
♔ -> black king   ->  6

*/

#[derive(Debug)]
enum Pieces {
    WhitePawn,
    WhiteRook,
    WhiteKnight,
    WhiteBishop,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackRook,
    BlackKnight,
    BlackBishop,
    BlackQueen,
    BlackKing
}