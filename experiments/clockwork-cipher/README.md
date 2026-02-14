# Clockwork Cipher

A Moonshot experiment combining **Gear Ratios** and **Encryption Key Generation**.

## Concept
A mechanical encryption machine driven by a system of gears with prime number tooth counts (13, 17, 19, 23).
As the main drive gear rotates, the prime gears turn at different rates, creating a complex, long-period interference pattern.
Cam disks attached to the prime gears are read by mechanical feeler arms.
The combined position of these arms generates a stream of pseudo-random bytes—the "Keystream".

## Physics
- **Engine**: `bevy` + `bevy_rapier2d`
- **Gears**: Physically simulated with accurate collision geometry (involute profiles).
- **Dynamics**: Torque-driven simulation with friction and restitution.

## Controls
- (Headless/Auto-run currently) - Visualizes the mechanism and the generated key.
