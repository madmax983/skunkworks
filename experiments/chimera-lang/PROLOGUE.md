# Prologue: Rune Logic 🔮

Prologue is a Grid-based Visual Logic Language embedded within Chimera.
It allows for the construction of "Digital Circuits" and "Logic Agents" directly on the memory grid.

## Activation

Toggle Prologue mode with `[` in the TUI or the `OpCode::Prologue`.

## The Grid Circuit

Prologue treats the grid as a circuit board. Signals propagate instantly (within one tick) through connected components.

### Runes

| Rune | Name | Function |
|---|---|---|
| `!` | **Source** | **Emits** the value of the cell to its **North**. |
| `?` | **Sink** | **Reads** signal from its **South**. If active, triggers a log/event. |
| `~` | **Wire** | Conducts signals in all cardinal directions (N, S, E, W). |
| `&` | **AND Gate** | Output (South) = Input (West) **AND** Input (East). |
| `|` | **OR Gate** | Output (South) = Input (West) **OR** Input (East). |
| `@` | **Agent** | A mobile logic cursor that can interact with the grid. |

### Signal Theory

*   **Values**: Signals carry the full `Value` type (Int, String, etc.).
*   **Propagation**: Signals travel through `~` wires.
*   **Conflict**: If multiple signals meet on a wire, the behavior is "Last Write Wins" or "Undefined" (implementation dependent).
*   **Logic**:
    *   `AND`: Requires both inputs to be Truthy (non-zero Int, non-empty Str). Output is 1.
    *   `OR`: Requires at least one input to be Truthy. Output is 1.

### Agents (`@`)

Agents are autonomous cursors.
*   They scan their local neighborhood.
*   (Future) They can be programmed via the Oracle to move towards specific signals.

## Example

**Simple Circuit:**
```
  42      (Value 42)
  !       (Source reads 42)
  ~       (Wire carries 42)
  ~       (Wire carries 42)
  ?       (Sink receives 42 -> Logs "42")
```

**Logic Gate:**
```
  1   1
  !   !
  ~   ~
  &       (AND Gate)
  ?       (Sink receives 1)
```
