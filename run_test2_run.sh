#!/bin/bash
rustc --edition 2021 run_test2.rs -L dependency=target/debug/deps --extern chimera_lang=target/debug/libchimera_lang.rlib
./run_test2
