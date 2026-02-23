# Prologue: Rune Logic 🔮

Prologue is a Grid-based Visual Logic Language embedded within Chimera.
It allows for the construction of "Digital Circuits" and "Logic Agents" directly on the memory grid.

## Activation

Toggle Prologue mode with `[` in the TUI or the `OpCode::Prologue`.

## The Grid Circuit

Prologue treats the grid as a circuit board. Signals propagate instantly (within one tick) through connected components.

### Runes

| Rune | Name | Function |
|---|---|---|
| `!` | **Source** | **Reads** value from **West**, **Emits** to **Self**. |
| `?` | **Sink** | **Reads** signal from its **South** (or Self). Triggers log or **Gene Execution**. |
| `~` | **Wire** | Conducts signals in all cardinal directions (N, S, E, W). |
| `&` | **AND Gate** | Output (South/Self) = Input (West) **AND** Input (East). |
| `|` | **OR Gate** | Output (South/Self) = Input (West) **OR** Input (East). |
| `+` | **XOR Gate** | Output (South/Self) = Input (West) **XOR** Input (East). |
| `*` | **Splitter** | Input (North) -> Output (Self). Acts as a diode/repeater allowing W/E/S to read. |
| `#` | **Delay** | Input (North) -> Output (Self) in the **Next Tick**. |
| `@` | **Agent** | A mobile logic cursor that moves towards signals. |
| `$` | **Scribe** | Reads signal from **West**, writes Value to Grid **South**. |
| `%` | **Modulo** | Output (Self) = Input (West) **%** Input (East). |
| `^` | **Jump** | Input (West) -> Output (East). Skips self (Teleport). |
| `M` | **Mutate** | Reads signal from **West**, writes **Random** Value to Grid **South**. |
| `O` | **Organelle** | Reads signal from **West**, spawns **Agent (@)** at Grid **South**. |
| `[` | **Collect** | Reads inputs from **N, E, S, W**. Creates a `Value::Junction` list. Output **Self**. |
| `]` | **Scatter** | Reads **West** (List). Distributes elements: `[0]->N`, `[1]->E`, `[2]->S`. |
| `U` | **Unwrap** | Reads **West** (List). Outputs `Head` (Element 0) to **Self**. |
| `V` | **Vector** | Reads **West** (List). Outputs `Tail` (Elements 1..) to **Self**. |
| `F` | **Filter** | Reads **West** (List), **North** (Mask). Outputs filtered list to **Self**. |
| `T` | **Take** | Reads **West** (List), **North** (Count). Outputs first `N` elements to **Self**. |
| `\` | **Mirror Back** | Reflects signal 90° (N<->E, S<->W). |
| `/` | **Mirror Fwd** | Reflects signal 90° (N<->W, S<->E). |
| `-` | **Beam H** | Conducts signal horizontally (W<->E), blocks vertical. |
| `B` | **Blueprint** | Reads **West** (Trig), **North** (H), **South** (W). Captures Grid East. |
| `Π` | **Prototyper**| Reads **West** (Blueprint). Pastes to Grid East. |
| `Φ` | **Photon** | Reads **West** (Intensity) and **N/E/S** (Color). Emits Light. |
| `Λ` | **Sensor** | Reads Light Intensity at **Self**. Output to **Self**. |
| `Ω` | **Absorb** | Reads **West** (Intensity). Absorbs Light at **Self**. |
| `†` | **Bury** | Reads **West** (Strand Index). Buries that strand (Moves to Graveyard). |
| `‡` | **Exhume** | Reads **West** (Trigger). Restores last buried strand to Helix. |
| `Ψ` | **Seance** | Reads **West** (Trigger). Executes last buried strand as a Ghost. |
| `c` | **Prophecy** | Reads **West** (Strand). Simulates 100 ticks. Writes **1** (Death) or **0** (Life) to **South**. |
| `8` | **Entangle** | Reads **West** (Strand A) and **East** (Strand B). Quantum Entangles them. |

### Gene Execution

If a **Sink** (`?`) receives a signal that is a String matching a named Strand in the DNA (e.g. `"my_function"`), it will interrupt the VM and execute that Strand immediately. This allows the logic grid to control the genetic execution flow.

### Signal Theory

*   **Values**: Signals carry the full `Value` type (Int, String, etc.). `Int(0)` and `Str("")` are considered Empty (No Signal).
*   **Propagation**: Signals travel through `~` wires instantly within a tick.
*   **Conflict**: If multiple signals meet on a wire, the behavior is "Last Write Wins".
*   **Logic**:
    *   `AND`: Requires both West and East inputs to be Truthy. Output is 1.
    *   `OR`: Requires at least one of West or East inputs to be Truthy. Output is 1.
    *   `XOR`: Requires exactly one of West or East inputs to be Truthy. Output is 1.
    *   `Delay`: Holds the signal for one tick before releasing it.

### Agents (`@`)

Agents are autonomous cursors.
*   They scan their local neighborhood.
*   They move towards adjacent cells containing signals or wires.

## Example

**Simple Circuit:**
```
  42      (Value 42)
  !       (Source reads 42)
  ~       (Wire carries 42)
  ~       (Wire carries 42)
  ?       (Sink receives 42 -> Logs "42")
