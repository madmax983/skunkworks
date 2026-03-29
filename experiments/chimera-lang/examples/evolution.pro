config {
    mode: Orca
}

grid {
    "!"  "~"  "~"  "£"
    "."  "."  "."  "."
    "🤖" "~"  "~"  "K"
}

definitions {
    £: {
        strand custom_event {
            "DNA sequence triggered by £ rune" print
        }
    }
}

dna {
    strand main {
        "Evolution Simulation Started" print
    }
}
