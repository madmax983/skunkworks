#[cfg(test)]
mod tests {
    use crate::safe_gl::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        #[should_panic]
        fn test_with_scissor_havoc(
            x in any::<i32>(), y in any::<i32>(), w in any::<i32>(), h in any::<i32>(),
            px in any::<i32>(), py in any::<i32>(), pw in any::<i32>(), ph in any::<i32>()
        ) {
            with_scissor(x, y, w, h, Some((px, py, pw, ph)), |_| {});
        }
    }
}
