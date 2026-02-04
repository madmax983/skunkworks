# Hidden Brush 🐢🎨

> "The best encryption is invisible."

**Hidden Brush** is a steganographic virtual machine. It allows you to hide "Turtle Graphics" programs inside innocent-looking images. When executed, the hidden program takes control of the brush and draws a new image or animation.

## Concept

We combine **LSB Steganography** with **Code Distribution**.
Instead of hiding a static message, we hide a *program*.
The image is the cartridge. The tool is the console.

## Usage

### Encode
Hide a script in a cover image (or generate noise):
```bash
cargo run -- encode --script demo.asm --output secret.png
```

### Run
Execute the hidden program:
```bash
cargo run -- run --image secret.png
```

## Bytecode
The VM supports a simple Turtle instruction set:
- `FWD`: Move forward
- `ROT <deg>`: Rotate
- `PEN <0|1>`: Pen up/down
- `COL <r> <g> <b>`: Set color
- `REP <count> <len>`: Repeat next `len` instructions `count` times.
- `SET_STEP <val>`: Set step size.
- `ADD_STEP <val>`: Modify step size (useful for spirals).

## Example Script
```asm
COL 0 255 255
SET_STEP 2
REP 100 3
FWD
ROT 89
ADD_STEP 1
END
```
