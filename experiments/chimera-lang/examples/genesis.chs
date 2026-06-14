strand main {
    "Genesis Sequence Initiated" print
    5 3 add print

    # Conditional jump
    0 eq brz(end)

    jump(loop)
}

strand loop {
    # ...
    jump(end)
}

strand end {
    apoptosis
}
