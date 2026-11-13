/// "Fizz" for multiples of 3, "Buzz" for multiples of 5, "FizzBuzz" for multiples of both,
/// and the number itself otherwise.
pub fn fizzbuzz(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        _ => n.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_of_three() {
        assert_eq!(fizzbuzz(3), "Fizz");
    }

    #[test]
    fn multiple_of_five() {
        assert_eq!(fizzbuzz(5), "Buzz");
    }

    #[test]
    fn multiple_of_both() {
        assert_eq!(fizzbuzz(15), "FizzBuzz");
    }

    #[test]
    fn neither() {
        assert_eq!(fizzbuzz(7), "7");
    }
}
