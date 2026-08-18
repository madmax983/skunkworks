import sys

with open("experiments/lattice-brain/src/audio.rs", "r") as f:
    lines = f.readlines()

for i, line in enumerate(lines):
    if "use crossbeam_channel::{unbounded, Sender};" in line:
        lines[i] = "use crossbeam_channel::{unbounded, Sender, Receiver};\n"
    if "pub enum AudioCommand {" in line:
        lines[i] = "#[allow(dead_code)]\npub enum AudioCommand {\n"
    if "let (cmd_tx, cmd_rx) = unbounded::<AudioCommand>();" in line:
        lines[i] = "        let (cmd_tx, _cmd_rx) = unbounded::<AudioCommand>();\n"

with open("experiments/lattice-brain/src/audio.rs", "w") as f:
    f.writelines(lines)
