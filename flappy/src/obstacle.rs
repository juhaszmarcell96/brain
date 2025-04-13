use game::bounding_box::BoundingBox;

#[derive(Clone)]
pub struct Obstacles {
    pub upper : BoundingBox,
    pub lower : BoundingBox
}

/*
        x
--------+-----+---------
        |     |       ^
        |     |       |
      h |     |       |
        |  w  |       |
        +-----+       |
                      |
         gap         full height (fh)
                      |
      y +-----+       |
        |  w  |       |
        |     |       |
        |     |       v
--------+-----+---------

*/

impl Obstacles {
    pub fn new (fh: f32, x: f32, w: f32, h: f32, gap: f32) -> Self {
        let y = h + gap;
        let h_lower = fh - gap - h;
        Self {
            upper: BoundingBox::new(x, 0.0, w, h),
            lower: BoundingBox::new(x, y, w, h_lower)
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