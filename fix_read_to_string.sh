#!/bin/bash
for file in $(grep -rl "std::io::Read::read_to_string(" experiments/); do
    sed -i 's/std::io::Read::read_to_string(\s*&mut std::io::Read::take(\([a-zA-Z0-9_]*\), limit + 1),\s*&mut \([a-zA-Z0-9_]*\)\s*)/\1.take(limit + 1).read_to_string(\&mut \2)/g' "$file"
done
