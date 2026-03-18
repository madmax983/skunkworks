#!/bin/bash
cd experiments/chimera-lang
cargo build --features="nova"
cargo test --test biolum_test --features="nova" -- --nocapture
