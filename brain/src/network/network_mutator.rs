use crate::math::random::RandomGenerator;
use crate::network::network::Network;

pub struct NetworkMutatorConfig {
    // seed for the random number generator for reproducibility
    pub random_generator_seed: u64,
    // weight mutation parameters
    // if a randomly generated number between 0.0 and 1.0 is smaller than the mutation probability,
    // the weight mutates adding a random number to it from the interval [lower, upper]
    pub weight_mutation_probability: f32,
    pub weight_change_upper_limit: f32,
    pub weight_change_lower_limit: f32
}

pub struct NetworkMutator {
    value_change_generator: RandomGenerator<f32>,
    value_change_lower_limit: f32,
    value_change_upper_limit: f32,
    value_change_probability: f32,
    probability_generator: RandomGenerator<f32>,
    factor: f32,
}

impl NetworkMutator {
    pub fn new(config: NetworkMutatorConfig) -> Self {
        Self {
            value_change_generator: RandomGenerator::with_seed(config.random_generator_seed),
            value_change_lower_limit: config.weight_change_lower_limit,
            value_change_upper_limit: config.weight_change_upper_limit,
            value_change_probability: config.weight_mutation_probability,
            probability_generator: RandomGenerator::<f32>::with_seed(config.random_generator_seed),
            factor: 1.0
        }
    }

    pub fn set_factor (&mut self, factor: f32) {
        self.factor = factor;
    }

    pub fn get_factor (&self) -> f32 {
        self.factor
    }

    pub fn mutate(&mut self, network: &mut Network<f32>) {
        for layer in 0..network.get_num_weight_layers() {
            let dimensions = network.get_weights_dimension(layer);
            for row in 0..dimensions.rows {
                for col in 0..dimensions.cols {
                    let mutation_roll = self.probability_generator.generate(0.0, 1.0);
                    if mutation_roll < self.value_change_probability {
                        let value_change = self.value_change_generator.generate(self.value_change_lower_limit, self.value_change_upper_limit);
                        let mut weight = network.get_weight(layer, row, col);
                        weight += value_change * self.factor;
                        network.set_weight(layer, row, col, weight);
                    }
                }
            }
        }
        for layer in 0..network.get_num_bias_layers() {
            let dimensions = network.get_biases_dimension(layer);
            assert_eq!(dimensions.rows, 1, "something is off, biases layer should have dimensions 1xN");
            for col in 0..dimensions.cols {
                let mutation_roll = self.probability_generator.generate(0.0, 1.0);
                if mutation_roll < self.value_change_probability {
                    let value_change = self.value_change_generator.generate(self.value_change_lower_limit, self.value_change_upper_limit);
                    let mut bias = network.get_bias(layer, col);
                    bias += value_change * self.factor;
                    network.set_bias(layer, col, bias);
                }
            }
        }
    }
}