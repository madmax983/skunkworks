# Soroban HFT 🧮📈

A High-Frequency Trading simulator where the Matching Engine runs on Abacus (Soroban) arithmetic.

## Concept
Modern financial systems rely on floating point or fixed point arithmetic. We reject these.
This experiment implements a Limit Order Book and Matching Engine using **pure bead manipulation logic**.

## Arithmetic
The `Soroban` type does not wrap a `u64`. It is a collection of `Column` structs.
*   **Addition**: Simulated by mechanically moving beads (checking `earth_active` count, flipping `heaven_active`, carrying over).
*   **Subtraction**: Simulated by borrowing from neighbor columns.

## TUI Visualization
The interface renders the Order Book using ASCII bead representation:
`[v|....]` = 5
`[^|****]` = 4
`[^|***.]` = 3

*   **Bids**: Green.
*   **Asks**: Red.
*   **Engine**: Runs automatically, placing random orders and matching them.

## Controls
*   `q`: Quit.

## License
MIT
