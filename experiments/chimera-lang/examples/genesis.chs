# Genesis: The Standard Library of Chimera 🧬
# Defines core macros, utilities, and demonstrates advanced features.

# --- Core Logic ---

macro IF {
    # Expects: [condition] on stack
    # Usage: <cond> IF(true_strand, false_strand)
    # This macro is a bit tricky since we don't have direct if-else
    # But we can use `brz` (Branch if Zero)
    # However, `brz` takes a strand index/label.
    # So this macro is more of a placeholder for the concept.
    # Real if-else is done via:
    #   <cond> brz(false_label)
    #   ... true block ...
    #   jump(end_label)
    #   strand false_label { ... }
    #   strand end_label { ... }
}

# --- Polyglot Grammars ---

grammar simple_math {
    # Defines a simple math parser: "1 + 2" -> [Push(1), Push(2), Add]
    Map(
        Seq(
            Int(Regex("[0-9]+")),
            Regex("[ \t]*\\+[ \t]*"),
            Int(Regex("[0-9]+"))
        ),
        all(
            any("push", ?1),
            any("push", ?3),
            any("add")
        )
    )
}

grammar lisp_lite {
    # A tiny Lisp parser: (add 1 2) -> [Push(1), Push(2), Add]
    # Uses recursive definitions (Ref) if supported, or just simple structure

    # Rule for a list: (op arg1 arg2)
    Map(
        Seq(
            Match("("),
            Regex("[a-z]+"), # Op
            Regex("[ \t]+"),
            Int(Regex("[0-9]+")), # Arg1
            Regex("[ \t]+"),
            Int(Regex("[0-9]+")), # Arg2
            Match(")")
        ),
        all(
            any("push", ?4),
            any("push", ?6),
            any(?2)
        )
    )
}

# --- Chaos & Entropy ---

macro DOOMSDAY {
    chaos {
        100
        entropy
        add
        print
    }
}

# --- Oracle Logic ---

oracle {
    fact(parent("cronus", "zeus"))
    fact(parent("cronus", "poseidon"))
    fact(parent("zeus", "ares"))

    # Sibling rule: X is sibling of Y if Z is parent of X AND Z is parent of Y
    rule(sibling(?x, ?y)) :- parent(?z, ?x), parent(?z, ?y)
}

# --- Main Entry Point ---

strand main {
    "Beginning Genesis..." print

    # Test Polyglot
    "Testing Polyglot Math..." print
    polyglot simple_math { 10 + 20 }
    print # Should be 30

    "Testing Polyglot Lisp..." print
    polyglot lisp_lite { (sub 50 10) }
    print # Should be 40

    # Test Chaos
    "Invoking Chaos..." print
    DOOMSDAY

    # Test Oracle
    "Consulting the Oracle..." print
    # Query: Who are the children of Cronus?
    query(parent("cronus", ?child))
    # If successful, ?child is bound.
    # How do we access bindings? Currently `query` returns success boolean.
    # Future expansion: `find_all` returns a list of bindings.

    "Genesis Complete." print
    apoptosis
}
