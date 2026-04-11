# 🎼 Quipu Symphony

**A Polyphonic Data Sequencer based on Incan Khipu.**

> "The knots are not just data; they are notes in a history we are learning to play." - The Splice Surgeon

## 🧬 Lineage
- **Parent A**: `experiments/quipu-renderer` (The Archaeologist)
  - *Inheritance*: Data structure (`Cord`, `Knot`), physical logic of positional notation.
- **Parent B**: `experiments/thread-phase` (The Mycelium/Genesis)
  - *Inheritance*: Audio engine (`cpal`, `SoundKind`), main loop timing logic.
- **Hybrid Trait**: The vertical structure of the Quipu is treated as a musical score. Playhead moves down due to "gravity", triggering sounds based on knot types and positions.

## 🕹️ Controls
- **Space**: Pause / Resume playback.
- **R**: Regenerate random Quipu cords (Compose new song).
- **Up / Down**: Adjust playback speed (Gravity).
- **Q**: Quit.

## 🎵 How it Works
- **Vertical Axis**: Time (Gravity).
- **Horizontal Axis**: Tracks (Cords).
- **Red Cord**: Rhythm Track. Simple knots = Kick, Long knots = Snare, Figure-8 = HiHat.
- **Blue Cord**: Bass Track. Knot values determine pitch.
- **Yellow Cord**: Lead Track. Knot values determine pitch.

## 🏗️ Architecture
The system uses `ratatui` for visualization and `cpal` for real-time synthesis. It demonstrates how ancient data structures can be re-contextualized as modern multimedia sequences.
