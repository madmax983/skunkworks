import re
from collections import defaultdict

with open('DX_AUDIT_LOG.md', 'r') as f:
    dx_content = f.read()

reports = dx_content.split('---')
target_reports = defaultdict(list)

for report in reports:
    if "Echo's DX Audit Log" not in report:
        continue

    target_match = re.search(r'\*\*Target:\*\* `(.*?)`', report)
    if not target_match:
        target_match = re.search(r'\*\*Target:\*\* (.*?)\n', report)
        if not target_match:
            continue
    target = target_match.group(1).strip('` ')

    # Try to find The Reality
    reality_match = re.search(r'🕵️ \*\*The Reality:\*\* "(.*?)"', report)
    if reality_match:
        target_reports[target].append(reality_match.group(1))

with open('GUESTBOOK.md', 'r') as f:
    gb_content = f.read()

map_start = gb_content.find('## 🧫 Current Pheromone Map')

if map_start != -1:
    end_of_line = gb_content.find('\n', map_start)
    insert_point = end_of_line + 1

    pheromones = []
    for target, realities in target_reports.items():
        if len(realities) > 1:
            level = "CRITICAL MASS"
        else:
            level = "STABLE TRAIL"

        status_items = list(set(realities))
        if len(status_items) > 1:
            status = "Multiple friction points: " + " | ".join(status_items)
        else:
            status = status_items[0]

        pheromone = f"### [Concentration Level: {level}] - Location: {target}\n- **Scent Origin:** Echo 🗣️\n- **Status:** {status}\n\n"
        pheromones.append(pheromone)

    # We will PREPEND these pheromones into `## 🧫 Current Pheromone Map`
    # instead of wiping existing records
    new_content = gb_content[:insert_point] + "\n" + "".join(pheromones) + gb_content[insert_point:]

    with open('GUESTBOOK.md', 'w') as f:
        f.write(new_content)
    print("Done")
else:
    print("Not found")
