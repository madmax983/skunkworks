strand main {
    // Fuel up first!
    push(1000)
    consume

    push("--- BEGINNING RITUAL ---")
    print

    // Light chaos
    push(20)
    glossolalia
    push("Integrity slightly compromised.")
    print

    push("Speak")
    glossolalia

    push("Generated Speech:")
    print
    dup
    print

    push("Compiling...")
    print

    // Restore integrity fully before compile/exec to avoid crashing the test runner
    push(20)
    clarify

    compile

    push("Executing generated strand:")
    print
    dup
    print

    exec

    push("--- RITUAL COMPLETE ---")
    print
}
