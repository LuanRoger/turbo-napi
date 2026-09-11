/// Adds two numbers.
pub fn add(left: f64, right: f64) -> f64 {
    left + right
}

/// Subtracts the right operand from the left operand.
pub fn subtract(left: f64, right: f64) -> f64 {
    left - right
}

/// Multiplies two numbers.
pub fn multiply(left: f64, right: f64) -> f64 {
    left * right
}

/// Divides the left operand by the right operand.
pub fn divide(left: f64, right: f64) -> f64 {
    left / right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn performs_basic_arithmetic() {
        assert_eq!(add(8.0, 2.0), 10.0);
        assert_eq!(subtract(8.0, 2.0), 6.0);
        assert_eq!(multiply(8.0, 2.0), 16.0);
        assert_eq!(divide(8.0, 2.0), 4.0);
    }
}
