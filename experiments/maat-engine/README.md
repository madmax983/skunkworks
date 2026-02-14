# Maat Engine ⚖️𓁨

> "The Scales of Maat must balance. No resource shall be granted unless it is a sum of unit fractions."

**Maat Engine** is a "Moonshot" experiment combining **Egyptian Fractions** (Fibonacci-Sylvester Algorithm) with **Resource Allocation**.

It serves as a Resource Allocator (Scheduler) that enforces a strict "Ancient Law":
- Every resource request (e.g., 3/4 of CPU) is decomposed into distinct Unit Fractions (e.g., 1/2 + 1/4).
- The allocator must find contiguous free blocks of exactly those sizes (1/2 size, 1/4 size) in the timeline.
- Output uses **Egyptian Hieroglyphs** instead of decimal numbers.

## Features
- **Scales of Maat**: The core scheduler logic.
- **Hieroglyphic Output**: Numbers are rendered as 𓏤, 𓎆, 𓍢...
- **TUI Visualization**: Watch as Souls (processes) are weighed and placed onto the Field of Reeds (Timeline).

## Usage
Run the simulation:
```bash
cargo run -p maat-engine
```

- **Space**: Summon a Soul (Process) with a random resource demand.
- **Enter**: Weigh the Heart (Attempt allocation).
- **Q**: Quit.

## The Ancient Math
Egyptian Fractions represent numbers as sums of distinct unit fractions $1/n$.
For example:
$$ 5/21 = 1/5 + 1/27 + 1/945 $$
This forces the allocator to handle fragmentation in a unique way, filling large gaps first (Greedy algorithm) and then smaller dust gaps.

## License
MIT / Ancient Papyrus
