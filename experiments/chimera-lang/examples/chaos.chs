# ChimeraChaos Experiment 🧪
# An organism that feeds on chaos and attempts to tune the universe.

strand init {
    # Set global coupling to 0.2 (20)
    push(20) push(2) chaos

    # Initialize counter on stack
    push(0)

    # Jump to loop
    jump(loop)
}

strand loop {
    # Stack: [counter]

    # Check limit (10 iterations)
    dup push(10) sub brz(end)

    # Increment
    push(1) add

    # Stack: [counter+1]

    # 1. Read local chaos value (0-100)
    push(0) chaos

    # Stack: [counter+1, chaos_val]

    # 2. Visualize
    dup print

    # 3. Tune parameter 'r' based on value
    push(100) swap sub
    # Stack: [counter+1, 100 - chaos_val]

    # Set r
    push(1) chaos

    # Stack: [counter+1]

    # Consume energy to stay alive
    photosynthesize

    jump(loop)
}

strand end {
    pop
    print("Chaos Experiment Complete.")
    apoptosis
}
