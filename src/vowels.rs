/// Whether `c` is one of a, e, i, o, u, in either case.
fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}

/// How many vowels (a, e, i, o, u, in either case) appear in `s`.
pub fn count_vowels(s: &str) -> usize {
    s.chars().filter(|&c| is_vowel(c)).count()
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
        assert_eq!(count_vowels("AEIOU aeiou"), 10);
    }

    #[test]
    fn a_string_without_vowels_has_none() {
        assert_eq!(count_vowels("rhythm"), 0);
    }
}
