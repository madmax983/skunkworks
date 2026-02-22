# 048. Chimera Compiler Security Hardening

## Status
Accepted

## Context
As **ChimeraScript** evolved to support macros (`macro`), external file inclusion (`include`), and recursive data structures (`Junction`), the compiler became vulnerable to several classes of Denial-of-Service (DoS) and security attacks:

1.  **Infinite Recursion:** Macros calling themselves or mutually recursive inclusions could cause a stack overflow in the compiler, crashing the host application (the "Havoc" vulnerability).
2.  **Path Traversal:** The `#include` directive allowed arbitrary file reads if filenames were not sanitized (e.g., `include "../../secret.txt"`).
3.  **Deep Nesting:** Deeply nested brackets `[[[[...]]]]` could exhaust the recursion stack of the `pest` parser.

## Decision
We enforce strict, compile-time limits on recursion depth and file access to ensure the stability and security of the runtime.

### 1. Recursion Limits
We introduce hard limits for various nesting types:
*   `MAX_INCLUDE_DEPTH` (32): Limits the depth of `#include` chains.
*   `MAX_PARSE_DEPTH` (256): Limits the depth of recursive grammar structures (e.g., nested Junctions).
*   `MAX_NESTING_DEPTH` (200): A fast-fail pre-check for bracket nesting `() {} []` to protect the parser.

### 2. Circular Dependency Detection
The preprocessor now tracks `visited` paths during inclusion. Any attempt to include a file that is already in the current inclusion stack results in an immediate compilation error.

### 3. Path Sandboxing
All `#include` paths are now resolved relative to a `base_path`.
*   We use `std::fs::canonicalize` to resolve symlinks and `..` segments.
*   We explicitly verify that the resolved path starts with the canonical `base_path`.
*   If a path attempts to escape the sandbox, access is denied.

## Consequences

### Positive
*   **Stability:** The compiler is resilient against stack overflow crashes caused by malformed or malicious code.
*   **Security:** Arbitrary file read vulnerabilities via `include` are mitigated.
*   **Predictability:** Compilation resources (stack usage) are bounded.

### Negative
*   **Complexity Limit:** Valid programs that rely on extremely deep recursion or nesting (e.g., fractals generated via source code structure) may now fail to compile.
*   **Performance:** Canonicalization of paths introduces file system I/O overhead during preprocessing.
