#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use crate::compiler::compile;

    proptest! {
        #[test]
        fn test_compile_random_string(ref s in "\\PC*") {
            let _ = compile(s, None);
        }
    }
}
