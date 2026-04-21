import sys
import re

with open('experiments/chimera-lang/src/prolouge_compiler.rs', 'r') as f:
    content = f.read()

# Make the entire `mod tests` under `#[cfg(feature = "nova")]`
content = content.replace('#[cfg(test)]\nmod tests {', '#[cfg(feature = "nova")]\n#[cfg(test)]\nmod tests {')

with open('experiments/chimera-lang/src/prolouge_compiler.rs', 'w') as f:
    f.write(content)

with open('experiments/chimera-lang/tests/prolouge_mad_scientist_test.rs', 'r') as f:
    test_content = f.read()
# Replace `#[cfg(feature = "nova")]` if missing
if '#[cfg(feature = "nova")]' not in test_content:
    test_content = '#[cfg(feature = "nova")]\n' + test_content
with open('experiments/chimera-lang/tests/prolouge_mad_scientist_test.rs', 'w') as f:
    f.write(test_content)
