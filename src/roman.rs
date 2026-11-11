/// Roman numerals for 1..=3999; None outside that range.
pub fn to_roman(n: u32) -> Option<String> {
    if !(1..=3999).contains(&n) {
        return None;
    }
    const TABLE: [(u32, &str); 13] = [
        (1000, "M"), (900, "CM"), (500, "D"), (400, "CD"), (100, "C"), (90, "XC"),
        (50, "L"), (40, "XL"), (10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I"),
    ];
    let mut rest = n;
    let mut out = String::new();
    for (value, symbol) in TABLE {
        while rest >= value {
            out.push_str(symbol);
            rest -= value;
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::to_roman;

    #[test]
    fn converts_the_listed_values() {
        for (n, want) in [(1, "I"), (4, "IV"), (9, "IX"), (14, "XIV"), (40, "XL"), (90, "XC"), (400, "CD"), (1994, "MCMXCIV"), (3999, "MMMCMXCIX")] {
            assert_eq!(to_roman(n).as_deref(), Some(want), "{n}");
        }
    }

    #[test]
    fn refuses_values_outside_the_range() {
        assert_eq!(to_roman(0), None);
        assert_eq!(to_roman(4000), None);
    }
}
