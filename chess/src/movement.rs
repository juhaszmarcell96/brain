use crate::pieces::Piece;
use crate::coordinate::Coordinate;

pub struct Movement {
    pub from: Coordinate,
    pub to: Coordinate,
    pub taken: Option<Piece>
}

impl Movement {
    pub fn new (from_index: usize, to_index: usize, taken: Option<Piece>) -> Self {
        Self {
            from: Coordinate::from_index(from_index),
            to: Coordinate::from_index(to_index),
            taken
        }
    }
}