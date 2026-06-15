#!/bin/bash
for test in "tui-shared" "poincare-disk" "neuro-sim" "resonance-audio" "hyper-system" "physics-pbd" "locus" "platter" "market-sim" "gray-scott" "git-associates" "origami" "quipu" "flocking"; do
    cargo test -p $test --test havoc
done
