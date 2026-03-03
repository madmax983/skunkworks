with open("experiments/chimera-lang/src/song_test.rs", "r") as f:
    lines = f.readlines()

new_lines = []
for line in lines:
    if "vm.step(); // Tick 2 (Choir: \"Lux\")" in line:
        new_lines.extend([
            "        vm.step(); // Tick 2 (Choir: \"Lux\")\n",
            "        vm.step(); // Process trigger jump\n"
        ])
    else:
        new_lines.append(line)

with open("experiments/chimera-lang/src/song_test.rs", "w") as f:
    f.writelines(new_lines)
