use crate::game::position::Position;
use crate::game::dimensions::Dimensions;

#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub origin: Position,
    pub dimensions: Dimensions
}

impl BoundingBox {
    pub fn new (x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            origin: Position::new(x, y),
            dimensions: Dimensions::new(w, h),
        }
    }

    fn is_inside (&self, x: f32, y: f32) -> bool {
        if x < self.origin.x { return false; }
        if y < self.origin.y { return false; }
        if x > self.origin.x + self.dimensions.w { return false; }
        if y > self.origin.y + self.dimensions.h { return false; }
        true
    }

    fn is_position_inside (&self, pos: Position) -> bool {
        self.is_inside(pos.x, pos.y)
    }

    pub fn is_colliding_with (&self, other: &BoundingBox) -> bool {
        // rather than checking if they collide, check if they are separated and negate the result
        !((( self.origin.x +  self.dimensions.w) < other.origin.x) || // other is to the right
          ((other.origin.x + other.dimensions.w) <  self.origin.x) || // other is to the left
          (( self.origin.y +  self.dimensions.h) < other.origin.y) || // other is above
          ((other.origin.y + other.dimensions.h) <  self.origin.y))   // other is below
    }

    pub fn place (&mut self, x: f32, y: f32) {
        self.origin.x = x;
        self.origin.y = y;
    }

    pub fn change_position (&mut self, dx: f32, dy: f32) {
        self.origin.x += dx;
        self.origin.y += dy;
    }
}

#[cfg(test)]
mod tests {
    // import everything from the parent module, the parent module being activation
    use super::*;

    #[test]
    fn test_is_inside() {
        let bb = BoundingBox::new(0.0, 0.0, 10.0, 10.0);
        assert!(bb.is_inside(5.0, 5.0));
        assert!(bb.is_inside(0.1, 9.9));
        assert!(!bb.is_inside(-1.0, 5.0));
        assert!(!bb.is_inside(11.0, 5.0));
        assert!(!bb.is_inside(5.0, -1.0));
        assert!(!bb.is_inside(5.0, 11.0));
    }

    #[test]
    fn test_collision() {
        let bb = BoundingBox::new(0.0, 0.0, 10.0, 10.0);
        let mut other = BoundingBox::new(-5.0, -5.0, 2.0, 2.0);
        assert!(!bb.is_colliding_with(&other));
        other.origin.x = 1.0;
        other.origin.y = 1.0;
        assert!(bb.is_colliding_with(&other));
        other.origin.x = 11.0;
        other.origin.y = 11.0;
        assert!(!bb.is_colliding_with(&other));
        other.origin.x = 5.0;
        other.origin.y = 5.0;
        assert!(bb.is_colliding_with(&other));
    }

    #[test]
    fn test_place() {
        let mut bb = BoundingBox::new(0.0, 0.0, 10.0, 10.0);
        assert_eq!(bb.origin.x, 0.0);
        assert_eq!(bb.origin.y, 0.0);
        bb.place(1.0, 2.0);
        assert_eq!(bb.origin.x, 1.0);
        assert_eq!(bb.origin.y, 2.0);
    }

    #[test]
    fn test_change_position() {
        let mut bb = BoundingBox::new(1.0, 2.0, 10.0, 10.0);
        assert_eq!(bb.origin.x, 1.0);
        assert_eq!(bb.origin.y, 2.0);
        bb.change_position(1.0, 2.0);
        assert_eq!(bb.origin.x, 2.0);
        assert_eq!(bb.origin.y, 4.0);
    }
}