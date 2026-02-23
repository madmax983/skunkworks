strand main {
    # Build AND Gate at y=5, x=5 Facing East (1)
    # Stack: type=0(AND) dir=1 y=5 x=5 (Top)
    0 1 5 5 construct
    "Built Gate" print

    # Wire Inputs
    # y=5, x=4 (West) -> Stack: 5 4 (Top)
    5 4 wire
    # y=4, x=5 (North) -> Stack: 4 5 (Top)
    4 5 wire

    # Wire Output
    # y=5, x=6 (East) -> Stack: 5 6 (Top)
    5 6 wire

    # Pulse Inputs
    5 4 pulse
    4 5 pulse

    "Pulsed Inputs" print

    # Step Circuit (Manual Clock)
    conduct

    # Read Output at y=5, x=6
    # Should be 2 (Head)
    5 6 g_read
    "Output:" print
    print

    # Cleanup (requires Nova)
    0 apoptosis
}
