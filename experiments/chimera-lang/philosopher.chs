strand main {
    "Genesis: Spawning Philosopher" print

    # Register an Omen (Prophecy)
    # If cell(5, 5) becomes 100, execute strand 'truth' (index 3)
    # Effect: 3
    3
    # Condition: any("cell", 5, 5, 100)
    push(any("cell", 5, 5, 100))
    augury
    "Omen Registered: Waiting for Truth at 5,5" print

    # Spawn Philosopher running 'mind' (Strand 1)
    # Type 10 = Philosopher
    # Stack: [Strand, Type] -> Spawn
    1 10 spawn

    jump(loop)
}

strand mind {
    "Philosopher pondering..." print

    # Trigger the prophecy by writing to grid
    100 5 5 g_write
    "Philosopher wrote Truth to 5,5" print

    # Wait loop
    jump(mind_wait)
}

strand mind_wait {
    photosynthesize
    jump(mind_wait)
}

strand truth {
    "PROPHECY FULFILLED! The Philosopher saw it coming." print
    "Gaining Enlightenment Energy..." print
    100 consume
    apoptosis
}

strand loop {
    photosynthesize
    jump(loop)
}
