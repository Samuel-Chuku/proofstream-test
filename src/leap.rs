/// Whether `year` is a leap year in the Gregorian calendar: divisible by 4, except
/// century years, which are leap years only when divisible by 400.
pub fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divisible_by_four() {
        assert!(is_leap_year(2024));
    }

    #[test]
    fn century_is_not() {
        assert!(!is_leap_year(1900));
    }

    #[test]
    fn every_400_years_is() {
        assert!(is_leap_year(2000));
    }

    #[test]
    fn ordinary_year_is_not() {
        assert!(!is_leap_year(2023));
    }
}
