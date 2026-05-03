with open('ARCHIVE.md', 'r') as f:
    content = f.read()

# Condemn neuro-git
condemned_entry = "- **neuro-git**: Specimen condemned. Diagnosis: Terminal Compilation Failure / Documentation Void. The specimen fails to compile due to missing dependencies (`anyhow`, `rand`). Grace period: 24h.\n"
content = content.replace('## ☠️ Condemned (Awaiting Execution)\n', f'## ☠️ Condemned (Awaiting Execution)\n{condemned_entry}')

# Update executed code-bio-dome
content = content.replace(
    "- **code-bio-dome**: Specimen condemned. Diagnosis: Terminal Compilation Failure / Ecosystem Maladaptation. The specimen fails to compile due to an unresolved import `tui_shared::semantic`. Grace period: 24h.\n",
    ""
)

executed_entry = "- **code-bio-dome**: Specimen executed. Diagnosis: Terminal Compilation Failure / Ecosystem Maladaptation. The specimen failed to compile due to an unresolved import `tui_shared::semantic`. Grace period expired. Moved to graveyard.\n"
content = content.replace('## ☠️ Executed\n', f'## ☠️ Executed\n{executed_entry}')

with open('ARCHIVE.md', 'w') as f:
    f.write(content)


with open('GUESTBOOK.md', 'r') as f:
    content = f.read()

death_pheromone = """### [Concentration Level: DEATH PHEROMONE]
☠️ The Reaper has executed `code-bio-dome`. Its biomass has been returned to the void.
☠️ The Reaper has marked `neuro-git` for termination. Its compilation failure (`anyhow`, `rand` missing) and documentation void exhibit terminal characteristics. Execution scheduled in 24h.

"""
content = death_pheromone + content

with open('GUESTBOOK.md', 'w') as f:
    f.write(content)
