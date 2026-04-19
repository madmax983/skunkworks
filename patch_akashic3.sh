#!/bin/bash
sed -i 's/Err(_) => { let mut r = Self { storage: HashMap::new(), karma: 0, memories: HashMap::new(), corrupted: true, file_path: file_path.to_string() }; return Ok(r); },/Err(_) => return Ok(Self { storage: HashMap::new(), karma: 0, memories: HashMap::new(), corrupted: true, file_path: file_path.to_string() }),/g' experiments/chimera-lang/src/vm/akashic.rs
