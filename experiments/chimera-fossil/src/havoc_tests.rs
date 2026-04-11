#[cfg(test)]
mod havoc_tests {
    use crate::entropy::Fossil;

    #[test]
    #[should_panic(expected = "byte index 60 is not a char boundary")]
    fn test_havoc_fossil_display_unicode_panic() {
        // 👺 Havoc: `Fossil::fmt` checks `self.original_text.len() > 60` and then slices `&self.original_text[..60]`.
        // It assumes that byte index 60 is a valid UTF-8 character boundary.
        // If the 60th byte falls in the middle of a multi-byte Unicode character, this will panic!
        //
        // Let's build a string where byte 60 is precisely inside a multi-byte character.
        // "A" is 1 byte. 59 "A"s = 59 bytes.
        // "🚀" (Rocket Emoji) is 4 bytes.
        // So the string is 63 bytes long.
        // Byte 60 will slice right into the middle of the rocket emoji.
        let original = "A".repeat(59) + "🚀";
        let displayed = original.clone();
        let mask = vec![true; 60];

        let fossil = Fossil {
            original_text: original,
            displayed_text: displayed,
            mask,
        };

        // This format! call uses the Display trait, which will slice `[..60]`.
        // It cuts off the rocket mid-flight, causing a panic!
        let _output = format!("{}", fossil);
    }
}
