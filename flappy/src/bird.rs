use game::bounding_box::BoundingBox;
use brain::network::network::Network;
use brain::math::matrix::Matrix;

#[derive(Clone)]
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

    pub fn wanna_jump (&mut self, x_distance: f32, y_distance_1: f32, y_distance_2: f32, valocity: f32) -> bool {
        let mut input = Matrix::<f32>::new(1, 4);
        input[0][0] = x_distance;
        input[0][1] = y_distance_1;
        input[0][2] = y_distance_2;
        input[0][3] = valocity;
        let output = self.brain.forward(&input);
        output[0][0] > 0.55
    }

    pub fn reset (&mut self, x: f32, y: f32) {
        self.score = 0;
        self.velocity = 0.0;
        self.alive = true;
        self.bounding_box.place(x, y);
    }

    pub fn kill (&mut self) {
        self.alive = false;
    }

    pub fn is_alive (&self) -> bool {
        self.alive
    }

    pub fn fall (&mut self, g : f32) {
        self.velocity += g;
    }

    pub fn apply_physics (&mut self) {
        self.bounding_box.change_position(0.0, self.velocity);
    }

    pub fn jump (&mut self, boost: f32) {
        self.velocity -= boost;
    }
}