```

**Logic Gate:**
```
  1   1
  !   !
  ~   ~
  &       (AND Gate)
  ?       (Sink receives 1)
```

**Gene Trigger:**
```
  "func"  (Strand Name)
  !
  #       (Delay 1 Tick)
  ?       (Sink receives "func" -> Executes Strand "func")
```

# The Interpreter (Forth Agent) ₣

A stack-based mobile interpreter that traverses the grid, executing runes as instructions.

| Rune | Name | Function |
|---|---|---|
| `₣` | **Interpreter** | Moves East (default). Reads grid cells. |

### Instruction Set

*   **Movement**: `N`, `S`, `E`, `W` set direction.
*   **Stack**: `"` (Dup), `_` (Drop), `\` (Swap).
*   **Math**: `+`, `-`, `*`, `/`, `%`.
*   **IO**: `!` (Pop -> Signal), `?` (Consume Signal -> Push), `.` (Pop -> Log).
*   **Literals**: Integers and Strings are pushed to the stack.

### Host Operations (Genetic Engineering)

The Forth Agent can directly interact with the Chimera VM's genetic code, allowing the grid to reprogram the organism.

*   `r`: **Read Gene**. Pops `strand`, `gene`. Pushes `op_string`, `arg`.
*   `w`: **Write Gene**. Pops `arg`, `op_string`, `gene`, `strand`. Writes to DNA.
*   `x`: **Execute**. Pops `strand`. Triggers immediate execution (Interrupt).
*   `n`: **New Strand**. Creates a new empty strand. Pushes `new_strand_idx`.
*   `l`: **Length**. Pops `strand` (-1 for Helix). Pushes length.

The Interpreter is **non-destructive**. It moves *over* other runes without erasing them, temporarily replacing them with itself. It blocks if it encounters another Agent.

# Zeta: Wire Lisp (ζ)

The **Zeta Agent** (`ζ`) brings the power of Lisp to the grid. Unlike the Forth agent which executes cells step-by-step, the Zeta Agent scans the connected wire network to read complete S-Expressions.

| Rune | Name | Function |
|---|---|---|
| `ζ` | **Zeta** | Moves East (default). Scans wires for S-Expressions. |

### Wire Reading

When the Zeta Agent activates, it scans in its facing direction (default East), following wires (`~`) to find tokens.

*   **Delimiters**: `( ... )`, `[ ... ]`, `{ ... }`.
*   **Atoms**: Integers, Strings, and OpCodes found on the path.
*   **Structure**: The sequence of tokens is reconstructed into a Lisp string and evaluated.

