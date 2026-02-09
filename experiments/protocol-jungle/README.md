# Protocol Jungle 🌴🦜

**"Me Tarzan, You Jane."**

Protocol Jungle simulates the emergence of a shared language (Pidgin) from a population of agents with random initial communication protocols.

## The Concept

In linguistics, a **Pidgin** is a simplified language that develops between two or more groups that do not have a language in common. It is not the native language of any community, but is learned as a second language.

In this simulation:
- **Agents** start with random internal protocols mapping Meanings (Greetings, Ack) to Symbols (Colors/Numbers).
- When agents meet, they attempt a Handshake:
  1. Initiator sends a `Greetings` symbol.
  2. Receiver tries to interpret it.
  3. If understood, Receiver sends `Ack`.
  4. If Initiator understands `Ack`, the handshake is successful.
- **Failure & Learning**: If a handshake fails, the confused party "learns" the symbol they just heard, mapping it to the expected context. (e.g., "I expected a Greeting, you said 'Baka', so 'Baka' must mean Greeting").
- **Visuals**:
  - Agents are circles.
  - **Outer Ring**: Color of the `Greetings` symbol.
  - **Inner Ring**: Color of the `Ack` symbol.
  - **Lines**: Green = Successful interaction, Red = Failed interaction.

## Running

```bash
cargo run -p protocol-jungle
```

## Observations
- Initially, the jungle is a chaos of multi-colored rings (many dialects).
- As agents interact, "Dominant Dialects" emerge locally.
- Eventually, large clusters of agents share the same colors, indicating a shared protocol.
