#[cfg(test)]
mod tests {
    use crate::sexagesimal::Sexagesimal;
    use std::str::FromStr;

    #[test]
    fn test_parse_one() {
        let one = Sexagesimal::from_str("𒐕").expect("Failed to parse 1");
        assert_eq!(one.digits, vec![1]);
    }

    #[test]
    fn test_parse_ten() {
        let ten = Sexagesimal::from_str("𒌋").expect("Failed to parse 10");
        assert_eq!(ten.digits, vec![10]);
    }

    #[test]
    fn test_parse_eleven() {
        // 11 = 10 + 1 = 𒌋𒐕
        let eleven = Sexagesimal::from_str("𒌋𒐕").expect("Failed to parse 11");
        assert_eq!(eleven.digits, vec![11]);
    }

    #[test]
    fn test_parse_sixty() {
        // 60 = 1, 0 (or space placeholder)
        // Let's use space as separator for places.
        // "𒐕, 𒐕" = 60 + 1? No. "𒐕 𒐕"
        // Let's assume space separates places.
        // "𒐕 𒌋" = 1*60 + 10 = 70.
        let seventy = Sexagesimal::from_str("𒐕 𒌋").expect("Failed to parse 70");
        assert_eq!(seventy.digits, vec![1, 10]);
    }

    #[test]
    fn test_addition() {
        let a = Sexagesimal::from_str("𒐕").unwrap();
        let b = Sexagesimal::from_str("𒐕").unwrap();
        let c = a + b;
        // 1 + 1 = 2 (𒐕𒐕)
        assert_eq!(c.digits, vec![2]);
    }

    #[test]
    fn test_carry_addition() {
        // 59 + 1 = 1,0
        // 59 = 5 tens (<<<<<) + 9 ones (YYYYYYYYY)
        // Let's assume we implement parsing for numbers > 9 correctly.
        // For now, let's construct manually to test logic if parsing fails.
        let fifty_nine = Sexagesimal::new(vec![59]);
        let one = Sexagesimal::new(vec![1]);
        let sixty = fifty_nine + one;
        assert_eq!(sixty.digits, vec![1, 0]);
    }
}
