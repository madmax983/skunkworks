# Virus Demo 🦠
# Creates a "GLITCH" virus that targets "DATA" cells.

strand setup {
    # 1. Seed the grid with "DATA"
    "DATA" 8 8 g_write
    "DATA" 8 9 g_write
    "DATA" 9 8 g_write
    "DATA" 9 9 g_write
    "DATA" 7 8 g_write
    "DATA" 8 7 g_write

    # 2. Release Virus
    # Stack: [ ..., rate, pattern, name ]
    # Rate: 50%
    # Pattern: "DATA"
    # Name: "GLITCH"
    50 "DATA" "GLITCH" infect

    # Move cursor to center to start infection locally
    8 8 g_read drop # just to set context_loc or verify it works

    # 3. Trigger Outbreak Loop
    jump(loop)
}

strand loop {
    # Trigger viral spread and mutation
    outbreak

    # Gain energy
    photosynthesize
    photosynthesize

    # Loop
    jump(loop)
}
