#!/bin/bash

# Find all Cargo.toml files in experiments/
for cargo_file in $(find experiments/ -name "Cargo.toml"); do
    experiment_dir=$(dirname "$cargo_file")
    experiment_name=$(basename "$experiment_dir")

    # Check if README.md exists
    if [ ! -f "$experiment_dir/README.md" ]; then
        echo "$experiment_name: MISSING README.md"
    else
        echo "$experiment_name: HAS README.md"
    fi

    # Check if we can build the experiment
    if ! cargo check -p "$experiment_name" >/dev/null 2>&1; then
        echo "$experiment_name: COMPILATION FAILURE"
    else
        echo "$experiment_name: COMPILES"
    fi

    echo "---"
done
