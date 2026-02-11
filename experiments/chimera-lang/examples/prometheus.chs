strand main {
    "INIT PROMETHEUS" print
    logos

    # Assert reaction("fire", "water", "steam", "air")
    # Using explicit push with junction syntax
    push(any("reaction", "fire", "water", "steam", "air"))
    assert
    "Reaction Rule Asserted" print

    # Rule: query(?Y, ?X, "STEAM_FOUND") :- cell(?AX, ?AY, "steam")
    # Head: query(?Y, ?X, "STEAM_FOUND")
    # Use variable names starting with ?
    push(any("query", "?Y", "?X", "STEAM_FOUND"))

    # Body: cell(?AX, ?AY, "steam")
    push(any("cell", "?AX", "?AY", "steam"))

    # OpCode::Rule pops Body (Top), then Head.
    # Stack: [Head, Body] -> rule
    rule
    "Query Rule Asserted" print

    # Grid Setup
    "fire" 2 2 g_write
    "water" 3 2 g_write
    "Q" 5 5 g_write
    "Grid Initialized" print

    # Loop Counter
    20
    jump(loop)
}

strand loop {
    # Check if done
    dup brz(finish)

    1 sub

    # Check Result at 6,5 (South of Q at 5,5)
    # Stack: [ ..., ctr ] -> [ ..., ctr, 6, 5 ]
    6 5 g_read
    # Stack: [ ..., ctr, val ]
    dup brz(next)

    "ALERT: " print
    print # Print the value found

    # Clear output
    0 6 5 g_write

    jump(loop)
}

strand next {
    drop # drop the 0 value from g_read
    jump(loop)
}

strand finish {
    drop # drop counter
    "Simulation Complete" print
    apoptosis
}
