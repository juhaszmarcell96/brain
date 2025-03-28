
#[derive(Debug, Clone, PartialEq)]
pub struct Dimensions {
    pub w: f32,
    pub h: f32
}

impl Dimensions {
    pub fn new(w: f32, h: f32) -> Self {
        if w < 0.0 || h < 0.0 {
            panic!("cannot have negative dimensions");
        }
        Self { w, h }
    }
}