**Example:**
```
  ζ ~ ( ~ + ~ 1 ~ 2 ~ )
```
1.  **Scan**: `(`, `+`, `1`, `2`, `)`
2.  **Compile**: `( + 1 2 )` -> `[Push(1), Push(2), Add]`
3.  **Execute**: Pushes `3` to the Agent's stack.

This allows for complex, nested logic to be laid out spatially using wires.

# Data Alchemy ⚗️

Runes for transforming, combining, and inspecting raw data values (Integers, Strings, Lists).

| Rune | Name | Function |
|---|---|---|
| `t` | **Transmute** | Reads **West** (Value) and **North** (Mode). Converts type (0=Str, 1=Int, 2=Type, 3=Len). |
| `f` | **Fuse** | Reads **West** (A) and **East** (B). Combines them (Concat, Add, Push). |
| `d` | **Distill** | Reads **West** (Value). Splits into **North** (Head) and **South** (Tail). |

# Elemental Alchemy ⚗️

Introduces elemental forces to the grid.

| Rune | Name | Function |
|---|---|---|
| `Δ` | **Fire** | Emits **Fire** Element. |
| `∇` | **Water** | Emits **Water** Element. |
| `◊` | **Earth** | Emits **Earth** Element. |
| `○` | **Air** | Emits **Air** Element. |
| `☆` | **Aether** | Emits **Aether** (Spirit) Element. |
| `☿` | **Mercury** | **Mixes** adjacent elemental signals. |

**Reactions:**
*   Fire + Water -> **Steam**
*   Fire + Earth -> **Lava**
*   Water + Earth -> **Mud**
*   Fire + Air -> **Plasma**

# Void Alchemy 🕳️

Runes for interacting with the Void Buffer (Global LIFO Storage).

| Rune | Name | Function |
|---|---|---|
| `µ` | **Vacuum** | Detects Emptiness. **West** (Empty) -> **Self** (1). |
| `Ø` | **Void In** | **Consumes** West Signal -> Pushes to **Void Buffer**. |
| `§` | **Void Out** | **Pops** from **Void Buffer** -> Outputs to **Self**. |

# Psionics 🧠

Runes for remote action and telepathy.

| Rune | Name | Function |
|---|---|---|
| `Θ` | **Telepathy** | Reads **West** (Y) and **North** (X). Outputs value at `grid[Y][X]` to **Self**. |
| `Ξ` | **Telekinesis** | Reads **West** (Dir), **North** (Y), **East** (X). Moves value at `grid[Y][X]` in Direction. |
| `Σ` | **Suggestion** | Reads **West** (Val), **North** (Y), **East** (X). Writes `Val` to `grid[Y][X]`. |

# Resonance 🎵

Runes for audio synthesis and musical interaction.

| Rune | Name | Function |
|---|---|---|
| `♪` | **Note** | Reads **West** (Value) as MIDI Note. Plays Tone. |
| `♫` | **Chord** | Reads **West** (Root) and **North** (Type). Plays Chord. |
| `🥁` | **Drum** | Reads **West** (Trigger). Plays Percussive Sound. |

# Symbiotes 🧬

Advanced genetic manipulation and fusion runes.

| Rune | Name | Function |
|---|---|---|
| `p` | **Parasite** | Reads **West** (Gene/Strand). Injects it into **East** Agent. |
| `o` | **Osmosis** | Reads **West** and **East** Agents. Swaps Resources/Genes. |
| `x` | **Xenograft**| Swaps **West** and **East** Agents. Triggers Mutation. |

# Linguistics 🗣️

Runes for string manipulation and text processing.

| Rune | Name | Function |
|---|---|---|
| `"` | **Stringify** | Reads **West** (Value). Converts to String. Outputs to **Self**. |
| `®` | **Regex** | Reads **West** (Target) and **North** (Pattern). Outputs **1** (Match) or **0** (No Match) to **Self**. |
| `;` | **Split** | Reads **West** (String) and **North** (Delimiter). Outputs List (Junction) to **Self**. |
| `©` | **Join** | Reads **West** (List) and **North** (Delimiter). Outputs String to **Self**. |

