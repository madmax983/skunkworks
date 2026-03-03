config {
    mode: Orca
}

grid {
    "A"  "!"  "~"  "~"  "*"  "~"  "~"  "?"
    "."  "."  "."  "."  "~"  "."  "."  "."
    "£"  "~"  "~"  "~"  "~"  "."  "."  "."
    "B"  "."  "."  "."  "."  "."  "."  "."
    "."  "."  "."  "."  "🤖" "."  "."  "."
    "."  "."  "."  "."  "."  "."  "."  "."
    "!"  "~"  "K"  "~"  "?"  "."  "."  "."
}

definitions {
    A: {
        "Alpha Triggered" print
    }
    B: {
        "Beta Triggered" print
    }
    £: {
        "Custom Rune £ Executed" print
    }
}

dna {
    strand main {
        "Evolution Simulation Started" print
    }
}
