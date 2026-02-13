# Parity Ruins: Data Sarcophagus

**"The Library of Babel is burning. You are the fire, but you are also the scribe trying to rewrite the books before they turn to ash."**

`parity-ruins` is a digital archaeology simulation that visualizes the struggle of data survival against entropy. It uses **Reed-Solomon erasure coding** to protect an image split into shards.

## Concept
Data is split into **Data Shards** (grayscale) and **Parity Shards** (red).
- **Data Shards** contain the actual information.
- **Parity Shards** contain redundant information calculated via Galois Field arithmetic.

As long as the number of healthy shards (Data + Parity) is greater than or equal to the original Data Shard count, the image can be perfectly reconstructed. If too many shards are corrupted, the reconstruction fails, resulting in noise or glitches.

## Controls
- **Left Click + Drag**: Zero out data (Corruption).
- **Right Click + Drag**: Randomize data (High Entropy).
- **E**: Toggle "Entropy Mode" (Slow, automatic bit rot).
- **R**: Reset the Sarcophagus (Restore original data).

## Stack
- **Rust**
- **Macroquad**: Visualization
- **Reed-Solomon Erasure**: Math
- **CRC32**: Corruption detection

## Moonshot
This experiment combines **Bit Rot Simulation** with **Data Recovery Visualization**. It transforms abstract error correction mathematics into a tangible, interactive "Sarcophagus" that you can vandalize and observe healing in real-time.
