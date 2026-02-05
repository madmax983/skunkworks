# 🧶 Quipu Weaver

> "Data is not just stored; it is tied." - Genesis (The Archaeologist)

**Quipu Weaver** is a serialization system that encodes modern data structures into the ancient Incan **Quipu** format. It rejects the tyranny of the decimal system and the screen-based text buffer, replacing them with knotted cords and spatial relationships.

## 🏺 Excavation

The **Quipu** (or Khipu) was the recording device of the Inca Empire. Lacking a written script, the Incas used a positional system of knots on hanging strings to record census data, taxes, and possibly even narratives.

- **Structure**: A primary horizontal cord with pendant cords hanging from it. Subsidiary cords could hang from pendant cords.
- **Number System**: Base-10 positional system.
  - **Zero**: Represented by an empty space.
  - **Units**: A "Figure-Eight" knot (1) or a "Long Knot" (2-9).
  - **Tens/Hundreds**: Single overhand knots.
- **Colors**: Used to denote categories (Red = War/Government, Yellow = Gold/Corn, etc.).

## ⚗️ Translation

We have mapped Rust's type system to the Quipu structure:

- **Structs**: A Cord where fields are subsidiary Cords.
- **Integers**: Encoded strictly in the Inca Base-10 knot system.
- **Strings**: Encoded as sequences of byte-values (treated as numbers).
- **Vectors**: A sequence of pendant cords.

This is not just a visualization; it is a `serde::Serializer`. Any serializable Rust data structure can be converted into a `Quipu`.

## 🕹️ Usage

```bash
cargo run -p quipu-weaver -- --input data.json
```

If no input is provided, a demo dataset is woven.

### Controls

- **Arrow Keys**: Pan the view (Scroll the primary cord).
- **Q**: Quit.

## 🔮 Moonshot Features

- **TUI Rendering**: A custom `ratatui` widget that renders the physical structure of the knots.
- **No Arabic Numerals**: Values are read by counting knots (hovering or "inspecting" is required to translate, but the primary view is authentic).

## ⚠️ Warning

Do not attempt to untie the knots while the program is running. Segfaults may occur if the primary cord is severed.
