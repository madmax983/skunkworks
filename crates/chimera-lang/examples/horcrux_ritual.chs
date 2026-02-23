# Horcrux Ritual ⚰️
# Splits the soul of a strand into a grid artifact, then resurrects it.

strand main {
    "Starting Ritual..." print

    # Execute the victim logic
    # We call strand 1
    1 call

    "Victim soul bound. Grid contains Horcrux." print

    # Read grid to verify (optional visual check)
    5 5 g_read print

    # Resurrect!
    "Resurrecting..." print

    # Coords (5, 5)
    5 5

    # Restore Soul
    rebirth

    # Execute the resurrected strand (it gets a new index)
    # Rebirth pushed new index to stack
    "Resurrected strand ID:" print
    dup print

    # Jump to it to verify it works (it enters from start)
    call

    "Ritual Complete." print
}

strand victim {
    "I am the victim." print
    "Binding soul..." print

    # Self-reference
    s_index

    # Coords (5, 5)
    5 5

    # Split Soul (Dies immediately)
    horcrux

    "This should not print." print
}
