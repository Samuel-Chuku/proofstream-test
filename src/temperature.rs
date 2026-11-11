/// Degrees Celsius to degrees Fahrenheit.
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

/// Degrees Fahrenheit to degrees Celsius.
pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn freezing() {
        assert!(close(celsius_to_fahrenheit(0.0), 32.0));
        assert!(close(fahrenheit_to_celsius(32.0), 0.0));
    }

    #[test]
    fn boiling() {
        assert!(close(celsius_to_fahrenheit(100.0), 212.0));
        assert!(close(fahrenheit_to_celsius(212.0), 100.0));
    }

    #[test]
    fn minus_forty_is_the_same_on_both_scales() {
        assert!(close(celsius_to_fahrenheit(-40.0), -40.0));
        assert!(close(fahrenheit_to_celsius(-40.0), -40.0));
    }
}
