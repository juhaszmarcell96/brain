use crate::math::random::RandomGenerator;
use crate::network::network::Network;

pub struct NetworkInitializer<T> {
    value_generator: RandomGenerator<T>,
    lower_limit: T,
    upper_limit: T
}

impl<T> NetworkInitializer<T>
where
    T: rand::distr::uniform::SampleUniform + std::cmp::PartialOrd + Copy + Default
{
    pub fn new(seed: u64, lower_limit: T, upper_limit: T) -> Self {
        if lower_limit >= upper_limit {
            panic!("Invalid range: lower limit must be less than upper limit");
        }
        Self {
            value_generator: RandomGenerator::with_seed(seed),
            lower_limit,
            upper_limit
        }
    }

    pub fn initialize(&mut self, network: &mut Network<T>) {
        for layer in 0..network.get_num_weight_layers() {
            let dimensions = network.get_weights_dimension(layer);
            for row in 0..dimensions.rows {
                for col in 0..dimensions.cols {
                    let weight = self.value_generator.generate(self.lower_limit, self.upper_limit);
                    network.set_weight(layer, row, col, weight);
                }
            }
        }
        for layer in 0..network.get_num_bias_layers() {
            let dimensions = network.get_biases_dimension(layer);
            assert_eq!(dimensions.rows, 1, "something is off, biases layer should have dimensions 1xN");
            for col in 0..dimensions.cols {
                let bias = self.value_generator.generate(self.lower_limit, self.upper_limit);
                network.set_bias(layer, col, bias);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_test() {
        let mut network = Network::<f32>::new(3).expect("Failed to create network");
        network.resize(0, 3);
        network.resize(1, 4);
        network.resize(2, 2);
        let mut initializer = NetworkInitializer::<f32>::new(50, 3.0, 4.0);
        initializer.initialize(&mut network);

        assert_eq!(network.get_num_layers(), 3);
        assert_eq!(network.get_num_weight_layers(), 2);
        assert_eq!(network.get_num_bias_layers(), 2);
        let dims = network.get_weights_dimension(0);
        assert_eq!(dims.rows, 3);
        assert_eq!(dims.cols, 4);
        let dims = network.get_weights_dimension(1);
        assert_eq!(dims.rows, 4);
        assert_eq!(dims.cols, 2);
        let dims = network.get_biases_dimension(0);
        assert_eq!(dims.rows, 1);
        assert_eq!(dims.cols, 4);
        let dims = network.get_biases_dimension(1);
        assert_eq!(dims.rows, 1);
        assert_eq!(dims.cols, 2);

        for layer in 0..network.get_num_weight_layers() {
            let dimensions = network.get_weights_dimension(layer);
            for row in 0..dimensions.rows {
                for col in 0..dimensions.cols {
                    let w = network.get_weight(layer, row, col);
                    assert!(w >= 3.0 && w < 4.0);
                }
            }
        }
        for layer in 0..network.get_num_bias_layers() {
            let dimensions = network.get_biases_dimension(layer);
            assert_eq!(dimensions.rows, 1, "something is off, biases layer should have dimensions 1xN");
            for col in 0..dimensions.cols {
                let b = network.get_bias(layer, col);
                assert!(b >= 3.0 && b < 4.0);
            }
        }
    }
}
