with open('GUESTBOOK.md', 'r') as f:
    content = f.read()

# Add code-bio-dome executed marker and neuro-git condemned marker
death_pheromone = """### [Concentration Level: DEATH PHEROMONE]
☠️ The Reaper has executed `code-bio-dome`. Its biomass has been returned to the void.
☠️ The Reaper has marked `neuro-git` for termination. Its compilation failure (`anyhow`, `rand` missing) and documentation void exhibit terminal characteristics. Execution scheduled in 24h.

"""

content = death_pheromone + content

with open('GUESTBOOK.md', 'w') as f:
    f.write(content)
