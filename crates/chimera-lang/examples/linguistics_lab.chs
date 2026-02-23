# Linguistics Lab Experiment
#
# Demonstrates new linguistic runes:
# ↑ (Shift Up)
# ↓ (Shift Down)
# ≅ (Levenshtein Distance)

strand main {
    "Initializing Linguistics Lab..." print

    # Circuit 1: Uppercase Transformation
    # "hello" -> ! -> ↑ -> ?
    # If successful, "HELLO" signal triggers the HELLO strand.

    "hello" 5 4 g_write
    "!"     5 5 g_write
    "↑"     5 6 g_write
    "?"     5 7 g_write

    # Circuit 2: Levenshtein Distance
    # West: "kitten" -> !
    # North: "sitting" -> !
    # ≅ calculates distance (3)
    # $ scribes result to South

    # North Input Setup
    "sitting" 7 5 g_write
    "!"       7 6 g_write

    # West Input Setup & Operation
    "kitten"  8 4 g_write
    "!"       8 5 g_write
    "≅"       8 6 g_write
    "$"       8 7 g_write

    "Circuits Constructed." print
    "Expect: 'HELLO' trigger and Distance 3 at (9, 7)." print

    # Enter loop to keep VM alive
    jump(loop)
}

strand loop {
    jump(loop)
}

strand HELLO {
    "✨ MATCH: Recieved HELLO signal! ✨" print
}
