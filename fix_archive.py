with open('ARCHIVE.md', 'r') as f:
    content = f.read()

# Remove code-bio-dome from Condemned
content = content.replace(
    "- **code-bio-dome**: Specimen condemned. Diagnosis: Terminal Compilation Failure / Ecosystem Maladaptation. The specimen fails to compile due to an unresolved import `tui_shared::semantic`. Grace period: 24h.\n",
    ""
)

# Add code-bio-dome to Executed
executed_entry = "- **code-bio-dome**: Specimen executed. Diagnosis: Terminal Compilation Failure / Ecosystem Maladaptation. The specimen failed to compile due to an unresolved import `tui_shared::semantic`. Grace period expired. Moved to graveyard.\n"
content = content.replace('## ☠️ Executed\n', f'## ☠️ Executed\n{executed_entry}')

# Add neuro-git to Condemned
condemned_entry = "- **neuro-git**: Specimen condemned. Diagnosis: Terminal Compilation Failure / Documentation Void. The specimen fails to compile due to missing dependencies (`anyhow`, `rand`). Grace period: 24h.\n"
content = content.replace('## ☠️ Condemned (Awaiting Execution)\n', f'## ☠️ Condemned (Awaiting Execution)\n{condemned_entry}')

with open('ARCHIVE.md', 'w') as f:
    f.write(content)
