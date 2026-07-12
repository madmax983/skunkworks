import re

with open("GUESTBOOK.md", "r") as f:
    guestbook = f.read()

scent = "\n- `[DEATH PHEROMONE]` ☠️ **Reaper**: Condemned `locus-origami` (Skeletal Implementation, Execution Void). 24h until execution.\n"

guestbook = guestbook.replace("## Scent Trails (Living System State)", "## Scent Trails (Living System State)" + scent)

with open("GUESTBOOK.md", "w") as f:
    f.write(guestbook)
