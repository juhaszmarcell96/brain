use game::bounding_box::BoundingBox;

#[derive(Clone)]
pub struct Obstacles {
    pub upper : BoundingBox,
    pub lower : BoundingBox
}

impl Obstacles {
    pub fn new (x: f32, y: f32, w: f32, h1: f32, h2: f32) -> Self {
        Self {
            upper: BoundingBox::new(x, 0.0, w, h1),
            lower: BoundingBox::new(x, y, w, h2)
        }
    }

    pub fn change_position (&mut self, dx : f32, dy : f32) {
        self.upper.change_position(dx, dy);
        self.lower.change_position(dx, dy);
    }

    pub fn x (&self) -> f32 {
        self.upper.origin.x
    }
}