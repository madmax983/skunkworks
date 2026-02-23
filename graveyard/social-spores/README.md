# Social Spores 🍄📢

**"Spore dispersal patterns + Information propagation in social networks"**

## Concept
A visualization of viral information spread using fungal metaphors.
- **Nodes (Mushrooms)**: Social media influencers. They have an Opinion (Color) and Influence (Size).
- **Spores**: Information packets (Tweets/Posts). They drift on the "Wind of Discourse".
- **Wind**: The zeitgeist, pushing information in shifting directions.
- **Infection**: When a spore lands on a node:
    - **Echo Chamber**: If colors match, the node grows larger (Influence increases).
    - **Persuasion**: If colors differ, the node's opinion shifts slightly.
- **Viral Burst**: When a node accumulates critical influence, it explodes, releasing hundreds of spores in a massive wave.

## Simulation
- **Physics**: Particle system driven by a Perlin-like noise vector field.
- **Biology**: Nodes pulse and emit spores based on their energy.
- **Sociology**: Opinion dynamics modeled as color blending and influence accumulation.

## Controls
- **Click**: Spawn a new Influencer Node (Random Color).
- **Space**: Shift the Wind Direction (Time Jump).
- **R**: Reset the simulation.

## Tech Stack
- **Rust**: Core logic.
- **Macroquad**: 2D Visualization.
