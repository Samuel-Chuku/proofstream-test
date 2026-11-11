/// The arithmetic mean, or None for no values.
pub fn mean(xs: &[f64]) -> Option<f64> {
    if xs.is_empty() {
        return None;
    }
    Some(xs.iter().sum::<f64>() / xs.len() as f64)
}

/// The median, or None for no values. An even number of values gives the average of the
/// two in the middle.
pub fn median(xs: &[f64]) -> Option<f64> {
    if xs.is_empty() {
        return None;
    }
    let mut sorted = xs.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        Some((sorted[mid - 1] + sorted[mid]) / 2.0)
    } else {
        Some(sorted[mid])
    }
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

    #[test]
    fn median_of_nothing_is_none() {
        assert_eq!(median(&[]), None);
    }

    #[test]
    fn median_of_odd_length_is_the_middle_value() {
        assert_eq!(median(&[3.0, 1.0, 2.0]), Some(2.0));
    }

    #[test]
    fn median_of_even_length_averages_the_two_middle_values() {
        assert_eq!(median(&[4.0, 1.0, 3.0, 2.0]), Some(2.5));
    }
}
