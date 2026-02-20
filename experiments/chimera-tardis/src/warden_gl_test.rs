
#[cfg(test)]
mod tests {
    use super::super::safe_gl::*;

    #[test]
    fn test_negative_scissor_dimensions() {
        let r1 = (0, 0, -100, -100);
        let r2 = (0, 0, 100, 100);
        let intersection = intersect_rect(r1, r2);

        // r1: x=0, w=-100.
        // x2 = x + w = 0 + -100 = -100.
        // r2: x=0, w=100. x2=100.
        // x1 = max(0, 0) = 0.
        // ax2 = -100. bx2 = 100.
        // x2 = min(-100, 100) = -100.
        // width = x2 - x1 = -100 - 0 = -100.
        // max(0) -> 0.

        assert_eq!(intersection.2, 0); // Width should be 0
        assert_eq!(intersection.3, 0); // Height should be 0
    }
}
