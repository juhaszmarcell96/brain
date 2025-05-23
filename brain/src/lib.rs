pub mod math; // include the math/mod.rs
pub mod network;

pub use math::matrix::Matrix;
pub use math::random::{RandomGenerator, RandomGeneratorConfig};
pub use network::network::Network;