#!/bin/bash
count=0
for exp in experiments/*; do
  if [ -d "$exp" ]; then
    name=$(basename "$exp")
    if ! grep -q "exclude = .*\b$name\b" Cargo.toml; then
        cargo check -p "$name" > /dev/null 2>&1
        if [ $? -ne 0 ]; then
          echo "$name failed to compile"
          count=$((count+1))
        fi
    fi
  fi
  if [ $count -ge 10 ]; then
      break
  fi
done
