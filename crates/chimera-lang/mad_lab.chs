strand main {
    "INITIATING MAD SCIENCE..." print

    # Spawn Mad Scientist (Type 10)
    push(scientist_dna)
    10
    spawn

    "Mad Scientist Spawned." print

    jump(wait)
}

strand wait {
    photosynthesize
    jump(wait)
}

strand scientist_dna {
    # Mad Scientist logic is mostly native (random events),
    # but we need to keep the organelle alive.
    photosynthesize
    jump(scientist_dna)
}
