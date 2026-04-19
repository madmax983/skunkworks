#!/bin/bash
sed -i 's/Err(_) => return Ok/Err(_) => Ok/g' experiments/chimera-lang/src/vm/akashic.rs
sed -i 's/{ let r = Self { storage: HashMap::new(), karma: 0, memories: HashMap::new(), corrupted: true, file_path: file_path.to_string() }; r }/Self { storage: HashMap::new(), karma: 0, memories: HashMap::new(), corrupted: true, file_path: file_path.to_string() }/g' experiments/chimera-lang/src/vm/akashic.rs
