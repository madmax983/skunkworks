#!/bin/bash
for exp in experiments/*; do
  if [ -d "$exp" ]; then
    if [ ! -f "$exp/README.md" ]; then
      echo "No README: $exp"
    fi
  fi
done
