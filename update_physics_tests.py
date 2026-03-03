with open('crates/resonance-audio/src/physics.rs', 'r') as f:
    content = f.read()

parts = content.split('#[test]\nfn test_pluck_zero_width() {')
base = parts[0]
new_tests = '#[test]\nfn test_pluck_zero_width() {' + parts[1]

# Find the closing brace of the mod tests
idx = base.rfind('}')
idx2 = base.rfind('}', 0, idx)

fixed = base[:idx2+1] + '\n\n' + new_tests + '\n}\n'

with open('crates/resonance-audio/src/physics.rs', 'w') as f:
    f.write(fixed)
