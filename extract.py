with open("crates/git-associates/src/lib.rs", "r") as f:
    lines = f.readlines()

new_lines = []
for line in lines:
    if "for i in 0..diff.deltas().len() {" in line:
        new_lines.append(line.replace("for i in 0..diff.deltas().len() {", "for i in 0..diff.deltas().len() {\n            // extracted here"))
    else:
        new_lines.append(line)
