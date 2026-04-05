#!/bin/bash
for exp in experiments/*; do
  if [ -d "$exp" ]; then
    echo "Checking $exp"
    if [ ! -f "$exp/README.md" ]; then
      echo "No README: $exp"
    fi
    cd "$exp"
    cargo check --quiet > /dev/null 2>&1
    if [ $? -ne 0 ]; then
      echo "Compile failed: $exp"
    fi
    cd - > /dev/null
  fi
done
