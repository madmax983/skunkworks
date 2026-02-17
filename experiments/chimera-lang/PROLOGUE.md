# Prologue Language 🔮

Prologue is a Rune-based Logic Language that runs on the Chimera Grid.
It combines visual programming with logic inference.

## Activation

Toggle Prologue mode with `[` in the TUI or the `prologue` OpCode.

## Runes

Runes are single characters placed on the grid that define logic rules.

*   `?` **Query**: Scans the cell to the North. If not empty, logs a query.
*   `!` **Fact**: Asserts a fact about the cell to the North.
*   `@` **Agent**: A logic agent that can move or interact based on rules.
*   `~` **Stream**: Connects logic components.
*   `&` **AND**: Logic conjunction.
*   `|` **OR**: Logic disjunction.

## Logic Engine

The Prologue engine scans the grid every tick (`prologue_tick`).
It identifies Runes and infers rules dynamically.

Example:
```
  A
  |
  ?
```
This implies "Query A".

## Integration

Prologue runs alongside Nova physics and Chimera DNA execution.
Use `Rune(char, y, x)` to place runes programmatically.
