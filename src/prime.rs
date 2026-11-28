/// True when `n` is a prime number. 0 and 1 are not prime.
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d <= n / d {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_and_one() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
    }

    #[test]
    fn small_primes() {
        assert!(is_prime(2));
        assert!(is_prime(13));
    }

    #[test]
    fn composites() {
        assert!(!is_prime(9));
        assert!(!is_prime(25));
    }

    #[test]
    fn large_prime() {
        assert!(is_prime(1_000_000_007));
    }
}
