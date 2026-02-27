
config {
    mode: Normal
}

agents {
    X: {
        # Custom Agent "X"
        # State: [dy, dx, underfoot]
        # Logic: Always move South (dy=1, dx=0)

        # Stack: [dy, dx, underfoot]
        # We need to return: [dy, dx, new_state]
        # Implementation expects stack: [dx, dy, new_state] (Top is new_state)

        # 1. Pop underfoot (keep it clean)
        drop

        # 2. Pop dx
        drop

        # 3. Pop dy
        drop

        # 4. Push new DX (0)
        0

        # 5. Push new DY (1)
        1

        # 6. Push new State (Just keep empty for now)
        0
    }
}

grid {
    . . . . .
    . . X . .
    . . . . .
    . . . . .
    . . . . .
}
