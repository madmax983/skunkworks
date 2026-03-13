#[cfg(test)]
#[cfg(feature = "loom")]
mod tests {
    use super::*;
    use loom::sync::Arc;
    use loom::thread;
    use parking_lot::Mutex;

    // loom does not support parking_lot Mutex
}
