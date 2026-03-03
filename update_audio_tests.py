with open('crates/resonance-audio/src/audio.rs', 'r') as f:
    content = f.read()

# Fix the test placements
parts = content.split('#[test]\nfn test_audio_oscillator_zero_frequency')
base = parts[0]
new_tests = '#[test]\nfn test_audio_oscillator_zero_frequency' + parts[1]

# Find the closing brace of the mod tests
idx = base.rfind('}')
idx2 = base.rfind('}', 0, idx)
# Put new tests right before the last closing brace

fixed = base[:idx2+1] + '\n\n' + new_tests + '\n}'

with open('crates/resonance-audio/src/audio.rs', 'w') as f:
    f.write(fixed)
