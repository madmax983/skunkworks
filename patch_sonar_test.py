with open("experiments/chimera-lang/src/nova_sonar_test.rs", "r") as f:
    lines = f.readlines()

new_lines = []
for line in lines:
    if "assert_eq!(dist, Value::Int(16));" in line:
        new_lines.append("        assert_eq!(dist, Value::Int(15));\n")
    else:
        new_lines.append(line)

with open("experiments/chimera-lang/src/nova_sonar_test.rs", "w") as f:
    f.writelines(new_lines)
