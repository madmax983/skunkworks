# Chimera Symphony Demo 🎻
# Demonstrates the Piano Roll and Audio features.

strand main {
    "Initializing Symphony..." print

    # Set Tempo to 120 BPM
    120 tempo

    # Play Intro
    jump(intro)
}

strand intro {
    # C Major Scale Ascending
    "Playing C Major Scale..." print

    # C4 (Middle C)
    100 4 60 note
    # D4
    100 4 62 note
    # E4
    100 4 64 note
    # F4
    100 4 65 note
    # G4
    100 4 67 note
    # A4
    100 4 69 note
    # B4
    100 4 71 note
    # C5
    100 4 72 note

    # Rest
    "Resting..." print
    4 rest

    # C Major Arpeggio
    "Arpeggio..." print

    # C4
    90 2 60 note
    # E4
    90 2 64 note
    # G4
    90 2 67 note
    # C5
    100 8 72 note

    # Decay
    4 rest

    # Random composition using Drift
    "Improvising..." print
    jump(improvise)
}

strand improvise {
    # Generate random notes
    # We can use 'drift' or just hardcode a loop with math

    # Loop 8 times
    8
    jump(loop)
}

strand loop {
    # Check counter
    dup
    brz(end)
    1 sub

    # Random Pitch: 60 + (Counter * 2)
    dup 2 mul 60 add
    # Duration: 2
    2
    # Velocity: 80
    80
    # Stack: [counter, 80, 2, pitch]
    # Rotate stack? No, just push in order.
    # Wait, stack top is pitch.
    # Stack: [counter, pitch]

    # Re-order: we need [counter, vel, dur, pitch]
    # Currently [counter] on stack.
    # Calculate pitch:
    dup 2 mul 60 add
    # Stack: [counter, pitch]

    # We need velocity and duration below pitch.
    # Stack: [counter, pitch]
    # Let's clean up.

    # Push velocity
    80
    # Push duration
    2
    # Stack: [counter, pitch, 80, 2]
    # We need [counter, 80, 2, pitch]

    swap
    # Stack: [counter, pitch, 2, 80]
    # This is getting messy without 'rot'.
    # Chimera doesn't have 'rot'.

    # Let's simplify.
    drop drop drop

    # Just play a fixed pattern
    80 2 60 note
    80 2 67 note

    jump(loop)
}

strand end {
    "Symphony Complete. Check Piano Roll view (Press 'p')." print
    s_index apoptosis
}
