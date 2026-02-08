# 019. Chimera Chemistry System

## Status
Proposed

## Context
Chimera organisms exist in a grid-based world where interaction has historically been limited to movement, consumption (eating value), and reproduction. While "Signals" allowed for communication, there was no mechanism for organisms to manipulate the material composition of their environment in complex ways. Users and developers wanted a way for organisms to "craft" tools or weapons and for the environment to have more reactive properties (e.g., acids, buffs, mutagenic pools) beyond simple energy values.

## Decision
Implement a **Chemistry System** within the Nova feature set (`src/vm/nova_chemistry.rs`) that introduces the concept of mixing ingredients into solutions.

### Key Components:
1.  **JunctionType::Dish:** A new variant for the `Junction` value type that acts as a container for a list of ingredients (other Values).
2.  **OpCode::Mix(radius):** Gathers non-empty values from the surrounding area into a `Dish` at the center.
3.  **OpCode::Brew(heat):** Transforms the contents of a `Dish` based on a recipe lookup table.
    *   *Recipes:*
        *   Water + Fire + Heat(10) -> Acid
        *   Life + Energy + Heat(5) -> Elixir
        *   Chaos + Entropy -> Mutagen
        *   Earth + Water -> Mud
4.  **OpCode::Splash(radius, dy, dx):** Throws the contents of a `Dish` (if it contains a valid "Solution") at a target location, applying an area-of-effect impact.

## Consequences
*   **Positive:**
    *   **Emergent Gameplay:** Organisms can now engage in chemical warfare (Acid), healing (Elixir), or forced mutation (Mutagen).
    *   **Resource Economy:** Introduces a "crafting" layer where raw values (Strings/Ints) are commodities to be gathered and processed.
    *   **Environmental storytelling:** "Puddles" of chemicals can persist in the world, creating hazardous or beneficial zones.
*   **Negative:**
    *   **Grid Complexity:** Managing `Vec<Value>` inside a grid cell (`JunctionType::Dish`) is more memory-intensive than simple `Int` or `Char` values.
    *   **Performance:** `Mix` and `Splash` involve iterating over area coordinates, which could be costly if abused by many organisms simultaneously.
