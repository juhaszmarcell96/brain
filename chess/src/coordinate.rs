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

#[derive(Debug, Clone, Copy)]
pub struct Coordinate {
    index: usize
}

impl Coordinate {
    pub fn from_index (index: usize) -> Self {
        if index >= 64 { panic!("index out of range"); }
        Self {
            index
        }
    }
    pub fn from_row_col (col: char, row: u8) -> Self {
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
        Self {
            index: ((8 - row) as usize) * 8 + (col_int as usize)
        }
    }
    pub fn from_x_y (x: u8, y: u8) -> Self {
        if x >= 8 { panic!("x out of range"); }
        if y >= 8 { panic!("y out of range"); }
        Self {
            index: (y as usize) * 8 + (x as usize)
        }
    }

    pub fn as_index (&self) -> usize {
        self.index
    }
    pub fn as_x_y (&self) -> (u8, u8) {
        ((self.index as u8) % 8, (self.index as u8) / 8)
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

    pub fn convert_index_to_coordinates (index: usize) -> (u8, u8) {
        if index > 63 { panic!("invalid index"); }
        ((index as u8) % 8, (index as u8) / 8)
    }

    pub fn convert_coordinates_to_index (x: u8, y: u8) -> usize {
        if x >= 8 { panic!("x out of range"); }
        if y >= 8 { panic!("y out of range"); }
        (y as usize) * 8 + (x as usize)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn coordinate_conversion_test () {
        assert_eq!(Coordinate::convert_coordinates('a', 8), (0, 0, 0));
        assert_eq!(Coordinate::convert_coordinates('h', 1), (7, 7, 63));
        assert_eq!(Coordinate::convert_coordinates('c', 3), (2, 5, 42));
        
        assert_eq!(Coordinate::convert_index_to_coordinates(0), (0, 0));
        assert_eq!(Coordinate::convert_index_to_coordinates(63), (7, 7));
        assert_eq!(Coordinate::convert_index_to_coordinates(42), (2, 5));

        assert_eq!(Coordinate::convert_coordinates_to_index(0, 0), 0);
        assert_eq!(Coordinate::convert_coordinates_to_index(7, 7), 63);
        assert_eq!(Coordinate::convert_coordinates_to_index(2, 5), 42);
    }
}
