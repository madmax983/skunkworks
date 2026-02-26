#[cfg(test)]
mod tests {
    use chimera_lang::ast::Dna;
    use chimera_lang::vm::nova_babel_live::exec_live_parse;
    use chimera_lang::vm::{ChimeraVM, Value, GRID_SIZE};
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_babel_live_regex_infinite_loop() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let mut vm = ChimeraVM::new(Dna::from_genes(vec![]));
            // initialize grid with "a"
            for y in 0..GRID_SIZE {
                for x in 0..GRID_SIZE {
                    vm.grid[y][x] = Value::Str("a".to_string());
                }
            }
            // Set start of Regex at (0,0)
            vm.grid[0][0] = Value::Str("[".to_string());

            // Execute parse. This should traverse the whole grid, wrap around, and loop forever
            // because "]" is never found.
            // Start at (0,0) with dummy input.
            let result = exec_live_parse(&mut vm, 0, 0, "dummy".to_string());
            let _ = tx.send(result);
        });

        // The grid is small (usually 64x64 or 16x16 depending on config).
        // Traversing it once shouldn't take long.
        // If it takes more than 500ms, it's likely stuck in a loop.
        match rx.recv_timeout(Duration::from_millis(500)) {
            Ok(_) => {
                // Function returned.
                // If it returns, it means it didn't hang.
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                panic!("Babel Live Parse hanged! Infinite loop detected.");
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                panic!("Thread disconnected unexpectedly.");
            }
        }
    }

    #[test]
    fn test_babel_live_action_infinite_loop() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let mut vm = ChimeraVM::new(Dna::from_genes(vec![]));
            for y in 0..GRID_SIZE {
                for x in 0..GRID_SIZE {
                    vm.grid[y][x] = Value::Str("a".to_string());
                }
            }
            // Set start of Action at (0,0)
            vm.grid[0][0] = Value::Str("{".to_string());

            // Execute parse.
            let result = exec_live_parse(&mut vm, 0, 0, "dummy".to_string());
            let _ = tx.send(result);
        });

        match rx.recv_timeout(Duration::from_millis(500)) {
            Ok(_) => {}
            Err(mpsc::RecvTimeoutError::Timeout) => {
                panic!("Babel Live Parse (Action) hanged! Infinite loop detected.");
            }
            Err(_) => panic!("Thread error"),
        }
    }
}
