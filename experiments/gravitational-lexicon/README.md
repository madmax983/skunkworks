# Gravitational Lexicon 🧬🗣️🌌

**A Semantic Universe where words are physical bodies composed of phonemes.**

## Concept

"In the beginning, there was the Word, and the Word had Mass."

This experiment visualizes a universe where:
- **Words** are massive objects attracted to each other by **Semantic Gravity**.
- **Phonemes** are particles connected by linguistic tension (springs).
- **Space-Time** (the background text) is distorted by the mass of the words via **Gravitational Lensing**.
- **Mutation** occurs when words are perturbed, following historical sound change laws (Grimm's Law, Vowel Shift).

## Lineage 🧬

This is a hybrid of:

### Parent A: `lensing-poetry` 🌌
- **Contribution**: N-Body gravitational simulation, Gravitational Lensing Shader, `macroquad` rendering infrastructure.
- **Essence**: "Mass bends meaning."

### Parent B: `biomorphic-lexicon` 🗣️
- **Contribution**: Phonological structure (Phonemes, Voice/Manner/Place), Spring-Mass word physics, Sound Change Rules.
- **Essence**: "Language is a physical, evolving system."

## Controls

- **Left Click**: Spawn a new random word at cursor position.
- **Space**: **Mutate** all words (apply sound laws) and apply a random force kick.
- **Enter**: Reset the universe.

## Implementation Details

- **Physics**: Words are composite bodies. Gravity acts on the Center of Mass of each word. Internal structure is maintained by springs between phonemes.
- **Rendering**: A custom GLSL shader calculates the gravitational deflection of light (background text) based on the positions and masses of the words.
- **Phonology**: Uses simplified linguistic rules (Grimm's Law, Vowel Shift) to evolve words over time.

## Status

Compiles. Verified.
