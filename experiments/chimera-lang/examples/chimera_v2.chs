strand main {
    "=== Chimera 2.0 Evolution ===" print
    run_oracle
    run_chaos
    "Done." print
    apoptosis
}

strand run_oracle {
    oracle {
        fact(man, "socrates")
        # Rule: mortal(X) :- man(X)
        # Defined as rule(HeadArgs...) :- BodyPredicates...
        # Head becomes any(HeadArgs...)
        rule(mortal, ?X) :- man(?X)
    }

    "Querying Oracle..." print
    # Check if socrates is mortal. ?Who should bind to "socrates"
    any("mortal", "?Who") query

    # Check if success (1) or fail (0)
    brz(oracle_fail)
    "Oracle: Success!" print
    jump(oracle_end)
}

strand oracle_fail {
    "Oracle: Failed." print
    jump(oracle_end)
}

strand oracle_end {
    drop # Drop any artifacts
}

strand run_chaos {
    "Entering Chaos..." print
    chaos {
        "Chaos Active (10%)" print
        100 200 add print
        "Surviving..." print
    }
    "Chaos Ended." print
}
