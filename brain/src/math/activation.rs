pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

pub fn relu(x: f64) -> f64 {
    x.max(0.0)
}

#[cfg(test)]
mod tests {
    // import everything from the parent module, the parent module being activation
    use super::*;

    #[test]
    fn test_sigmoid() {
        let result = sigmoid(0.0);
        assert!((result - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_relu() {
        assert_eq!(relu(3.0), 3.0);
        assert_eq!(relu(-2.0), 0.0);
    }
}