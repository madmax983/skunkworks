#!/bin/bash

experiments=(
    "ferrous-chimera"
    "luminous-valley"
    "gravitational-bridge"
    "spqr-rsa"
    "quipu-symphony"
)

echo "Phase 1: Verifying Hybrids..."

for exp in "${experiments[@]}"; do
    echo "Checking $exp..."
    if cargo build -p "$exp" --quiet; then
        echo "✅ $exp compiles."
    else
        echo "❌ $exp failed compilation."
    fi
done

echo "Checking condemned experiments..."
if [ -d "experiments/git-quipu" ]; then
    echo "⚠️ git-quipu directory exists."
else
    echo "💀 git-quipu directory not found (Moved to graveyard/ executed)."
fi

if [ -d "experiments/hertzian-shimmer" ]; then
    echo "⚠️ hertzian-shimmer directory exists."
else
    echo "💀 hertzian-shimmer directory not found (Moved to graveyard/ executed)."
fi
