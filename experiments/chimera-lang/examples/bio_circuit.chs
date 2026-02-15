# Bio-Electric Circuit Demo
#
# This example demonstrates the interaction between Orca signals,
# Elektra voltage simulation, and Genetic execution.
#
# Circuit:
# 1. A Bang (*) triggers an Electrode (E).
# 2. The Electrode injects 5V into the grid.
# 3. A Wire (1) conducts the voltage to a Voltmeter (V).
# 4. The Voltmeter (Threshold 1V) detects the pulse and outputs a Bang (*).
# 5. The Bang triggers a Play (P) operator.
# 6. The Play operator calls Strand 1.

strand main {
    # 1. Enable Orca and Elektra modes
    # These are toggle toggles, assuming default OFF.
    # Default is Orca=ON, but let's be sure.
    # Actually, default nova features sets Orca=ON.
    # Elektra might need enabling if not default.
    # We will assume features are enabled in Cargo.toml.

    # 2. Write the Circuit to the Grid

    # Row 1: Parameters
    # (1, 3) = 5 (Voltage Amount for E)
    5 1 3 g_write
    # (1, 5) = 1 (Threshold for V)
    1 1 5 g_write
    # (1, 7) = 1 (Target Strand for P)
    1 1 7 g_write

    # Row 2: Components
    # (2, 2) = * (Signal Source - Bang)
    "*" 2 2 g_write
    # (2, 3) = E (Electrode)
    "E" 2 3 g_write
    # (2, 4) = 1 (Wire - Silicon/Int 1)
    1 2 4 g_write
    # (2, 5) = V (Voltmeter)
    "V" 2 5 g_write
    # (2, 7) = P (Play)
    "P" 2 7 g_write

    # 3. Print status
    "Bio-Circuit Constructed." print
    "Watch the grid for 'Circuit Active!' messages." print

    # 4. Loop forever (so the VM keeps ticking)
    jump(loop)
}

strand loop {
    # Keep alive
    jump(loop)
}

strand payload {
    "⚡ Circuit Active! ⚡" print
}
