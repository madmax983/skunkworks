# Frankenstein Laboratory Test

strand a {
    "AAAA" print
}

strand b {
    "BBBB" print
}

strand main {
    "Initiating Frankenstein Protocol..." print

    # Gather massive energy for the procedure
    500 consume

    # Push strand indices
    0 # Strand A
    1 # Strand B
    3 # Stitches

    frankenstein

    # Resulting strand index is on stack
    "Monster Created! Strand ID:" print
    dup print

    # Decompile to check structure
    dup decompile
    "Monster DNA:" print
    print

    # Execute the monster
    "It's Alive!" print
    jump_s
}
