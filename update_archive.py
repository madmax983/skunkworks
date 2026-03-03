import re

with open('ARCHIVE.md', 'r') as f:
    content = f.read()

# Pardon rigid-origami
content = re.sub(r'- \*\*rigid-origami\*\*: Compilation Failure.*?Grace period 24h\.\n', '', content)

# Move genetic-flock to executed
content = re.sub(r'- \*\*genetic-flock\*\*: Documentation Void.*?Grace period 24h\.\n', '', content)
content = content.replace('## ☠️ The Graveyard (Executed)', '## ☠️ The Graveyard (Executed)\n- **genetic-flock**: Specimen executed. Documentation Void. The DNA logic fails to manifest true emergent behaviors. Moved to graveyard.')

# Move ferrous-choreography to executed
content = re.sub(r'- \*\*ferrous-choreography\*\*: Documentation Void.*?Grace period 24h\.\n', '', content)
content = content.replace('## ☠️ The Graveyard (Executed)', '## ☠️ The Graveyard (Executed)\n- **ferrous-choreography**: Specimen executed. Documentation Void. The DNA is hardcoded. Moved to graveyard.')

# Condemn hanging-gardens
condemned_entry = "- **hanging-gardens**: Terminal Stagnation. Vestigial code and generic hybridization. Grace period 24h.\n"
content = content.replace('### Condemned (Grace Period: 24h)\n', f'### Condemned (Grace Period: 24h)\n{condemned_entry}')

with open('ARCHIVE.md', 'w') as f:
    f.write(content)
