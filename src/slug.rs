/// Turns text into a URL slug: ASCII letters lowercased, every run of other characters
/// replaced by one '-', and no '-' at either end.
pub fn slugify(s: &str) -> String {
    let mut out = String::new();
    let mut gap = false;
    for c in s.chars() {
        if c.is_ascii_alphanumeric() {
            if gap && !out.is_empty() {
                out.push('-');
            }
            out.push(c.to_ascii_lowercase());
            gap = false;
        } else {
            gap = true;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::slugify;

    #[test]
    fn lowercases_letters() {
        assert_eq!(slugify("Hello"), "hello");
    }

    #[test]
    fn collapses_runs_of_other_characters_into_one_dash() {
        assert_eq!(slugify("a  b--c!!d"), "a-b-c-d");
    }

    #[test]
    fn trims_dashes_at_both_ends() {
        assert_eq!(slugify("  --Proof Stream!-- "), "proof-stream");
    }
}
