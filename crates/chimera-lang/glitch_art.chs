# Glitch Art Generator
# Demonstrates the new Entropy Visualization system

strand main {
    "Initializing Reality Decay..." print

    # Start with a bang - destroy center of grid
    8 8 disintegrate

    jump(pulse)
}

strand pulse {
    # Add visual chaos (Severity 5)
    # This increases global glitch_level
    5 glitch
    "Chaos Rising..." print

    # Sonic accompaniment (Chord: Genesis)
    "Do" sing
    "Mi" sing
    "Sol" sing

    # Wait loop
    20
    jump(wait)
}

strand wait {
    1 sub
    dup
    brz(stabilize_phase)
    jump(wait)
}

strand stabilize_phase {
    # Restore order slightly
    "Stabilizing..." print
    2 stabilize

    # Loop back to pulse
    jump(pulse)
}
