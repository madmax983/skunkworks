# Liquidity Bridge 🐜📈

A hybrid experiment combining **Biomimetic Bridge** (Ant Colony Optimization) and **Market Swarm** (Continuous Double Auction).

## Concept

Ants (Liquidity Providers) build structural bridges across the Spread (Gap) between Bids and Asks.
When the bridge is complete, trades flow across it.
High volatility shakes the bridge, causing liquidity to dry up (ants fall).

## Lineage

- **Parent A:** `experiments/biomimetic-bridge` (Ant bridge building logic)
- **Parent B:** `experiments/market-swarm` (Market simulation, Order Book visualization)
- **Novel Trait:** Biological Liquidity Provision. Visualizing the "Spread" as a physical chasm that must be bridged by agents.

## Running

```bash
cargo run -p liquidity-bridge
```

## Controls

- `q` / `Esc`: Quit
- `r`: Reset simulation

## Legend

- **Green**: Bids (Buyers)
- **Red**: Asks (Sellers)
- **Yellow**: Bridge (Liquidity)
- **White**: Ants (Agents)
- **Black**: Gap (Spread)
