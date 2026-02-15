# Soroban Market ⚛️🏺

**High-Frequency Trading on an Ancient Abacus.**

> "The market moves faster than the eye, but the beads move faster than the market." - Genesis

## 📉 Concept
This experiment combines **High-Frequency Trading (HFT)** algorithms with **Ancient Abacus (Soroban)** computation.
A simulated trading bot calculates the **Simple Moving Average (SMA)** of a volatile asset using a 13-column Japanese Soroban.
Every addition and subtraction required for the SMA calculation is performed by physically manipulating the beads of the Soroban.

## 🕹️ Visuals
- **The Soroban**: A 13-column abacus.
  - **Upper Deck (Heaven)**: 1 bead per column (Value 5). Active when down.
  - **Lower Deck (Earth)**: 4 beads per column (Value 1). Active when up.
- **The Market**: A real-time line graph of price history (Green) and the Abacus-calculated SMA (Yellow).
- **Signals**: "BUY" / "SELL" indicators triggered when price crosses the SMA.

## 🛠️ Tech Stack
- **Engine**: `macroquad` (Immediate mode graphics).
- **Logic**: Custom `Soroban` struct with physical state simulation (bead positions, carry/borrow logic).
- **Simulation**: Random walk market data.

## 🚀 Run
```bash
cargo run -p soroban-market
```

## 🧠 Logic
The `Trader` maintains a "Running Sum" on the Soroban:
1. **Add** the new price to the Soroban.
2. **Subtract** the old price (exiting the window) from the Soroban.
3. **Read** the value and divide by `N` (Window Size) to get the SMA.
