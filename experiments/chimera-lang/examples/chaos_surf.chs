# Chaos Surfing Demo 🌀
# Demonstrates the Strange Attractor dynamics

strand main {
    "Initiating Lorenz Attractor..." print

    # Initialize Attractor (Mode 0 = Lorenz)
    0 attractor_init

    # Enter the Chaos Loop
    jump(chaos)
}

strand chaos {
    # Step the simulation (dt = 10/1000 = 0.01)
    10 attractor_step

    # Map Attractor X state to Reality Glitch Level (Target 0)
    0 attractor_map

    # Map Attractor Y state to Havoc Rate (Target 1)
    1 attractor_map

    # Print a heartbeat message
    "Surfing the Strange Attractor..." print

    # Loop forever
    jump(chaos)
}
