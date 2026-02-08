# Feudalism Simulation 🏰
# Demonstrates Sovereignty and Taxation features.

# Strand 0: The Lord
strand lord {
    "Lord: Claiming Territory..." print

    # Claim 5 cell radius around current location
    5 claim

    "Lord: Setting Tax Rate..." print
    # Set Tax Rate to 2 Energy per tick
    2 tax

    jump(lord_loop)
}

strand lord_loop {
    # Check Wealth (Credits from Tax)
    balance
    "Lord Balance:" print

    # Wait
    photosynthesize
    jump(lord_loop)
}

# Strand 1: The Peasant
strand peasant {
    # Peasant starts at (0,0). Move into Lord's territory (8,8)
    # This will take a while. Let's teleport/portal via migrate?
    # Or just spawn peasant near lord?
    # Default spawn is (8,8).
    # So both start at (8,8).

    "Peasant: Serving the Lord..." print

    # Move East
    0 1 migrate

    # Move West
    0 -1 migrate

    # Try to cede land? (Should fail)
    # 8 8 cede

    jump(peasant)
}
