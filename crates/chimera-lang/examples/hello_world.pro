config {
    mode: Orca
}

grid {
    . . . . .
    . 1 1 . .
    . ! ! . .
    . ~ ~ . .
    . & . . .
    . ? . . .
}

dna {
    strand main {
        push("Reading Grid at 1,1:")
        print
        push(1) // y
        push(1) // x
        g_read
        print
    }
}
