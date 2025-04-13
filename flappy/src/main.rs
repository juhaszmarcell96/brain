pub mod bird;
pub mod obstacle;

use bird::Bird;
use obstacle::Obstacles;
use brain::math::random::RandomGenerator;
use brain::network::network_creator::NetworkCreator;
use brain::network::network_mutator::{NetworkMutatorConfig, NetworkMutator};
use std::io::{self, Write};
use std::fs::File;

const SCENE_W : f32 = 100.0;
const SCENE_H : f32 =  40.0;

const NUM_OBSTACLES : usize = 4;
const OBSTACLE_GAP_X : f32 = 40.0;
const OBSTACLE_GAP_Y : f32 = 15.0;
const UPPER_OBSTACKLE_H_MIN : f32 = 5.0;
const UPPER_OBSTACKLE_H_MAX : f32 = 20.0;
const OBSTACLE_W : f32 = 4.0;

const NUM_BIRDS : usize = 1000;
const BIRD_X : f32 = 10.0;
const BIRD_Y : f32 = 20.0;
const BIRD_W : f32 =  2.0;
const BIRD_H : f32 =  2.0;

const NUM_FITTEST : usize = 10;
const MIN_HIDDEN_LAYERS : usize = 1;
const MAX_HIDDEN_LAYERS : usize = 4;

const GRAVITY : f32 = 0.2;
const JUMP_BOOST : f32 = 1.5;
const OBSTACLE_VELOCITY : f32 = 1.0;

const JUMP_FREQUENCY : u32 = 5;
const GOAL : u32 = 1000000;
const SEED : u64 = 50;

