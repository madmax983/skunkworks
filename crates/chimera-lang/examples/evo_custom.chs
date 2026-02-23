evolution "TargetSearch" {
    population: 50
    mutation_rate: 0.1

    # Custom Fitness Function
    # The candidate code runs first. Its result is on the stack.
    # We want the result to be 42.
    fitness {
        # Stack: [CandidateResult]
        42
        sub
        # Stack: [Diff]
        # We want absolute difference as error score
        dup
        0
        lt
        # Stack: [Diff, IsNegative]
        brz(pos)

        # Negate if negative
        0 swap sub

        strand pos {
            # Stack: [AbsDiff]
            # Done. This value is the fitness score (0 = Perfect).
        }
    }
}

strand seed {
    # Initial guess
    0
}
