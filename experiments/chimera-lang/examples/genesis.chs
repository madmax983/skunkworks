# Genesis.chs - The First Spark of ChimeraScript

strand main {
    "Start Genesis..." print

    # Simple Arithmetic Test
    5 3 add
    dup
    "Calculated: " print
    print

    # Check if result is 8. (5+3=8)
    # sub(8, 8) -> 0
    8 sub

    # Branch if Zero (Equal) to success strand
    brz(success)

    "Math Failed!" print
    jump(end)
}

strand success {
    "Math Verified." print

    # Spawn a child running 'child_logic'
    # Mitosis expects target strand index on stack
    # We use push(strand_name) to put the index on stack
    push(child_logic)
    mitosis

    "Parent done." print
}

strand child_logic {
    "Child active." print
    "Photosynthesizing..." print
    photosynthesize
    "Child done." print

    # Self-destruct
    push(child_logic)
    apoptosis
}

strand end {
    "Terminating." print
}
