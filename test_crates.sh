crates=(ferrous-core flocking git-associates gray-scott hyper-system locus market-sim miller-lattice neuro-sim origami physics-pbd platter poincare-disk quipu resonance-audio tui-shared)
for crate in "${crates[@]}"; do
    echo "Checking crate: $crate"
    cargo check -p $crate
done
