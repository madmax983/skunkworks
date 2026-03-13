with open("experiments/chimera-lang/src/tui/app/handlers/editing/backspace.rs", "r") as f:
    lines = f.read().splitlines()

new_lines = []
for line in lines:
    if line.strip() == "_ => {}":
        continue
    new_lines.append(line)

with open("experiments/chimera-lang/src/tui/app/handlers/editing/backspace.rs", "w") as f:
    f.write("\n".join(new_lines) + "\n")
