strand main {
    "Elektra Component Test" print

    # Energy
    photosynthesize photosynthesize photosynthesize photosynthesize
    photosynthesize photosynthesize photosynthesize photosynthesize
    photosynthesize photosynthesize photosynthesize photosynthesize
    photosynthesize photosynthesize photosynthesize photosynthesize

    # 1. Forward Bias Test
    100 5 5 battery
    1 5 6 diode
    50 5 7 muscle
    5 8 ground

    nop nop nop nop nop

    5 7 sense_volt
    "Muscle V (Forward):" print
    print

    # 2. Reverse Bias Test
    # Reset voltages
    5 6 ground
    5 7 ground
    nop

    # Rebuild
    3 5 6 diode
    50 5 7 muscle

    # Wait
    nop nop nop nop nop nop nop nop nop nop

    5 7 sense_volt
    "Muscle V (Reverse):" print
    print

    # 3. Transistor Test
    "Transistor Test..." print
    # Battery at (10, 5)
    100 10 5 battery

    # Transistor at (10, 6) Base North (0)
    0 10 6 transistor

    # Muscle at (10, 7)
    50 10 7 muscle

    # Ground at (10, 8)
    10 8 ground

    # Base Control: Battery at (9, 6)
    100 9 6 battery

    nop nop nop nop nop

    10 7 sense_volt
    "Transistor On V:" print
    print

    # Turn off Base (Ground it)
    9 6 ground

    nop nop nop nop nop

    10 7 sense_volt
    "Transistor Off V:" print
    print

    apoptosis
}
