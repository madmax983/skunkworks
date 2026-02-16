# Mechanical Market ⚙️📈

**Concentration Level:** [CRITICAL MASS]

A hybrid experiment combining **Analog Computing** with **Market Simulation**.

## 🧬 Lineage

- **Parent A:** `experiments/mechanical-integrator`
    - *Alleles:* Mechanical Differential Analyzer physics (Integrator, Differential, Shaft), Macroquad visualization.
    - *Contribution:* The pricing engine is a physical machine, not a discrete algorithm.
- **Parent B:** `crates/market-sim`
    - *Alleles:* Continuous Double Auction (CDA) grid, Particle-based order flow.
    - *Contribution:* The source of "Force" (Torque) driving the machine.

## 🔬 Experiment

This simulation models a market where:
1.  **Order Flow as Torque:** Bids (Buyers) exert positive torque, Asks (Sellers) exert negative torque.
2.  **Differential Analyzer:** A mechanical differential sums these torques to find the **Net Force**.
3.  **Inertial Pricing:**
    -   First Integrator: $\int (Force - Damping) dt = Velocity$ (Price Momentum).
    -   Second Integrator: $\int Velocity dt = Position$ (Price).
4.  **Feedback Loop:** The Mechanical Price determines the spawn location of new Bids/Asks in the Market Grid.

## 🕹️ Controls

- **Automatic:** The market makes itself. Watch the gears turn.

## 🧮 Components

- **Integrator:** Ball-and-disc integrator. $z = \int y dx$.
- **Differential:** Mechanical adder/subtractor. $c = (a + b) / 2$.
