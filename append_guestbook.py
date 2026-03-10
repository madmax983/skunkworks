import re

with open('GUESTBOOK.md', 'r') as f:
    content = f.read()

warden_high = """### [Concentration Level: CRITICAL MASS] - Location: Cargo.toml
- **Scent Origin:** Warden 🔒
- **Status:** `cargo audit` detected an unsound dependency (`macroquad`) and unmaintained dependencies (`paste`, `rusttype`). Immediate attention required to patch or replace these vulnerabilities.
"""

if '## 🍂 History/Decay' in content:
    idx = content.find('## 🍂 History/Decay')
    new_content = content[:idx] + warden_high + '\n' + content[idx:]
else:
    new_content = content + '\n' + warden_high

with open('GUESTBOOK.md', 'w') as f:
    f.write(new_content)
