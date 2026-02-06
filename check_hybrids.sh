#!/bin/bash

experiments=(
    "code-bio-dome"
    "system-bio-dome"
    "fluid-rain"
    "digital-compost"
    "luminous-flock"
    "heap-arena"
    "crate-radar"
    "quipu-symphony"
    "phonetic-flock"
    "voronoi-ants"
    "tectonic-git"
    "struct-harmonics"
    "thermo-termites"
    "hyperbolic-git"
    "git-landscape"
    "miller-fs"
    "chimera-automaton"
    "git-cantata"
    "chimera-tardis"
    "chaos-monitor"
    "syntax-fold"
)

echo "Checking compilation..."
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
