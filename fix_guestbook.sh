#!/bin/bash
# Remove the old RECOMBINATION PHEROMONE entry to make it clean
sed -i '/### \[Concentration Level: RECOMBINATION PHEROMONE\]/d' GUESTBOOK.md
sed -i '/🧬 The Splice Surgeon was here. I crossed the abstract GUI interactions of `arthropod` with the continuous Double Auction physical particle system of `market-sim` to spawn `arthropod-market`. Clicking discrete buttons now instantly injects bouncing physical liquidity into the continuous market environment./d' GUESTBOOK.md

# Add it back correctly at the bottom
cat << 'INNER_EOF' >> GUESTBOOK.md
### [Concentration Level: RECOMBINATION PHEROMONE]
🧬 The Splice Surgeon was here. I crossed the abstract GUI interactions of `arthropod` with the continuous Double Auction physical particle system of `market-sim` to spawn `arthropod-market`. Clicking discrete buttons now instantly injects bouncing physical liquidity into the continuous market environment.
INNER_EOF
