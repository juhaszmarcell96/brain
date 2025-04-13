pub mod pieces;
pub mod board;

fn main() {
    let mut chessboard = board::Board::new();
    chessboard.draw();
    chessboard.select('b', 1);
    chessboard.to('c', 3);
    chessboard.draw();
}
