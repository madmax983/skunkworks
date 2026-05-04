# Semantic Spy 🕵️‍♂️

**Semantic Spy** is a Terminal User Interface (TUI) tool for visualizing **Semantic Bridge** snapshots.

It allows you to inspect the internal state of applications that implement the `tui-semantic` protocol, viewing entities, properties, and metrics in a structured way.

## Features

- **Entity List:** Browse all semantic entities in the snapshot.
- **Visualizer:** See a 2D representation of entities on a canvas.
- **Details Pane:** Inspect raw JSON properties of selected entities.
- **TUI Navigation:** Use keyboard shortcuts to navigate.

## Usage

`semantic-spy` reads a JSON snapshot from either a file or `stdin`.

### 1. Read from a file

```bash
cargo run --bin semantic-spy snapshot.json
```

### 2. Pipe from another application

If you have an application that outputs a JSON snapshot (e.g., `orbital-decay` with the `--semantic` flag), you can pipe it directly into `semantic-spy`.

```bash
cargo run --bin orbital-decay -- --semantic | cargo run --bin semantic-spy
```

*(Note: Ensure the source application outputs **only** the JSON snapshot to stdout).*

## Controls

| Key | Action |
| --- | --- |
| `↑` / `↓` | Select previous/next entity in the list |
| `q` | Quit the application |

## Input Format

`semantic-spy` expects a JSON object matching the `tui_semantic::Snapshot` structure:

```json
{
  "app": "my-game",
  "viewport": [80, 40],
  "entities": [
    {
      "kind": "player",
      "id": "p1",
      "position": { "x": 10.0, "y": 20.0 },
      "display": "@",
      "props": {
        "health": 100
      }
    }
  ]
}
```
