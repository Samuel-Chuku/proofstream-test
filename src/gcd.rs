/// Greatest common divisor by Euclid's algorithm. gcd(0, 0) is 0.
pub fn gcd(a: u64, b: u64) -> u64 {
    let (mut a, mut b) = (a, b);
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_factor() {
        assert_eq!(gcd(48, 18), 6);
    }

    #[test]
    fn coprime() {
        assert_eq!(gcd(17, 5), 1);
    }

    #[test]
    fn zero_is_identity() {
        assert_eq!(gcd(0, 9), 9);
        assert_eq!(gcd(9, 0), 9);
    }

    #[test]
    fn both_zero() {
        assert_eq!(gcd(0, 0), 0);
    }
}
