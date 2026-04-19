#!/bin/bash
for file in $(grep -rl "std::io::Read::read_to_string(" experiments/); do
    sed -i 's/std::io::Read::read_to_string(/std::io::Read::take(file, limit + 1).read_to_string(\&mut content)/g' "$file"
done
