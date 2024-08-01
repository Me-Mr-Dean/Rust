pub fn sigmoid(x: f64) -> f64 {
  1.0 / (1.0 + (-x).exp())
}

pub fn sigmoid_derivative(x: f64) -> f64 {
  let sig = sigmoid(x);
  sig * (1.0 - sig)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sigmoid() {
        assert!((sigmoid(0.0) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_sigmoid_derivative() {
        assert!((sigmoid_derivative(0.0) - 0.25).abs() < 1e-6);
    }
}
