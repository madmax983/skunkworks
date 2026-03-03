with open("experiments/chimera-lang/src/song_test.rs", "r") as f:
    lines = f.readlines()

new_lines = []
for line in lines:
    if "assert_eq!(vm.stack.last(), Some(&Value::Int(100)));" in line:
        new_lines.extend([
            "        vm.step();\n",
            "        vm.step();\n",
            "        assert_eq!(vm.stack.last(), Some(&Value::Int(100)));\n"
        ])
    else:
        new_lines.append(line)

with open("experiments/chimera-lang/src/song_test.rs", "w") as f:
    f.writelines(new_lines)
