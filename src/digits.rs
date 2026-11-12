/// Sum of the decimal digits of `n`. digit_sum(0) is 0.
pub fn digit_sum(n: u64) -> u64 {
    let mut n = n;
    let mut sum = 0;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        assert_eq!(digit_sum(0), 0);
    }

    #[test]
    fn single_digit() {
        assert_eq!(digit_sum(7), 7);
    }

    #[test]
    fn several_digits() {
        assert_eq!(digit_sum(12345), 15);
    }

    #[test]
    fn largest_u64() {
        assert_eq!(digit_sum(u64::MAX), 87);
    }
}
