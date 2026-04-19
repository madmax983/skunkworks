#!/bin/bash
sed -i 's/Err(_) => Ok(Self {/Err(_) => return Err("Read Error".to_string()),/g' experiments/chimera-lang/src/vm/akashic.rs
