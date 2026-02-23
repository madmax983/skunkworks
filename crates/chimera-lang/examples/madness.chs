strand main {
    "Setting up Mad Science Lab..." print

    # --- Gamma (Mutation) Test ---
    # Layout at (5,5):
    #   * Γ .
    # * fires into Γ, Γ writes random char to East (.)

    "Γ" 5 5 g_write
    "*" 5 4 g_write
    "Gamma initialized at 5,5" print

    # --- Sigma (Summation) Test ---
    # Layout at (10,10):
    #     1
    #   2 Σ 3
    #     .
    # Sum = 1+2+3 = 6. Output to South.

    "Σ" 10 10 g_write
    1 9 10 g_write
    2 10 9 g_write
    3 10 11 g_write
    "Sigma initialized at 10,10" print

    # Enable Orca Mode if not already active (it defaults to true in Nova, but good to be explicit)
    orca

    jump(loop)
}

strand loop {
    # Keep the VM alive
    photosynthesize
    jump(loop)
}
