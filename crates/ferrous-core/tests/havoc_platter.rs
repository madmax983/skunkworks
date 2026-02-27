#[cfg(test)]
mod tests {
    use ferrous_core::Platter;

    #[test]
    #[should_panic(expected = "Platter size overflow")]
    fn havoc_platter_overflow() {
        // [HAVOC] Allocating a Platter with wrapping dimensions.
        // width = 2^63 + 1, height = 2 => width*height = 2^64 + 2 => 2 (wrapped usize::MAX+1)

        let width = (usize::MAX / 2) + 2;
        let height = 2;

        // This should now panic at creation time due to checked_mul
        let _p = Platter::new(width, height);
    }
}
