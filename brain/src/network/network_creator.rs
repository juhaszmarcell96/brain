use std::ops::{Add, Sub, Mul, Div, Index, IndexMut, MulAssign, DivAssign}; // +, -, *, /, [], *=, /=
use std::fmt;

use crate::network::network_initializer::NetworkInitializer;
use crate::math::random::RandomGenerator;
use crate::network::network::Network;

pub struct NetworkCreator<T> {
    initializer : NetworkInitializer<T>,
    layer_number_generator : RandomGenerator<usize>,
    min_hidden_layers : usize,
    max_hidden_layers : usize,
    neuron_number_generator : RandomGenerator<usize>,
    min_neurons : usize,
    max_neurons : usize
}

impl<T> NetworkCreator<T>
where
    T: rand::distr::uniform::SampleUniform + std::cmp::PartialOrd + Copy + Default,
    T: Add + fmt::Debug + Div + num::Float + Mul + num::Signed + Sub + num::Zero
{
    pub fn new(v_seed : u64, lower_limit: T, upper_limit: T,
               l_seed : u64, min_hidden_layers : usize, max_hidden_layers : usize,
               n_seed : u64, min_neurons : usize, max_neurons : usize) -> Self
    {
        if lower_limit >= upper_limit {
            panic!("Invalid range: lower limit must be less than upper limit");
        }
        Self {
            initializer: NetworkInitializer::<T>::new(v_seed, lower_limit, upper_limit),
            layer_number_generator: RandomGenerator::<usize>::with_seed(l_seed),
            min_hidden_layers, max_hidden_layers,
            neuron_number_generator: RandomGenerator::<usize>::with_seed(n_seed),
            min_neurons, max_neurons
        }
    }

    pub fn create (&mut self, input_size : usize, output_size : usize) -> Network<T>
    {
        let num_hidden_layers = self.layer_number_generator.generate(self.min_hidden_layers, self.max_hidden_layers);
        let mut network = Network::<T>::new(num_hidden_layers + 2); // +2 for input and output
        network.resize(0, input_size); // input layer
        // hidden layers
        for layer in 1..num_hidden_layers {
            network.resize(layer, self.neuron_number_generator.generate(self.min_neurons, self.max_neurons));
        }
        network.resize(num_hidden_layers + 1, output_size); // output layer
        self.initializer.initialize(&mut network);
        network
    }

    pub fn initialize (&mut self, network : &mut Network<T>) {
        self.initializer.initialize(network);
    }
}