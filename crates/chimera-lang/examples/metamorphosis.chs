strand main {
    "Metamorphosis Demo" print

    # Construct the new DNA on the Grid (Row 0)
    # Target Program: push("Reborn!") print() push(0) apoptosis()

    "push" 0 0 g_write
    "Reborn!" 0 1 g_write
    "print" 0 2 g_write
    "push" 0 3 g_write
    0 0 4 g_write
    "apoptosis" 0 5 g_write

    "Grid prepared. Triggering..." print
    metamorphosis

    "FAIL: Old DNA still running!" print
}
