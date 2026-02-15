#!/bin/bash

experiments=(
    "chimera-specter"
    "hanging-gardens"
    "code-acoustics"
    "stardust-compiler"
    "chimera-stardust"
    "chimera-terra"
    "turing-terra"
    "chimera-defense"
    "chaotic-defense"
)

echo "Checking compilation for recent hybrids..."
for exp in "${experiments[@]}"; do
    if [ -d "experiments/$exp" ]; then
        if cargo build -p "$exp" --quiet; then
            echo "✅ $exp: COMPILES"
        else
            echo "❌ $exp: FAILS"
        fi
    else
        echo "❓ $exp: NOT FOUND"
    fi
done
