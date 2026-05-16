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
        strand alpha {
            "Alpha Triggered" print
        }
    }
    B: {
        strand beta {
            "Beta Triggered" print
        }
    }
    £: {
        strand custom_rune {
            "Custom Rune £ Executed" print
        }
    }
}

dna {
    strand main {
        "Evolution Simulation Started" print
    }
}
