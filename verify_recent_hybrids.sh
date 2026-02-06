#!/bin/bash

experiments=(
    "git-landscape"
    "miller-fs"
    "chimera-automaton"
    "git-cantata"
    "cam-automaton"
    "chimera-fossil"
    "laban-rover"
    "quantum-boids"
    "compost-chimera"
)

echo "Checking compilation of recent hybrids..."
for exp in "${experiments[@]}"; do
    echo "---------------------------------------------------"
    echo "Checking $exp"
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
