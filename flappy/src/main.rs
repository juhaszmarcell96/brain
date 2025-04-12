pub mod bird;
pub mod obstacle;

use bird::Bird;
use game::bounding_box::BoundingBox;
use obstacle::Obstacles;
use brain::math::random::RandomGenerator;
use brain::network::network_creator::NetworkCreator;
use brain::network::network_mutator::{NetworkMutatorConfig, NetworkMutator};
use std::io::{self, Write};

const SCENE_W : f32 = 100.0;
const SCENE_H : f32 =  40.0;

const NUM_OBSTACLES : usize = 4;
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
    let mut rand: RandomGenerator<f32> = RandomGenerator::<f32>::with_seed(SEED);
    // birds
    let mut birds: Vec<Bird> = Vec::<Bird>::new();
    // network initializer and mutator
    let mut initializer = NetworkCreator::<f32>::new(SEED, -1.0, 1.0, SEED+5, MIN_HIDDEN_LAYERS, MAX_HIDDEN_LAYERS, SEED+10, 1, 8);
    let mutator_config = NetworkMutatorConfig {
        random_generator_seed: SEED,
        weight_mutation_probability: 0.5,
        weight_change_upper_limit: 0.5,
        weight_change_lower_limit: -0.5
    };
    let mut mutator = NetworkMutator::new(mutator_config);
    // populate birds
    for i in 0..NUM_BIRDS {
        birds.push(Bird::new(BIRD_X, BIRD_Y, BIRD_W, BIRD_H, initializer.create(4, 1)));
    }

    let mut generation : u32 = 0;
    let mut best_score : u32 = 0;
    let mut best_brain = birds[0].brain.clone(); // store the "best brain" for later

    // infinite loop for the game
    loop {
        // adjust the factor based on the current best score relative to the goal
        mutator.set_factor(1.0 - ((best_score as f32) / (GOAL as f32)));
        generation += 1;
        // create obstacles
        let upper_height = rand.generate(UPPER_OBSTACKLE_H_MIN, UPPER_OBSTACKLE_H_MAX);
        let lower_y = upper_height + OBSTACLE_GAP_Y;
        let mut obstacles: [Obstacles; NUM_OBSTACLES] = [
            Obstacles::new(SCENE_W + 1.0 * OBSTACLE_GAP_X, lower_y, OBSTACLE_W, upper_height, SCENE_H - lower_y),
            Obstacles::new(SCENE_W + 2.0 * OBSTACLE_GAP_X, lower_y, OBSTACLE_W, upper_height, SCENE_H - lower_y),
            Obstacles::new(SCENE_W + 3.0 * OBSTACLE_GAP_X, lower_y, OBSTACLE_W, upper_height, SCENE_H - lower_y),
            Obstacles::new(SCENE_W + 4.0 * OBSTACLE_GAP_X, lower_y, OBSTACLE_W, upper_height, SCENE_H - lower_y)
        ];
        let mut current_obstacle: u32 = 0;
        // variables to follow
        let mut current_score : u32 = 0;
        let mut obstacles_passed : u32 = 0;
        // inner loop for the game runs
        loop {
            current_score += 1;
            print_game_stats(current_score, obstacles_passed, best_score, generation);
            for bird in birds.iter_mut() {
                if bird.is_alive() {
                    bird.fall(GRAVITY);
                    bird.apply_physics();
                }
            }
            for obstacle in obstacles.iter_mut() {
                obstacle.change_position(-OBSTACLE_VELOCITY, 0.0);
                // did it move outside the scene?
                if obstacle.x() < -OBSTACLE_W {
                    let upper_height = rand.generate(UPPER_OBSTACKLE_H_MIN, UPPER_OBSTACKLE_H_MAX);
                    let lower_y = upper_height + OBSTACLE_GAP_Y;
                    let new_x = obstacle.x() + ((NUM_OBSTACLES - 1) as f32) * OBSTACLE_GAP_X;
                    *obstacle = Obstacles::new(new_x, lower_y, OBSTACLE_W, upper_height, SCENE_H - lower_y);
                }
            }
        }
    }




}

fn print_game_stats(current_score: u32, obstacles_passed: u32, best_score: u32, generation: u32) {
    print!("\r"); // move the cursor to the beginning of the line
    print!(
        "score: {:>10}      obstacles: {:>10}      best score: {:>10}      generation: {:>10}",
        current_score, obstacles_passed, best_score, generation
    );
    io::stdout().flush().unwrap();
}
