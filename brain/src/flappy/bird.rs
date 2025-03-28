use brain::game::position::BoundingBox;
use network::network::Network;

#[derive(Debug, Clone)]
pub struct Bird {
    pub bounding_box: BoundingBox,
    pub brain: Network<f32>,
    velocity: f32,
    score: u32,
    alive: bool
}

impl Bird {
    pub fn new (x: f32, y: f32, w: f32, h: f32, network: Network<f32>) -> Self {
        Self {
            bounding_box: BoundingBox::new(x, y, w, h),
            brain: network,
            velocity: 0.0,
            score: 0,
            alive: true
        }
    }

    pub fn reset (&mut self, x: f32, y: f32) {
        self.score = 0;
        self.velocity = 0.0;
        self.alive = true;
        self.bounding_box.place(x, y);
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