# Siren: Genomic Music 🧜‍♀️

A musical agent that treats the grid as a sequencer.
The Siren (`♬`) moves constantly in a direction, playing notes and executing musical commands found in the cells it traverses.

| Command | Function |
|---|---|
| `A`..`G` | **Play Note** (Major). |
| `a`..`g` | **Play Note** (Sharp `#`). |
| `0`..`9` | **Set Octave** (Relative to base, 5=Default). |
| `^` / `v` | **Octave Shift** Up / Down. |
| `>` / `<` | **Velocity** Increase / Decrease. |
| `!` | **Accent** (Max Velocity). |
| `~` | **Rest** (Sustain). |
| `h/j/k/l` | **Change Direction** (Left, Down, Up, Right). |
| `N/S/W` | **Explicit Direction** (North, South, West). |
| `*` | **Random Direction**. |
| `w` | **Cycle Waveform** (Sine, Square, Saw, Tri, Noise). |

Example:
```
  ♬ -> A -> B -> C -> N
                      |
                      v
                      G
                      |
                      v
```

# Neural Logic 🧠

Runes for simulating neural networks and learning systems directly on the grid.

| Rune | Name | Function |
|---|---|---|
| `♦` | **Neuron** | Accumulates signals. Fires when threshold reached. |
| `•` | **Synapse** | Weighted connection. Reads **West**, Applies Weight, Emits **East**. |
| `°` | **Learning**| Hebbian Teacher. Reads **West**. Increases weight of adjacent Synapses. |

# Elektra: Circuit Logic ⚡

Runes for interfacing with the Elektra circuit simulator.

| Rune | Name | Function |
|---|---|---|
| `⚡` | **Bolt** | **Source**. Sets high voltage if powered. |
| `≡` | **Ground** | **Sink**. Sets ground (0V). |
| `∿` | **Sine** | **Sensor**. Reads local voltage -> Signal South. |
| `🔌` | **Gen** | **Generator**. Consumes Energy -> Voltage. |
| `💡` | **Lamp** | **Load**. Consumes Voltage -> Energy. |
| `🔋` | **Capacitor**| Sets local capacitance. |
| `♒` | **Memristor**| Variable Resistance based on flux. |
| `⇝` | **Varistor** | **Variable Resistor**. Reads **West** (Signal) -> Sets Resistance (Inv). |

# Prologue II: Signal Grid (Orca Mode) 🐋

Enabled via `OpCode::Orca` or TUI toggle. This is a concurrent cellular automata system inspired by Orca.

### New Operators

| Operator | Name | Function |
|---|---|---|
| `J` | **Jumper** | Moves value from **West** to **East**, skipping Self. |
| `(` | **Warp** | Swaps the values of **North** and **South** neighbors. |
| `*` | **Bang** | Fires a signal to all neighbors if it receives one. |
| `N/S/E/W` | **Directional** | Moves values in cardinal directions. |
| `A/B/D` | **Math** | Add, Subtract, Divide. |
| `C` | **Clock** | Outputs time-based modulo values. |
| `M` | **Mutate** | Mutates target strand or value. |
| `Q` | **Query** | Reads value at offset. |

# Hyper-Geometry ▣

Runes for 4D Hypercube manipulation and Hyperspace traversal.

| Rune | Name | Function |
|---|---|---|
| `▣` | **Tesseract** | **Source**. Emits current **W** coordinate to **Self**. |
| `⇪` | **Ascend** | Reads **West** (Delta). Moves agent at **South** in W-axis (Hyperspace). |
| `↻` | **Rotate** | Reads **West** (Angle). Rotates the Hypercube projection. |
| `⌖` | **Project** | Reads **West** (Mode). Changes projection method (Perspective, Ortho, Slice). |

