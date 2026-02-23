# Void Song and Prophecy Demo

strand main {
    "Initializing Ritual..." print

    # Gather Massive Energy
    1000 consume
    "Energy Gathered." print

    # 1. Setup the Altar (Grid)
    # Write "Fire" at 8,8 (Start location)
    "Fire" 8 8 g_write
    "Water" 8 9 g_write
    "Earth" 9 8 g_write
    "Air" 9 9 g_write

    # 2. Consult the Oracle (Prophecy)
    # Check if running for 20 ticks leads to death (it shouldn't)
    # Note: Prophecy is expensive (50 + ticks/2). We need > 100 energy.
    20 prophecy

    # Check result
    # If Prophecy = 1 (Death), 1-1=0, Jump to doom.
    # If Prophecy = 0 (Life), 0-1=-1, Continue.
    1 sub brz(doom)
    "Prophecy: Survival is likely." print
    jump(summon)
}

strand doom {
    "Prophecy: DOOM DETECTED!" print
    # Try to change fate?
    jump(summon)
}

strand summon {
    # 3. Summon the Void

    # Spawn Void at 8,8 (consumes "Fire")
    void
    "Void Summoned at 8,8" print

    # Move to 8,9 (East)
    0 1 migrate

    # Spawn another Void for Water
    void
    "Void Summoned at 8,9" print

    # Wait loop to let organelles tick
    10
    jump(wait)
}

strand wait {
    # Decrement loop counter
    1 sub
    dup
    brz(end)
    jump(wait)
}

strand end {
    "Ritual Complete." print
    s_index apoptosis
}
