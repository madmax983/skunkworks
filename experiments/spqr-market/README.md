# SPQR Market

**Lineage:** `ferrous-legion` × `process-auction`

A TUI visualization of a Roman Market where Bids (Green) and Asks (Red) are Roman Numeral particles.

## Concept
- **Legion Logic**: Bids drift Up (willing to pay more), Asks drift Down (willing to sell for less).
- **Collision Trading**:
  - If a Bid collides with an Ask:
    - If `Bid Value >= Ask Value` -> Trade Executed! Both particles are removed, and a "Transaction" artifact (Yellow) is left behind.
    - If `Bid Value < Ask Value` -> Price Mismatch! They bounce off each other.
- **Magnetic Stigmergy**: Traders leave trails on the market floor, influencing the movement of others.

## Controls
- `[Arrows]`: Pan the view.
- `[+/-]`: Zoom in/out.
- `[R]`: Reset view.
- `[Q]`: Quit.

## Lineage Details
- **Physics**: Inherited from `ferrous-legion` (Vector physics + Roman Numerals).
- **Market Logic**: Inherited from `process-auction` (Double Auction mechanism, Bids vs Asks).
- **Novel Trait**: Physical Order Book. The market depth is visualized as a physical cloud of numerals.

## License
MIT
