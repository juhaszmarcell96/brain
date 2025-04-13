pub mod pieces;
pub mod board;

fn main() {
    let mut chessboard = board::Board::new();
    chessboard.draw();
}
