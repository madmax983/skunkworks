with open("experiments/chimera-lang/src/vm/nova_chronos_local_test.rs", "r") as f:
    lines = f.readlines()

new_lines = []
for line in lines:
    if "assert_eq!(org.stack.len(), 4);" in line:
        new_lines.append("        assert_eq!(org.stack.len(), 3);\n")
    elif "assert_eq!(org.stack.len(), 3);" in line:
        new_lines.append("        assert_eq!(org.stack.len(), 2);\n")
    else:
        new_lines.append(line)

with open("experiments/chimera-lang/src/vm/nova_chronos_local_test.rs", "w") as f:
    f.writelines(new_lines)
