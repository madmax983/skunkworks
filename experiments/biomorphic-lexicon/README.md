# Biomorphic Lexicon 🧬🗣️

> "In the beginning was the Word, and the Word was with God, and the Word was God."

**Biomorphic Lexicon** is a hybrid experiment visualizing words as physical strings that evolve phonetically when agitated.

## Concept

What if language was a physical object?
In this simulation, words are "strings" made of phonemes. Each phoneme has mass and physical properties based on its linguistic features (Vowels are light, Consonants are heavy).

When the string vibrates (due to high kinetic energy), the phonemes become unstable and mutate according to historical sound change laws (Grimm's Law, Vowel Shifts).

## Lineage

- **Parent A:** `biomorphic-strings` (TUI Physics)
  - Provided the mass-spring physics engine and TUI rendering structure.
- **Parent B:** `glossolalia` (Phonological Evolution)
  - Provided the `Phoneme`, `Word`, and `Rule` traits, along with `GrimmsLaw` and `VowelShift` logic.

## Controls

- **Space**: Agitate the string (pluck random nodes). High energy triggers mutation.
- **Enter**: Reset to the original Word.
- **Q**: Quit.

## Emergent Behavior

- **Kinetic Drift**: You can "shake" the word into a new dialect.
- **Physical Philology**: Heavier consonants anchor the string, while vowels flutter more easily.
