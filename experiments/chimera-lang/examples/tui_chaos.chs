strand main {
    "CHAOS PROTOCOL INITIATED" 2 tui_mod
    0
    jump(loop)
}

strand loop {
    # Stack: [ counter ]
    dup
    1 add

    # Shake (Mode 1)
    dup 1 tui_mod

    # Glitch (Mode 0)
    dup 0 tui_mod

    # Check if > 50
    dup 50 gt
    brz(next)

    # Reset
    drop 0

    jump(next)
}

strand next {
    photosynthesize
    jump(loop)
}
