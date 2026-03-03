#!/bin/bash
for crate in crates/*; do
  if [ -d "$crate" ]; then
    echo "Dependencies for $(basename $crate):"
    grep -E '^(path|version|git) =' $crate/Cargo.toml || grep -E '^[a-zA-Z0-9_-]+ = ' $crate/Cargo.toml
    echo "-----------------"
  fi
done
