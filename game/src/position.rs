
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}