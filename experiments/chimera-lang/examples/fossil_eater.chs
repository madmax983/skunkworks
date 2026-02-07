# Fossil Eater 🦖
# A specialized organism that feeds on git history.

strand main {
    "Scanning temporal strata..." print

    # Get the most recent commit
    1 ancestry

    # Stack: [1, hash]
    # Check if we got anything
    dup
    0 eq
    brz(found_fossil)

    "No fossils found." print
    apoptosis
}

strand found_fossil {
    # Stack: [1, hash]

    # Remove the count (we know it is 1)
    swap drop

    # Stack: [hash]
    dup
    "Excavating commit: " print
    print # Print hash

    # Get the diff (Evolution)
    # Stack: [hash] -> [diff_string]
    evolution

    # Check diff size
    "Digestible biomass (bytes): " print
    dup consume # Convert string length to energy
    print # Print diff content

    "Metabolism complete." print
    s_len print
}
