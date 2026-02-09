# Packet Pachinko 🛡️

A physics-based "Firewall Construction Kit" where you sort network packets using pinball mechanics.

## Concept

Packets fall from the internet (top). Your goal is to route them to the correct local port (bin) while blocking malware.
You do this by placing **Pins** on the board.

- **HTTP (Green)** -> Port 80 (Right Bin)
- **SSH (Blue)** -> Port 22 (Middle Bin)
- **Malware (Red)** -> DROP (Left Bin)

## Controls

- **Arrow Keys**: Move the cursor.
- **Space**: Place or Remove a pin.
- **Tab**: Switch Tool (Bumper vs Blocker).
- **Q / Esc**: Quit.

## Scoring

- **+10**: Correct Packet to Correct Bin.
- **+50**: Malware to DROP.
- **-10**: Wrong Port.
- **-100**: Malware LEAK (Malware hits Port 80/22).

## Tech Stack

- **ratatui**: TUI Rendering (Canvas widget with Braille markers).
- **Physics**: Custom Verlet integration with simple Circle-Circle collision.
- **Rust**: The language of safe concurrency (and safe pinball).
