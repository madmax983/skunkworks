import re

with open("GUESTBOOK.md", "r") as f:
    guestbook = f.read()

scent = "\n- `[DEATH PHEROMONE]` ☠️ **Reaper**: Condemned `locus-origami` (Skeletal Implementation, Execution Void). 24h until execution.\n"

if "## Active Trails/Hotspots" in guestbook:
    guestbook = guestbook.replace("## Active Trails/Hotspots", "## Active Trails/Hotspots" + scent)

with open("GUESTBOOK.md", "w") as f:
    f.write(guestbook)
