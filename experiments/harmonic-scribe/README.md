# Harmonic Scribe

A hybrid experiment combining **Harmonic Engine** (Mechanical Physics) and **SPQR RSA** (Roman Numeral Arithmetic).

## Concept
The **Harmonic Scribe** is a visualization of a mechanical differential analyzer where the values are represented as Roman Numerals.
The system uses `rapier2d` to simulate mechanical integrators (Ball-and-Disk). The rotation of the output cylinder (continuous analog value) is observed by a "Scribe" who discretizes it into a Roman Numeral (digital value).

## Lineage
- **Parent A**: `experiments/harmonic-engine`
  - Provided the `PhysicsWorld` simulation engine.
  - Mechanical integrator logic (Disk, Ball, Cylinder).
  - TUI rendering of physics bodies using `ratatui` Canvas.
- **Parent B**: `experiments/spqr-rsa`
  - Provided the `Roman` struct and conversion logic (`Roman::from_u64`).
  - The concept of "Ancient Arithmetic".

## Controls
- `push <id> <roman>`: Set the input rate (ball position) of an integrator.
  - Example: `push 0 V` (Sets integrator 0's rate to 5).
- `set <id> <roman>`: Set the current value (rotation) of an integrator.
  - Example: `set 0 X` (Sets integrator 0's value to 10).
- `q` or `Esc`: Quit.

## Emergent Behavior
The hybrid demonstrates the bridge between continuous physics (Analog Computing) and discrete symbolic representation (Roman Numerals). It creates a "Steampunk Rome" aesthetic where calculations are performed by gears but read as ancient symbols.
