with open("experiments/chimera-lang/src/song_test.rs", "r") as f:
    lines = f.readlines()

new_lines = []
for line in lines:
    if "vm.step(); // tick organelle (sings \"Lux\", triggers host jump)" in line:
        new_lines.extend([
            "        vm.step(); // tick organelle (sings \"Lux\", triggers host jump)\n",
            "        vm.step(); // host processes jump\n"
        ])
    elif "for _ in 0..8 {" in line:
        new_lines.append("        for _ in 0..10 {\n")
    else:
        new_lines.append(line)

with open("experiments/chimera-lang/src/song_test.rs", "w") as f:
    f.writelines(new_lines)
