# 🕰️ Clockwork Chimera 🧬

> "The ticking of the clock is the heartbeat of the machine."

A hybrid experiment combining a physics-based mechanical clock with a biological virtual machine.

## 🧬 Lineage

- **Parent A**: `experiments/verge-computer` (Physics-based Verge Escapement)
- **Parent B**: `experiments/chimera-lang` (Evolutionary Virtual Machine)

## 🔬 Concept

**Mechanical Computing**: The Chimera VM, normally a free-running software organism, is here constrained by physical laws. It executes exactly one instruction for every "tick" of the simulated verge escapement mechanism.

The physics simulation (Bevy + Rapier2D) drives the logic. If the clock stops, the mind stops.

## ⚙️ How it Works

1.  **The Engine**: A mainspring applies torque to an Escape Wheel.
2.  **The Regulator**: A pendulum-driven Anchor allows the wheel to advance one tooth at a time.
3.  **The Bridge**: A system detects the discrete "ticks" of the wheel.
4.  **The Mind**: Each tick triggers `vm.step()` in the Chimera VM.

## 🧪 Experiment: Fibonacci Clock

The DNA loaded into the VM calculates the Fibonacci sequence using the grid as memory registers.
Watch the "Stack Top" in the UI increment as the gears turn.

## 🏃 Running

```bash
cargo run -p clockwork-chimera
```
