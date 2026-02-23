# Gene Drive Demo
# Demonstrates CRISPR-Cas9 editing of a running genome.

strand target {
    "Target: I am healthy." print
    # Sequence to target: push(1) push(2) add
    1 2 add
    "Target: 1 + 2 calculated." print
    print
}

strand main {
    "Gene Drive: Scanning for target sequence..." print

    # Scan 'target' strand for the pattern [Push, Push, Add]
    crispr(target) {
        pattern: push, push, add

        # Stack contains match_index (or -1 if not found)
        dup
        "Gene Drive: Found pattern at index: " print
        print

        # Cut the target strand at the match point
        # cas9_cut args: [strand_idx, cut_idx]
        # Stack: [match_index]
        target  # Push target strand index -> [match_index, target_idx]
        swap    # -> [target_idx, match_index]

        cas9_cut

        # Stack: [new_strand_idx]
        "Gene Drive: Cut successful. Viral payload inserted (metaphorically)." print

        # Run the excised tail (new strand) to prove it exists
        "Gene Drive: Executing excised tail..." print
        call
    }
}
