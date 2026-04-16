with open('experiments/chimera-lang/src/lisp.rs', 'r') as f:
    lines = f.readlines()

for i, line in enumerate(lines):
    if 's.starts_with(\'"\')' in line and 's.ends_with(\'"\')' in line:
        pass
    if 'let content = &s[1..s.len() - 1];' in line:
        lines[i] = line.replace('let content = &s[1..s.len() - 1];', 'let content = if s.len() >= 2 { &s[1..s.len() - 1] } else { "" };')

    if 's[1..s.len() - 1].to_string()' in line:
        lines[i] = line.replace('s[1..s.len() - 1].to_string()', 'if s.len() >= 2 { s[1..s.len() - 1].to_string() } else { "".to_string() }')

with open('experiments/chimera-lang/src/lisp.rs', 'w') as f:
    f.writelines(lines)
