#[cfg(all(test, feature = "nova"))]
mod tests {
    use crate::prelude::*;
    use std::fs;
    use std::io::Write;
    use std::path::Path;

    #[test]
    fn test_ipc_bomb_unbounded_read() {
        // 👺 HAVOC: Testing Unbounded File Read in IPC
        // Attack: Create a 5MB JSON file representing a valid massive Junction.
        // Expectation: VM should reject it (or handle gracefully), not read it into memory.
        // Currently: It reads it all.

        let channel = 666;
        let ether_dir = Path::new(".chimera_ether");
        let channel_dir = ether_dir.join(channel.to_string());

        // Cleanup previous run
        if channel_dir.exists() {
            fs::remove_dir_all(&channel_dir).unwrap();
        }
        fs::create_dir_all(&channel_dir).unwrap();

        // Create 5MB JSON file
        // Valid Value::Junction structure: {"Junction": ["Dish", [ {"Int": 0}, ... ]]}
        let file_path = channel_dir.join("bomb.json");
        let mut file = fs::File::create(&file_path).unwrap();

        file.write_all(b"{\"Junction\": [\"Dish\", [").unwrap();

        // Write massive array of Ints
        // Each item: {"Int":0}, (9 chars)
        // 500,000 items * 9 bytes ~= 4.5 MB
        let chunk = b"{\"Int\":0},";
        for _ in 0..500_000 {
            file.write_all(chunk).unwrap();
        }
        // Remove last comma if strict JSON? Serde might be lenient with trailing comma?
        // No, trailing comma in JSON is invalid.
        // But I wrote 500k chunks. I should write one without comma at end or seek back.
        // Easier: Write {"Int":0} at the end without comma.
        file.write_all(b"{\"Int\":0}").unwrap();

        file.write_all(b"]]}").unwrap(); // Close array and object

        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);

        // Prepare stack for receive: push channel ID
        vm.stack.push(Value::Int(channel));

        // Call receive
        // Note: crate::vm::ipc must be visible.
        crate::vm::ipc::receive(&mut vm);

        // Analyze result
        let result = vm.stack.pop();

        // If the fix works, it should reject the file (so stack has 0 or nothing relevant).
        // If vulnerable, it loaded the array.

        match result {
            Some(Value::Junction(_, list)) => {
                if list.len() > 1000 {
                    panic!(
                        "VULNERABILITY CONFIRMED: VM read a huge file ({} items) into memory!",
                        list.len()
                    );
                }
            }
            Some(Value::Int(0)) => {
                // This means "Channel empty" (file rejected/deleted).
                // This is the desired secure behavior.
            }
            _ => {
                // Any other result is fine (maybe error string on stack?)
            }
        }

        // Cleanup
        if channel_dir.exists() {
            fs::remove_dir_all(&channel_dir).unwrap();
        }
    }
}
