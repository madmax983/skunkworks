# Elektra Circuit Demo
# Run with: cargo run --features elektra -- --input examples/elektra_demo.chs

strand main {
    # Boost energy for long run
    1000 consume

    "Initializing Circuit..." print

    # Place Battery (Source) at (y=8, x=2) with 100V
    # Stack: [voltage, y, x]
    100 8 2 battery

    # Place Ground (Sink) at (y=8, x=12)
    # Stack: [y, x]
    8 12 ground

    # Draw Wire (Silicon Type 1) connecting them
    # Coordinates: (8, 3) to (8, 11)

    # We can use a loop or unroll it.
    # Let's unroll for simplicity in this demo.
    1 8 3 g_write
    1 8 4 g_write
    1 8 5 g_write
    1 8 6 g_write
    1 8 7 g_write
    1 8 8 g_write
    1 8 9 g_write
    1 8 10 g_write
    1 8 11 g_write

    "Circuit Constructed." print
    jump(monitor)
}

strand monitor {
    "Monitor Tick..." print

    # Read Voltage at midpoint (y=8, x=7)
    8 7 sense_volt

    # Print it
    "Voltage at Midpoint:" print
    print

    # Print voltage near battery
    8 3 sense_volt
    "Voltage near Source:" print
    print

    # Print voltage near ground
    8 11 sense_volt
    "Voltage near Sink:" print
    print

    # Loop forever
    jump(monitor)
}
