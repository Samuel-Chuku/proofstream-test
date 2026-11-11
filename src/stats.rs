/// The arithmetic mean, or None for no values.
pub fn mean(xs: &[f64]) -> Option<f64> {
    if xs.is_empty() {
        return None;
    }
    Some(xs.iter().sum::<f64>() / xs.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mean_of_nothing_is_none() {
        assert_eq!(mean(&[]), None);
    }

    #[test]
    fn mean_of_values() {
        assert_eq!(mean(&[1.0, 2.0, 3.0, 4.0]), Some(2.5));
        assert_eq!(mean(&[7.0]), Some(7.0));
    }
}