**Hyperspace Theory:**
*   The grid exists at W=0 by default.
*   Agents can move to W!=0 using `⇪`.
*   At W!=0, agents are "Ghosts" (visible in Tesseract view, but may interact differently).
*   Coordinates are (Y, X, Z, W).

# Prism Language (Spectral Logic) 🌈

The Prism Language adds a layer of color-based semantics to the grid.
Standard runes behave differently depending on the **Color** of the cell they occupy.

This allows for dense, multi-modal logic where the same circuit topology can perform different functions based on its spectral state.

## Activation

Colors can be painted onto the grid using the `🖌` (Brush) rune or emitted by bioluminescent sources.

## Spectral Semantics

When a standard rune is colored, its function is modulated by the dominant color channel (R, G, B).

### Red (Energy / Force) 🔴
*   `+` **Amplified Add**: Multiplies the sum of inputs by 2. `(A + B) * 2`.
*   `*` **Explosive Split**: Propagates the input signal to **All Cardinal Neighbors** (N, S, E, W), overriding normal directionality.

### Green (Life / Growth) 🟢
*   `+` **Genetic Crossover**: Splices two string inputs. Takes the first half of West and the second half of East. `Head(A) + Tail(B)`.
*   `*` **Spore**: (Experimental) Spawns an agent/spore based on input.

### Blue (Logic / Time) 🔵
*   `+` **Logical AND**: Performs a boolean AND on integer inputs. Output is 1 only if both inputs are non-zero.
*   `*` **Time Dilation**: Acts as a logical NOT/Inverter on the signal existence. (If signal present -> 0, else -> 1).

## Runes

| Rune | Name | Function |
|---|---|---|
| `🎨` | **Palette** | Mixes R(West), G(North), B(East) signals into a Color Value. |
| `🖌` | **Brush** | Reads Color (West) and Paints the Grid Cell to the South. |
| `👁` | **Eye** | Reads the Color of the current cell and emits it as a signal. |
| `🔴` | **Extract R** | Extracts Red channel from West signal. |
| `🟢` | **Extract G** | Extracts Green channel from West signal. |
| `🔵` | **Extract B** | Extracts Blue channel from West signal. |

# Plasmid: The Genetic Vector (P) 🧬

A mobile agent dedicated to **Horizontal Gene Transfer**.

| Rune | Name | Function |
|---|---|---|
| `P` | **Plasmid** | Moves randomly. Absorbs signals from `!` sources. Injects payloads into other agents. |

## Behavior
*   **Absorption**: `P` adjacent to `!` (West) absorbs the signal.
*   **Conjugation**: `P` adjacent to another agent injects the payload.
    *   **Critters (`C`)**: Appends payload as a Gene.
    *   **Others**: Overwrites state.

# Evolutionary Runes 🧬

| Rune | Name | Function |
|---|---|---|
| `Ð` | **Reverse Transcriptase** | Reads **West** (String Signal). Compiles it into a **Gene** and appends it to the current DNA strand. |

# Mycelium 🍄

A distributed fungal network that shares a global buffer and grows organically.

| Rune | Name | Function |
|---|---|---|
| `🍄` | **Spore** | **Growth Node**. Consumes Nutrients (`Int > 10`) to spawn new Spores. Accesses Global Mycelium Buffer. |
| `📥` | **Inject** | **Wire**. Propagates signal. Sinks into Buffer. |
| `📤` | **Extract** | **Wire**. Reads from Buffer. Emits signal. |
| `🦋` | **Metamorph**| **Sink**. Consumes buffer item to mutate agent at location. |

## Mechanics

*   **Global Buffer**: All Spores share a single FIFO queue (`VecDeque`).
*   **Nutrient Growth**: If a Spore is adjacent to a grid cell with `Int(n)` where `n >= 10`, it consumes 10 units and spawns a new Spore in an adjacent empty cell.
*   **Spore Dispersal**: If the buffer contains >= 10 items, there is a 1% chance per tick for a Spore to consume 5 items and teleport-spawn a new Spore anywhere on the grid.
