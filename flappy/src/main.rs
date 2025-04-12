pub mod bird;
use bird::Bird;
use brain::math::random::RandomGenerator;
use brain::network::network_creator::NetworkCreator;
use brain::network::network_mutator::{NetworkMutatorConfig, NetworkMutator}

const SCENE_W : f32 = 100.0;
const SCENE_H : f32 =  40.0;

const NUM_OBSTACLES : u32 = 4;
const OBSTACLE_GAP_X : f32 = 40.0;
const OBSTACLE_GAP_Y : f32 = 15.0;
const UPPER_OBSTACKLE_H_MIN : f32 = 5.0;
const UPPER_OBSTACKLE_H_MAX : f32 = 20.0;
const OBSTACLE_W : f32 = 4.0;

const NUM_BIRDS : u32 = 1000;
const BIRD_X : f32 = 10.0;
const BIRD_Y : f32 = 20.0;
const BIRD_W : f32 =  2.0;
const BIRD_H : f32 =  2.0;

const NUM_FITTEST : i16 = 10;
const MIN_HIDDEN_LAYERS : usize = 1;
const MAX_HIDDEN_LAYERS : usize = 4;

const GRAVITY : f32 = 0.2;
const JUMP_BOOST : f32 = 1.5;
const OBSTACLE_VELOCITY : f32 = 1.0;

const JUMP_FREQUENCY : u32 = 5;
const GOAL : u32 = 1000000;
const SEED : u64 = 50;

fn main() {
    // random number generator for in-game random numbers, such as obstacle positions
    let rand: RandomGenerator<f32> = RandomGenerator::<f32>::with_seed(SEED);
    // birds
    let mut birds: Vec<Bird> = Vec::<Bird>::new();
    // network initializer and mutator
    let initializer = NetworkCreator::<f32>::new(SEED, -1.0, 1.0, SEED+5, MIN_HIDDEN_LAYERS, MAX_HIDDEN_LAYERS, SEED+10, 1, 8);
    let mutator_config = NetworkMutatorConfig {
        random_generator_seed: SEED,
        weight_mutation_probability: 0.5,
        weight_change_upper_limit: 0.5,
        weight_change_lower_limit: -0.5
    };
}
