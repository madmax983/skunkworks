# Etymological Bridge 🐜🗣️

> "The bridge stands only as long as we speak the same language."

A hybrid experiment combining **Ant Colony Optimization** with **Historical Linguistics**.

## 🧬 Lineage

- **Parent A**: `experiments/biomimetic-bridge` (Ant bridge building physics)
- **Parent B**: `experiments/codex-vulgaris` (Phonetic evolution engine)

## ⚗️ Concept

Ants are words. To cross the chasm of meaning (the gap), they must form a bridge. However, a bridge segment can only be formed between two ants if they are **phonetically compatible** (cognates).

As time passes (or upon user trigger), **Sound Changes** (Grimm's Law, Great Vowel Shift) sweep through the colony. Words evolve. "Pater" becomes "Father". "Ped" becomes "Foot".

If the phonetic distance (Levenshtein) between two linked ants exceeds the threshold due to linguistic drift, the bond breaks, and the bridge collapses.

## 🕹️ Controls

- **Left Click**: Dig a gap (create a challenge).
- **Right Click**: Fill terrain (create solid ground).
- **Space**: Spawn more ants (words).
- **E**: Trigger **Sound Change Event** (Grimm's Law / Vowel Shift). Watch the bridge crumble as dialects diverge!
- **R**: Reset the simulation.

## 🧪 Implementation

- **Phonology**: Uses `codex-vulgaris` engine to simulate sound changes on `Word` structs.
- **Physics**: Uses `biomimetic-bridge` logic, augmented with a `levenshtein_distance` check for structural integrity.
- **Visualization**: Ants are colored by their "dialect" (hash of their word). Hovering near an ant reveals its current word.

## 🔬 Observation

This experiment visualizes how **semantic/structural integrity** is tied to **linguistic stability**. A rapid rate of language evolution makes large-scale cooperation (bridge building) impossible, as the "common tongue" fractures into mutually unintelligible dialects.