fn main() -> io::Result<()> {
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
    for _i in 0..NUM_BIRDS {
        birds.push(Bird::new(BIRD_X, BIRD_Y, BIRD_W, BIRD_H, initializer.create(4, 1)));
    }

    let mut generation : u32 = 0;
    let mut best_score : u32 = 0;
    let mut best_brain = birds[0].brain.clone(); // store the "best brain" for later

    // write to file:
    let mut gameplay = File::create("gameplay.yaml")?;
    writeln!(gameplay, "scene:")?;
    writeln!(gameplay, "  x: 0")?;
    writeln!(gameplay, "  y: 0")?;
    writeln!(gameplay, "  w: {}", SCENE_W)?;
    writeln!(gameplay, "  h: {}", SCENE_H)?;
    writeln!(gameplay, "obstacle:")?;
    writeln!(gameplay, "  y: 0")?;
    writeln!(gameplay, "  w: {}", OBSTACLE_W)?;
    writeln!(gameplay, "  gap: {}", OBSTACLE_GAP_Y)?;
    writeln!(gameplay, "bird:")?;
    writeln!(gameplay, "  x: {}", BIRD_X)?;
    writeln!(gameplay, "  w: {}", BIRD_W)?;
    writeln!(gameplay, "  h: {}", BIRD_H)?;
    writeln!(gameplay, "gameplay:")?;

    // infinite loop for the game
    loop {
        // adjust the factor based on the current best score relative to the goal
        mutator.set_factor(1.0 - ((best_score as f32) / (GOAL as f32)));
        generation += 1;
        // create obstacles
        let mut obstacles: [Obstacles; NUM_OBSTACLES] = [
            Obstacles::new(SCENE_H, SCENE_W + 1.0 * OBSTACLE_GAP_X, OBSTACLE_W, rand.generate(UPPER_OBSTACKLE_H_MIN, UPPER_OBSTACKLE_H_MAX), OBSTACLE_GAP_Y),
            Obstacles::new(SCENE_H, SCENE_W + 2.0 * OBSTACLE_GAP_X, OBSTACLE_W, rand.generate(UPPER_OBSTACKLE_H_MIN, UPPER_OBSTACKLE_H_MAX), OBSTACLE_GAP_Y),
            Obstacles::new(SCENE_H, SCENE_W + 3.0 * OBSTACLE_GAP_X, OBSTACLE_W, rand.generate(UPPER_OBSTACKLE_H_MIN, UPPER_OBSTACKLE_H_MAX), OBSTACLE_GAP_Y),
            Obstacles::new(SCENE_H, SCENE_W + 4.0 * OBSTACLE_GAP_X, OBSTACLE_W, rand.generate(UPPER_OBSTACKLE_H_MIN, UPPER_OBSTACKLE_H_MAX), OBSTACLE_GAP_Y)
        ];
        let mut current_obstacle_index: usize = 0;
        // variables to follow
        let mut current_score : u32 = 0;
        let mut obstacles_passed : u32 = 0;
        // inner loop for the game runs
        loop {
            current_score += 1;
            print_game_stats(current_score, obstacles_passed, best_score, generation);
            writeln!(gameplay, "  - current_score: {}", current_score)?;
            writeln!(gameplay, "    best_score: {}", best_score)?;
            writeln!(gameplay, "    birds:")?;
            for bird in birds.iter_mut() {
                if bird.is_alive() {
                    bird.fall(GRAVITY);
                    bird.apply_physics();
                    writeln!(gameplay, "      - y: {}", bird.bounding_box.origin.y)?;
                }
            }
            writeln!(gameplay, "    obstacles:")?;
            for obstacle in obstacles.iter_mut() {
                obstacle.change_position(-OBSTACLE_VELOCITY, 0.0);
                // did it move outside the scene?
                if obstacle.x() < -OBSTACLE_W {
                    let new_x = obstacle.x() + ((NUM_OBSTACLES) as f32) * OBSTACLE_GAP_X;
                    *obstacle = Obstacles::new(SCENE_H, new_x, OBSTACLE_W, rand.generate(UPPER_OBSTACKLE_H_MIN, UPPER_OBSTACKLE_H_MAX), OBSTACLE_GAP_Y);
                }
                writeln!(gameplay, "      - x: {}", obstacle.x())?;
                writeln!(gameplay, "        h: {}", obstacle.upper.dimensions.h)?;
                writeln!(gameplay, "        active: false")?;
            }
            // collision detection and jump prediction
            let mut someone_is_alive = false;
            for bird in birds.iter_mut() {
                if !bird.is_alive() { continue; }
                let current_obstacles : &mut Obstacles = &mut obstacles[current_obstacle_index];
                // does bird collide with the obstacles?
                if bird.collides_with(&current_obstacles.upper) || bird.collides_with(&current_obstacles.lower) {
                    bird.kill();
                    continue;
                }
                // does bird collide with floor or ceiling?
                if bird.is_outside(0.0, SCENE_H) {
                    bird.kill();
                    continue;
                }
                bird.increase_score();
                someone_is_alive = true;
                let x_dist = (current_obstacles.x() - bird.bounding_box.origin.x) / SCENE_W;
                let y_dist_1 = (current_obstacles.upper.origin.y - bird.bounding_box.origin.y) / SCENE_H;
                let y_dist_2 = (current_obstacles.lower.origin.y - bird.bounding_box.origin.y) / SCENE_H;
                let velocity = bird.get_velocity() / 10.0;
                if (current_score % JUMP_FREQUENCY) == 0 {
                    if bird.wanna_jump(x_dist, y_dist_1, y_dist_2, velocity) {
                        bird.jump(JUMP_BOOST);
                    }
                }
            }
            // update the current obstacle
            if obstacles[current_obstacle_index].x() < (BIRD_X - OBSTACLE_W) {
                current_obstacle_index += 1;
                obstacles_passed += 1;
                if current_obstacle_index == NUM_OBSTACLES {
                    current_obstacle_index = 0;
                }
            }

            // loop breaking conditions -> goal reached or everyone dead
            if !someone_is_alive { break; }
            if current_score > GOAL { break; }
        }

        // round is over, sort birds based on score and select the fittest ones for darwinian evolution
        birds.sort_by(|a, b| b.get_score().cmp(&a.get_score()));
        if birds[0].get_score() > best_score {
            best_score = birds[0].get_score();
            best_brain = birds[0].brain.clone();
        }
        for i in 0..NUM_BIRDS {
            birds[i].reset(BIRD_X, BIRD_Y); // reset the game parameters
            // the fittest ones get to try again -> don't mutate them
            if i > NUM_FITTEST {
                // mutate the first third, re-initialize the second third, create new for the last third
                // first part -> mutation of the all-time best
                // second part -> mutation of the top N
                // third part -> newly initialized networks
                if i < (NUM_BIRDS / 5) {
                    birds[i].brain = best_brain.clone();
                    mutator.mutate(&mut birds[i].brain);
                }
                else if i < (NUM_BIRDS / 2) {
                    birds[i].brain = birds[i % NUM_FITTEST].brain.clone();
                    mutator.mutate(&mut birds[i].brain);
                }
                else {
                    initializer.initialize(&mut birds[i].brain);
                }
            }
        }
        if best_score > GOAL { break; }
    }
    print!("well done...");
    Ok(())
}

fn print_game_stats(current_score: u32, obstacles_passed: u32, best_score: u32, generation: u32) {
    print!("\r"); // move the cursor to the beginning of the line
    print!(
        "score: {:>10}      obstacles: {:>10}      best score: {:>10}      generation: {:>10}",
        current_score, obstacles_passed, best_score, generation
    );
    io::stdout().flush().unwrap();
}
