with open('ARCHIVE.md', 'r') as f:
    content = f.read()

# Update executed code-bio-dome
content = content.replace(
    "- **code-bio-dome**: Specimen pardoned",
    "- **code-bio-dome**: Specimen executed. Diagnosis: Terminal Compilation Failure / Ecosystem Maladaptation. Failed to resolve import `tui_shared::semantic`. Grace period expired. Moved to graveyard."
)

# Move from Pardoned to Executed
parts = content.split('## Pardoned')
top = parts[0]
bottom = parts[1]

# Need to extract the code-bio-dome line and move it
lines = bottom.split('\n')
executed_line = ""
new_bottom = []
for line in lines:
    if line.startswith('- **code-bio-dome**:'):
        executed_line = line
    else:
        new_bottom.append(line)

content = top + '## ☠️ Executed\n' + executed_line + '\n' + '\n'.join(new_bottom)
content = content.replace('## ☠️ Executed\n## ☠️ Executed', '## ☠️ Executed')
with open('ARCHIVE.md', 'w') as f:
    f.write(content)
