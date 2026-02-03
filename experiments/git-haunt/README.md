# 👻 Git Haunt

> "Find the skeletons in your codebase closet."

**Git Haunt** is a TUI forensics tool that visualizes code churn and "pain points" in your repository. It scans your git history to identify "haunted" files—those that change frequently and are often associated with fix commits.

## 🔮 The Concept

Not all code is equal. Some files are stable bedrock, others are poltergeists that constantly break things. **Git Haunt** calculates a **Haunt Score** for every file based on:

1.  **Churn:** How many times the file has been touched in recent history.
2.  **Pain:** How often the file is involved in commits with messages like "fix", "bug", "panic", "error".

## 🎮 Controls

- **Up / Down / j / k:** Navigate the haunted file list.
- **q / Esc:** Exorcise the program (Quit).

## 📊 Visuals

- **Green:** Safe. The file is at peace.
- **Yellow:** Restless. The file shows signs of disturbance.
- **Red:** **HAUNTED.** This file is a source of constant pain. Tread carefully.

## 🚀 Usage

```bash
cargo run -p git-haunt
```

The tool will scan the last 1000 commits of the current repository and present the séance results.
