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
| `?` | **Sink** | **Reads** signal from its **South** (or Self). Triggers log or **Gene Execution**. |
| `~` | **Wire** | Conducts signals in all cardinal directions (N, S, E, W). |
| `&` | **AND Gate** | Output (South/Self) = Input (West) **AND** Input (East). |
| `|` | **OR Gate** | Output (South/Self) = Input (West) **OR** Input (East). |
| `+` | **XOR Gate** | Output (South/Self) = Input (West) **XOR** Input (East). |
| `*` | **Splitter** | Input (North) -> Output (Self). Acts as a diode/repeater allowing W/E/S to read. |
| `#` | **Delay** | Input (North) -> Output (Self) in the **Next Tick**. |
| `@` | **Agent** | A mobile logic cursor that moves towards signals. |
| `$` | **Scribe** | Reads signal from **West**, writes Value to Grid **South**. |
| `%` | **Modulo** | Output (Self) = Input (West) **%** Input (East). |
| `^` | **Jump** | Input (West) -> Output (East). Skips self (Teleport). |
| `M` | **Mutate** | Reads signal from **West**, writes **Random** Value to Grid **South**. |
| `O` | **Organelle** | Reads signal from **West**, spawns **Agent (@)** at Grid **South**. |

### Gene Execution

If a **Sink** (`?`) receives a signal that is a String matching a named Strand in the DNA (e.g. `"my_function"`), it will interrupt the VM and execute that Strand immediately. This allows the logic grid to control the genetic execution flow.

### Signal Theory

*   **Values**: Signals carry the full `Value` type (Int, String, etc.). `Int(0)` and `Str("")` are considered Empty (No Signal).
*   **Propagation**: Signals travel through `~` wires instantly within a tick.
*   **Conflict**: If multiple signals meet on a wire, the behavior is "Last Write Wins".
*   **Logic**:
    *   `AND`: Requires both West and East inputs to be Truthy. Output is 1.
    *   `OR`: Requires at least one of West or East inputs to be Truthy. Output is 1.
    *   `XOR`: Requires exactly one of West or East inputs to be Truthy. Output is 1.
    *   `Delay`: Holds the signal for one tick before releasing it.

### Agents (`@`)

Agents are autonomous cursors.
*   They scan their local neighborhood.
*   They move towards adjacent cells containing signals or wires.

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

**Gene Trigger:**
```
  "func"  (Strand Name)
  !
  #       (Delay 1 Tick)
  ?       (Sink receives "func" -> Executes Strand "func")
```
