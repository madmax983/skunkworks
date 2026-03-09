with open('crates/locus/src/vec4.rs', 'r') as f:
    lines = f.readlines()

# extract the 3 tests and remove them from test_project_to_3d_clamping
import re

start1 = 860 # #[test]
end1 = 907

test_code = lines[start1:end1+1]

# Delete them from lines
del lines[start1:end1+1]

# Now find the end of the file. It should end with `}\n` which is the `mod tests` block.
# We want to insert `test_code` right before the last `}`.

last_brace_idx = -1
for i in range(len(lines)-1, -1, -1):
    if "}" in lines[i]:
        last_brace_idx = i
        break

lines.insert(last_brace_idx, "".join(test_code))

with open('crates/locus/src/vec4.rs', 'w') as f:
    f.writelines(lines)
