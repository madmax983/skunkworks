# The Philosopher's Stone 💎
# Demonstrates Geometry Transmutation and Alchemy.

strand main {
    "Preparing the Great Work..." print

    # 1. Prepare the Material (DNA -> Grid)
    # We want to write "lead" at (8,8) and "energy" at (8,9).
    # ProjectGeometry uses a Spiral pattern.
    # Spiral: (0,0), (0,1), ...
    # So we need a strand with: [ push("lead") push("energy") ]

    "Materializing Lead and Energy..." print
    push(lead_setup) # Strand Index of lead_setup
    push(8) # Y
    push(8) # X
    project_geometry

    # 2. Perform the Transmutation (Alchemy)
    # Alchemy opcode transmutes the cell at current context_loc based on neighbors.
    # We need to be at 8,8.

    "Transmuting..." print

    # Move context to 8,8.
    # Default context is 8,8. But let's be sure.
    # There isn't a direct "MoveContext" op in basic set, but Migrate(dy, dx) moves it relative.
    # Or just use `alchemy` which operates on current context.

    alchemy

    # 3. Absorb the Result (Grid -> DNA)
    # Read the center cell (8,8). Radius 0 = 1 cell.

    "Absorbing the Gold..." print
    push(0) # Radius
    push(8) # Y
    push(8) # X
    absorb_geometry

    # The new strand index is on the stack.
    # Let's call it!

    "Executing the Result..." print
    call

    # The result (Str("gold")) should be on the stack now.
    print
    "Transmutation Complete." print
}

strand lead_setup {
    push("lead")
    push("energy")
}
