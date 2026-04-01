# Chimera Maat ⚖️🧬

**"The Weighing of the Heart... and the Code."**

A hybrid experiment combining **Chimera Lang** (Evolutionary VMs) and **Maat Engine** (Egyptian Fraction Resource Allocation).

## Concept

In this simulation, digital organisms (Chimera VMs) do not run freely. They must **petition** the "Scales of Maat" for CPU time.

Every tick, each organism submits a **Resource Demand**, represented as a rational number (Numerator/Denominator). The Maat Engine decomposes this demand into **Egyptian Fractions** (sums of distinct unit fractions like 1/2 + 1/3 + 1/7) and attempts to allocate blocks of time on a fixed-capacity timeline.

- **Successful Petition**: The VM is granted execution time proportional to its demand.
- **Failed Petition**: The VM is denied resources and suffers "Entropy" (Energy loss + Mutation).

## The Logic

1. **Self-Calculated Demand**: Organisms determine their own demand by manipulating their stack. The top two values are interpreted as Numerator and Denominator.
2. **Greedy Allocation**: The Maat Engine uses the Fibonacci-Sylvester greedy algorithm to allocate time blocks.
3. **Bureaucratic Evolution**: Organisms must evolve to ask for resources that *fit* the available slots in the timeline. If they ask for 1/2 when 1/2 is taken, they die. They must learn to ask for 1/3, 1/4, or weird fractions that sum to available gaps.

## Visualization

- **Top Bar**: The "Scales of Maat" timeline. Blocks of color represent allocated time. Gaps are wasted cycles.
- **List**: The Petitioners. Shows their current demand in **Hieroglyphs** (because why not?), their energy, and whether they were Granted or Denied.

## Lineage

- **Parent A**: `experiments/chimera-lang` - The biological VM and genetic structure.
- **Parent B**: `experiments/maat-engine` - The resource allocation logic and Egyptian math.
- **Splice Surgeon**: Jules
