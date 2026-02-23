strand resonator {
    # Oscillate at Golden Frequency (161 Hz approx) with high strength
    push(100) push(161) oscillate
    # Also create high pressure shockwave for mutation (Scream)
    # Duration 5, Strength 100
    push(100) push(5) scream
    jump(0)
}

strand victim {
    # Do nothing, wait to be mutated
    photosynthesize
    jump(0)
}

strand main {
    # Spawn Resonator (Strand 0)
    push(0) # Strand 0
    push(0) # Type 0 (Worker)
    spawn

    # Spawn Victim (Strand 1)
    push(1) # Strand 1
    push(0) # Type 0 (Worker)
    spawn

    "Resonance Experiment Initiated." print
}
