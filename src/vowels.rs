/// How many of a, e, i, o, u appear in `s`, in either case.
pub fn count_vowels(s: &str) -> usize {
    s.chars().filter(|c| matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')).count()
}

#[cfg(test)]
mod tests {
    use super::count_vowels;

    #[test]
    fn empty_string_has_none() {
        assert_eq!(count_vowels(""), 0);
    }

    #[test]
    fn counts_both_cases() {
        assert_eq!(count_vowels("ProofStream ON NEAR"), 7);
    }

    #[test]
    fn a_string_without_vowels_has_none() {
        assert_eq!(count_vowels("rhythm"), 0);
    }
}
