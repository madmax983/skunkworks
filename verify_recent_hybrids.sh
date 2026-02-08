#!/bin/bash
experiments=(
    "hyperbolic-lexicon"
    "ink-jet"
    "morph-physics"
    "hyperbolic-raymarcher"
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
