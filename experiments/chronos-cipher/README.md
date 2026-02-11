# Chronos Cipher ⚛️

**Genesis: The Horologist**

A mechanical cryptography machine driven by a physical **Verge Escapement**.
This experiment combines **Verge Escapement** (time regulation) with **Encryption Key Generation** (gear ratios).

## Concept
The "Chronos Cipher" generates a cryptographic key stream not through software algorithms, but through the physical interaction of a gear train.
*   **Power Source**: A simulated high-torque mainspring.
*   **Regulator**: A Verge Escapement (Crown Wheel + Pallets + Foliot) that ensures the machine ticks at a physical rate.
*   **Cipher Core**: A train of gears with prime-number tooth counts (30 -> 23 -> 19 -> 17 -> 13 -> 7).
*   **Key Generation**: At each "tick", the angular position of every gear is sampled and XORed together to produce a byte.

The security of the cipher depends on the chaotic micro-variations in the physics simulation (floating point determinism notwithstanding) and the long period of the prime-number gear train.

## Mechanism
*   **Physics**: Built with `bevy_rapier2d`.
*   **Visuals**: Vector graphics using `bevy_prototype_lyon`.
*   **Logic**: `cipher.rs` reads the physical state of the `Gear` components.

## Running
```bash
cargo run -p chronos-cipher
```

## Future Work
*   Sonification of the "tick" and gear mesh.
*   Input text to be encrypted by the stream.
*   Physical "Rotor" settings (initial rotation) as the Key.
