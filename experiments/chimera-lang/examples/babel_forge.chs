# Babel Forge: Defining a new language within Chimera
# This demonstrates the "Mad Scientist of Language" capability.

strand main {
    "⚛️  Initializing Babel Forge..." print

    # 1. Define Grammar for "MathLang"
    # Format: Op Space Arg1 Space Arg2 (e.g., "add 10 20")

    # Op
    "[a-z]+" "Regex" grammar

    # Space
    "\s+" "Regex" grammar

    # Arg1
    "\d+" "Regex" grammar

    # Space
    "\s+" "Regex" grammar

    # Arg2
    "\d+" "Regex" grammar

    # Combine into Seq (using parser_seq_n for 5 items)
    5 parser_seq_n

    dup "📜 Grammar defined: " swap print

    # 2. Parse Input
    "add 10 20"
    "📝 Input: " swap dup print
    # Stack: [ Grammar Input ]
    # Parse expects [ Parser Input ]
    parse

    # Note: Parse might fail if regex engine behaves differently in sandbox
    dup "🌳 Parsed CST: " swap print

    # Check if parse succeeded (Result is Junction or 0)
    # If 0, stop.
    dup 0 eq brz(compile_step)
    "❌ Parse failed." print
    apoptosis
}

strand compile_step {
    "✅ Parse successful! Compiling..." print

    # 3. Compile CST to Chimera Interpreter
    # We push the handler strand index.
    handler
    babel_compile

    dup "🧬 Compiled Strand ID: " swap print

    # 4. Execute the new strand
    "🚀 Executing Compiled Code..." print
    call
}

strand handler {
    # This handler would interpret the CST.
    # For this demo, we just acknowledge execution.
    # The compilation process calls this handler for the Root Node.

    "🎉 Handler Executed! The AST was traversed." print

    # Clean up stack (CST children results would be here)
    # For Seq(5 children), we have 5 items + Type + Count.
    # Just drop everything for demo.
    drop drop drop drop drop drop drop
}
