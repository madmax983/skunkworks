strand main {
    # Switch to Grid View
    1 view
    "Constructing Plasmid..." status

    # Write code to grid: push "Hello Plasmid" print

    # Cell 0,0: "push"
    "push" 0 0 g_write

    # Cell 0,1: "Hello Plasmid"
    "Hello Plasmid" 0 1 g_write

    # Cell 0,2: "print"
    "print" 0 2 g_write

    "Incubating..." status

    # Incubate 3 cells at 0,0 -> New Strand Index
    3 0 0 incubate

    "Executing Plasmid..." status

    # Call the new strand (subroutine)
    call_s

    "Plasmid Finished." status

    # Cleanup (suicide to end simulation cleanly)
    apoptosis
}
