# Alchemy Demo
strand main {
    # Setup Grid for Ribosome
    # Track: > 0 1 ~

    # (8,8) >
    ">" 8 8 g_write

    # (8,9) 0 (dy)
    0 8 9 g_write

    # (8,10) 1 (dx)
    1 8 10 g_write

    # (8,11) ~ (Transmute)
    "~" 8 11 g_write

    # Target at (8, 12) (Offset 1,0 from 8,11)
    888 8 12 g_write

    # Spawn Ribosome (Type 4?) at (8,8)
    0 4 spawn

    # Loop to let simulation run
    jump(loop)
}

strand loop {
    jump(loop)
}
