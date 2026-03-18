#!/bin/bash
RUST_BACKTRACE=1 cargo test -p chimera-lang --test biolum_test -- --nocapture
