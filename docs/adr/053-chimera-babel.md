# 053. Chimera Babel System (Metalinguistics)

## Status
Accepted

## Context
As Chimera organisms evolved to interact with their environment in more complex ways, they needed to "speak" and "understand" data. Simple pattern matching was insufficient for parsing structured inputs (like JSON, CSV, or other languages) or generating complex outputs (like prose or code).

We needed a system that would allow the VM to define and execute grammars dynamically at runtime, effectively giving it the ability to learn and use languages.

## Decision
We introduced the **Babel** feature set, which implements Parser Combinators and Grammar definitions directly within the VM.

Key components:
*   **Babel State:** A dedicated structure (`BabelState`) within the VM that manages active grammars, parsing context, and linguistic entropy ("Chaos").
*   **Parser OpCodes:**
    *   `ParserMatch`, `ParserRegex`: Create primitive parsers.
    *   `ParserSeq`, `ParserAlt`: Combine parsers (Sequence, Alternative).
    *   `ParserMany`, `ParserOpt`: Repetition and Optionality.
    *   `Parse`: Executes a parser against an input string.
    *   `Generate`: Uses a grammar to produce a valid string (Reverse Parsing).
*   **Grammar Definition:** Parsers are first-class citizens on the stack (represented as Junctions of OpCodes) and can be composed dynamically.

## Consequences

### Positive
*   **Adaptability:** Organisms can parse unknown data formats by evolving the correct grammar.
*   **Communication:** Enables complex, structured communication between organisms or with the host system.
*   **Generativity:** The `Generate` opcode allows for procedural text generation (e.g., naming things, writing stories) constrained by rules.

### Negative
*   **Performance:** Combinator parsing is computationally expensive, especially with backtracking.
*   **Complexity:** Debugging grammar execution inside a stack VM is challenging.
*   **Chaos:** The `Babel` system introduces "Linguistic Entropy", where grammars can degrade over time (simulating language drift), adding instability.
