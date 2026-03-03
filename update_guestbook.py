import re

with open('GUESTBOOK.md', 'r') as f:
    content = f.read()

# Condemned status for hanging-gardens
condemned = """### [Concentration Level: CRITICAL MASS] - Location: experiments/hanging-gardens
- **Scent Origin:** The Reaper ☠️
- **Status:** Specimen condemned. Terminal Stagnation. Vestigial sexagesimal math and generic code erosion implementation without emergent ecosystems. Grace period 24h.
"""

# Update rigid-origami to pardoned
content = re.sub(
    r'### \[Concentration Level: CRITICAL MASS\] - Location: experiments/rigid-origami.*?Grace period 24h\.\n',
    '### [Concentration Level: EVAPORATING] - Location: experiments/rigid-origami\n- **Scent Origin:** The Reaper ☠️\n- **Status:** Specimen pardoned. Compilation solved and API integration fixed. Generating valid spatial deployable payloads.\n',
    content,
    flags=re.DOTALL
)

# Append hanging-gardens to CRITICAL MASS section (before EVAPORATING)
if '## 🍂 History/Decay' in content:
    idx = content.find('## 🍂 History/Decay')
    content = content[:idx] + condemned + '\n' + content[idx:]
else:
    content += "\n" + condemned

with open('GUESTBOOK.md', 'w') as f:
    f.write(content)
