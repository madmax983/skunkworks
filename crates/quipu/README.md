# Quipu 🧶

> "The Quipu is a device for recording information, consisting of a main cord with smaller cords of different colors attached to it and knotted in various ways." - *Garcilaso de la Vega*

A library for modeling the **Quipu** (Khipu), the ancient Inca recording device used for accounting and census data.
This crate provides data structures to represent Knots, Cords, and the Quipu itself, allowing you to perform
arithmetic operations using the logic of the Inca civilization.

## The Inca Number System

The Incas used a **base-10 positional system**, similar to ours, but represented vertically on hanging cords:

- **The Top:** Higher powers of 10 (Hundreds, Thousands, etc.).
- **The Bottom:** The Units place ($10^0$).
- **The Zero:** Represented by an empty space (no knot) in a position.

## Knots

There are three types of knots used to represent numbers:

1.  **Simple Knot (●):** Represents `1` in the Tens place and higher.
2.  **Long Knot (≡L):** Represents `2` to `9` in the Units place. The number of turns indicates the value.
3.  **Figure-Eight Knot (∞):** Represents `1` in the Units place.

## Example: The Hero's Journey (Accounting for the Harvest)

Imagine you are a *Quipucamayoc* (Keeper of the Quipu), recording the harvest of potatoes and maize.

```
use quipu::{Quipu, Cord, Knot};

// 1. Create a new Quipu to record the harvest.
let mut harvest_record = Quipu::new();

// 2. Record 123 sacks of potatoes.
//    - 1 Hundred (Simple)
//    - 2 Tens (Simple, Simple)
//    - 3 Units (Long Knot with 3 turns)
let potatoes = Cord::from(123);
harvest_record.add_cord(potatoes);

// 3. Record 45 sacks of maize.
//    - 4 Tens (Simple x4)
//    - 5 Units (Long Knot with 5 turns)
let maize = Cord::from(45);
harvest_record.add_cord(maize);

// 4. Calculate the total harvest.
//    The Incas performed arithmetic by moving knots or combining cords.
let total = harvest_record.cords[0].checked_add(&harvest_record.cords[1]).unwrap();

assert_eq!(total.value(), 168);

// Display the total cord (TUI representation)
// Output:
// ●          (1 Hundred)
// ● ● ● ● ● ● (6 Tens)
// ≡8         (8 Units)
println!("{}", total);
```