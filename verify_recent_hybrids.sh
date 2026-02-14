#!/bin/bash
hybrids=(
    "gaze-attractor"
    "bifurcation-landscape"
    "myco-diffusion"
    "chimera-pachinko"
    "etymological-mycelium"
)

echo "Verifying recent hybrids..."
for h in "${hybrids[@]}"; do
    echo "Checking $h..."
    if cargo build -p "$h" --quiet; then
        echo "✅ $h: COMPILES"
    else
        echo "❌ $h: FAILS"
    fi
done
