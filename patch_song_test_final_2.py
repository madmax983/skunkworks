with open("experiments/chimera-lang/src/song_test.rs", "r") as f:
    lines = f.readlines()

new_lines = []
for line in lines:
    if "assert_eq!(vm.ip.0, 1);" in line:
        new_lines.extend([
            "        vm.step(); // Extra tick to allow triggered host jump\n",
            "        assert_eq!(vm.ip.0, 1);\n"
        ])
    else:
        new_lines.append(line)

with open("experiments/chimera-lang/src/song_test.rs", "w") as f:
    f.writelines(new_lines)
