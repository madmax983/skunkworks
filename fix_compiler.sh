#!/bin/bash
sed -i 's/use std::str::FromStr;/use std::str::FromStr;\nuse std::io::Read;/g' experiments/chimera-lang/src/compiler.rs
