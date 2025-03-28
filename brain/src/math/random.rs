use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::marker::PhantomData;

pub struct RandomGenerator<T> {
    rng: StdRng,
    _marker: PhantomData<T>,
}

impl<T> RandomGenerator<T>
where
    T: rand::distr::uniform::SampleUniform + PartialOrd + Copy,
{
    pub fn new() -> Self {
        let seed = rand::random::<u64>();
        Self::with_seed(seed)
    }

    pub fn with_seed(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
            _marker: PhantomData,
        }
    }

    pub fn generate(&mut self, a: T, b: T) -> T {
        if a >= b {
            panic!("Invalid range: a must be less than b");
        }
        self.rng.random_range(a..b)
    }
}

// Configuration struct
pub struct RandomGeneratorConfig<T>
where
    T: rand::distr::uniform::SampleUniform + PartialOrd + Copy,
{
    pub seed: u64,
    pub lower_limit: T,
    pub upper_limit: T,
}

impl<T> RandomGenerator<T>
where
    T: rand::distr::uniform::SampleUniform + PartialOrd + Copy,
{
    pub fn from_config(config: RandomGeneratorConfig<T>) -> Self {
        Self {
            rng: StdRng::seed_from_u64(config.seed),
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_float() {
        let mut rng = RandomGenerator::<f64>::with_seed(42);
        let value = rng.generate(0.0, 1.0);
        assert!(value >= 0.0 && value < 1.0);
    }

    #[test]
    fn test_generate_int() {
        let mut rng = RandomGenerator::<i32>::with_seed(42);
        let value = rng.generate(10, 20);
        assert!(value >= 10 && value < 20);
    }

    #[test]
    #[should_panic(expected = "Invalid range")]
    fn test_invalid_range() {
        let mut rng = RandomGenerator::<i32>::new();
        rng.generate(10, 5); // Should panic
    }

    #[test]
    fn test_deterministic_output() {
        let mut rng1 = RandomGenerator::<i32>::with_seed(12345);
        let mut rng2 = RandomGenerator::<i32>::with_seed(12345);

        assert_eq!(rng1.generate(1, 100), rng2.generate(1, 100));
        assert_eq!(rng1.generate(1, 100), rng2.generate(1, 100));
    }

    #[test]
    fn test_generator_with_config() {
        let config = RandomGeneratorConfig {
            seed: 98765,
            lower_limit: 5.0,
            upper_limit: 10.0,
        };
        let mut rng = RandomGenerator::from_config(config);
        let value = rng.generate(5.0, 10.0);
        assert!(value >= 5.0 && value < 10.0);
    }
}
