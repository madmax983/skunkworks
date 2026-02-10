# Mad Scientist Laboratory
# Demonstration of Genetic Alchemy

strand main {
    "INITIATING GENETIC ALCHEMY..." print

    # 1. Clear Crucible (Mode 1)
    1 crucible

    # 2. Add Strand 1 (Subject) to Crucible
    # Stack: [Subject_Idx] -> [1]
    1
    # Mode 0 (Add)
    0 crucible

    # 3. Add Reagent "Fire" to Crucible
    # Stack: ["Fire"]
    "Fire"
    # Mode 0 (Add)
    0 crucible

    # 4. Transmute! (Mode 2)
    "Transmuting..." print
    2 crucible

    # 5. Withdraw Result (Mode 3)
    # This should push the new strand index to the stack
    3 crucible

    # 6. Check if we got a result
    dup
    "Result Strand Index:" print
    print

    # 7. Execute the new strand
    # Stack: [New_Strand_Idx]
    # We use 'spawn' to run it as a worker organelle
    # Spawn(Type=0, Strand=Top)
    0 swap spawn

    "Experiment Complete." print
}

strand subject {
    "I am the subject." print
    # This number '5' should be increased by Fire
    5 print
}
