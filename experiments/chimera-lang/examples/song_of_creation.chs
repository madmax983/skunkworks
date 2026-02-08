strand main {
    "Initializing Song of Creation..." print

    # 1. Register the Song
    # Chord: "Fiat" "Lux"
    # Effect: creation strand
    all("Fiat" "Lux")
    push(creation)
    harmonize

    # 2. Spawn the Choir
    "Spawning Seraphim..." print
    all("Fiat" "Lux")
    choir

    # 3. Enter Main Loop
    jump(loop)
}

strand loop {
    5 photosynthesize
    jump(loop)
}

strand creation {
    "AND THERE WAS LIGHT!" print
    100 5 lumine
    ret
}
