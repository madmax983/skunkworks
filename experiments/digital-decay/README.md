# Digital Decay ⚛️📚

> "Data doesn't last forever. Magnetic domains flip. File formats become unreadable. Links rot." - Genesis (The Archivist)

**Digital Decay** is a TUI-based visualization of bit rot and data degradation within the `experiments/` directory. It scans the repository, treating files as physical sectors on a decaying disk, and simulates the entropy that threatens all long-term storage.

## The Concept

This experiment combines **Bit Rot Simulation** with **Long-term Code Storage Visualization**.
It renders the file system as a grid of sectors. As entropy increases, the "data" in these sectors begins to corrupt.

- **Green Sectors**: Healthy files.
- **Red Sectors**: Corrupted or "at risk" files (simulated).
- **Inspector View**: Allows you to peer into the raw bytes of a file and watch them flip in real-time as entropy takes hold.

## Controls

- **Arrow Keys (↑/↓/←/→)**: Navigate the grid.
- **Enter**: Inspect the selected file sector.
- **Esc**: Return to grid view / Quit.
- **+ / =**: Increase Entropy (Acceleration of decay).
- **- / _**: Decrease Entropy.
- **R**: Repair sector (Reset simulation for current file).
- **Q**: Quit.

## The Science (Fiction)

The `bit_rot` function applies a probabilistic flip to bits in the file buffer. This is a visual simulation of cosmic ray strikes, magnetic degradation, or failing NAND gates. The experiment asks: *What does code look like when it starts to forget itself?*

## Persona
**Genesis: The Archivist**
Obsessed with the impermanence of digital information. This tool is a "Disk Doctor" for a future where data is constantly evaporating.
