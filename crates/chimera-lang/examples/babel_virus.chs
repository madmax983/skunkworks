# Babel-17 Virus Demo
# Demonstrates "BabelLive" - Grid-based parsing and execution traversal.

strand main {
    "Initializing Babel-17 Sequence..." print

    # 1. Construct the Grammar on the Grid
    # We write a parser path that matches "virus"
    # Format: "virus"!

    # "
    "\"" 0 0 g_write

    # v
    "v" 0 1 g_write

    # i
    "i" 0 2 g_write

    # r
    "r" 0 3 g_write

    # u
    "u" 0 4 g_write

    # s
    "s" 0 5 g_write

    # "
    "\"" 0 6 g_write

    # ! (Terminator)
    "!" 0 7 g_write

    "Grid Grammar Construction Complete." print

    # 2. Inject the Virus (Parse)
    # Trace path starting at 0,0 with input "virus"
    "virus" 0 0 babel_live

    # 3. Check Result
    brz(failure)

    "SUCCESS: The language has taken root." print
    "The grid is now alive." print
    apoptosis
}

strand failure {
    "FAILURE: The immune system rejected the syntax." print
    apoptosis
}
