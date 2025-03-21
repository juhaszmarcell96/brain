use brain::sigmoid;
use brain::relu;

fn main() {
    println!("Sigmoid(0.5): {}", sigmoid(0.5));
    println!("ReLU(-1.0): {}", relu(-1.0));
}