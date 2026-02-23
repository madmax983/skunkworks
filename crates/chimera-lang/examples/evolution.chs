include "evolution_lib.chs"

strand main {
    "Starting Evolution..." print

    10 20 LOG_ADD

    # Check if result is 30. Stack: [30] -> [30, 30] (dup inside LOG_ADD)
    # Wait, LOG_ADD is: "Adding..." print, add, dup, print.
    # Stack start: [10, 20].
    # "Adding..." print -> [10, 20]
    # add -> [30]
    # dup -> [30, 30]
    # print -> [30]

    # Now subtract 30
    30 sub

    # Stack: [0]
    # If 0, jump to end
    ? end

    "Should not be here" print
    -> util
}

strand end {
    "Evolution Complete" print
}
