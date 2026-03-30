config {
    mode: Orca
}

definitions {
    Z: {
        strand custom {
            "Mad Scientist Gene Z Executed!" print
        }
    }
}

grid {
    . . . . . . . . .
    . 1 1 . . . . . .
    . ! ! . . Z . . .
    . ~ ~ . . ? . . .
    . & . . . . . . .
    . ? . . M . . . .
    . . . . . . . . .
    . ₣ 42 10 + . . .
    . . . . . . . . .
}

dna {
    strand boot {
        "Booting Prolouge Laboratory..." print
        push(5) // y
        push(1) // x
        g_read
        "Checking Logic Gate at 1,5:" print
        print
    }
}
