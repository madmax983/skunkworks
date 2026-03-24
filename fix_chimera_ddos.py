import re

with open("MUTATIONS.md", "r") as f:
    content = f.read()

old_eval = "- **Evaluation**: Success. Compiled. Swarm intelligence adapts via genetic inheritance and crossover."
new_eval = "- **Evaluation**: Success. Compiled. Strong GUESTBOOK mentions confirm genetic adaptation. The Reaper pardoned the entity as the swarm intelligence correctly weaves and adapts via genetic inheritance and crossover, bypassing firewalls without exploding the stack."

content = content.replace(old_eval, new_eval)

with open("MUTATIONS.md", "w") as f:
    f.write(content)
