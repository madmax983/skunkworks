#!/bin/bash
cd experiments/chimera-lang
cp ../../run_test4.rs src/bin/
cargo run --bin run_test4 --features="nova"
