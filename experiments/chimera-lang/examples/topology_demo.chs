strand main {
    # Gain infinite power
    1000 consume

    # Test Sphere (7)
    7 shape

    # Move to (0, 8) from (8, 8)
    -8 0 migrate

    # Verify 0,8 (y, x)
    locate
    8 eq
    swap 0 eq
    add
    2 eq
    brz(fail)

    # Move North (-1, 0) across pole
    # Sphere: (0, 8) -> (-1, 8) -> (0, 0)
    -1 0 migrate
    locate
    0 eq
    swap 0 eq
    add 2 eq
    brz(fail_sphere)

    "Sphere Check Passed" print

    # Test Projective (8)
    8 shape

    # At (0, 0). Move North (-1, 0)
    # Projective: (0, 0) -> (-1, 0) -> (15, 15)
    -1 0 migrate
    locate
    15 eq
    swap 15 eq
    add 2 eq
    brz(fail_proj)

    "Projective Check Passed" print
    apoptosis
}

strand fail {
    "Locate Check Failed" print
    0 0 div
}

strand fail_sphere {
    "Sphere Logic Failed" print
    locate print print
    0 0 div
}

strand fail_proj {
    "Projective Logic Failed" print
    locate print print
    0 0 div
}
