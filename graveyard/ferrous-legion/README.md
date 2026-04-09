# Ferrous Legion

**Lineage:** `ferrous-graph` × `spqr-rsa`

A TUI visualization where Roman Numeral particles interact via "Magnetic Arithmetic".

## Concept
- **Magnetic Arithmetic**: Particles have mass proportional to their Roman Numeral value.
- **Merge**: When particles collide, if their sum <= 1000 (M), they merge into a larger numeral.
- **Bounce**: If their sum > 1000, they bounce off each other.
- **Field**: Particles leave a magnetic trail on the platter, influencing future movements.

## Controls
- `[Arrows]`: Pan the view.
- `[+/-]`: Zoom in/out.
- `[R]`: Reset view.
- `[Q]`: Quit.

## Lineage Details
- **Physics**: Inherited from `ferrous-graph` (Newtonian + Magnetic).
- **Numerals**: Inherited from `spqr-rsa` (Roman Numeral structs and arithmetic).
- **Novel Trait**: Physical accumulation of value.

## License
MIT
