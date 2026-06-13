import re
from collections import defaultdict

with open('DX_AUDIT_LOG.md', 'r') as f:
    dx_content = f.read()

# 1. Parse DX_AUDIT_LOG.md
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

    reality_match = re.search(r'🕵️ \*\*The Reality:\*\* "(.*?)"', report, re.DOTALL)
    if reality_match:
        val = reality_match.group(1).strip()
        val = val.replace('\n', ' ').replace('\r', ' ')
        if "Command failed" in val:
            val = "The example command resulted in an error because it could not determine which binary to run in the workspace."
        if "ratatui =" in val:
            val = "Turns out the README tells external users to use `ratatui = \"0.30\"`, but the internal `tui-shared` crate relies on workspace dependencies that are pinned to older versions."
        if "Turns out the repository is a massive workspace so a bare" in val:
            val = "Turns out the repository is a massive workspace so a bare `cargo run` doesn't work. Furthermore, the `chimera-lang` binary doesn't seem to know how to parse `.pro` files natively without extra configuration or flags that are completely missing from the README."
        if "Turns out the Mad Scientist mode injects chaos runes" in val:
            val = "Turns out the Mad Scientist mode injects chaos runes that the VM tries to execute as OpCodes, causing error spam. The repo structure doesn't match the clone instructions, and the library is deeply coupled with random workspace crates instead of keeping them optional or private."
        target_reports[target].append(val)

# Also capture any OTHER Echo items in DX_AUDIT_LOG that are maybe not explicitly "The Reality"
# For example, missing `genesis.chs`
for report in reports:
    if "Echo's DX Audit Log" not in report:
        continue
    target_match = re.search(r'\*\*Target:\*\* `(.*?)`', report)
    if not target_match:
        target_match = re.search(r'\*\*Target:\*\* (.*?)\n', report)
        if not target_match:
            continue
    target = target_match.group(1).strip('` ')

    # if it's genesis missing
    if "genesis.chs" in report and "does not exist in the codebase" in report and not reality_match:
        pass # Already captured above because reality_match handles "(.*?)"

# Handle cases where DX_AUDIT_LOG might use slightly different target names for the same module
normalized_reports = defaultdict(list)
for t, realities in target_reports.items():
    norm_t = t
    if t == "chimera-lang" or t == "chimera-lang Compilation" or t == "chimera-lang Getting Started (Story Demo)" or t == "chimera-lang Library Usage (story_demo)" or t == "experiments/chimera-lang/Cargo.toml" or t == "experiments/chimera-lang/src/main.rs" or t == "experiments/chimera-lang/src/prolouge_compiler.rs" or t == "chimera-lang/README.md":
        norm_t = "experiments/chimera-lang"
    if t == "experiments/chimera-lang/README.md":
        norm_t = "experiments/chimera-lang"
    if t == "README.md":
        norm_t = "README.md"
    if t == "MARKETPLACE.md":
        norm_t = "MARKETPLACE.md"

    for r in realities:
        if r not in normalized_reports[norm_t]:
            normalized_reports[norm_t].append(r)

# 2. Parse GUESTBOOK.md
with open('GUESTBOOK.md', 'r') as f:
    gb_content = f.read()

map_start = gb_content.find('## 🧫 Current Pheromone Map')
decay_start = gb_content.find('## 🍂 History/Decay')

pre_map = gb_content[:map_start + len('## 🧫 Current Pheromone Map\n\n')]
current_map_section = gb_content[map_start + len('## 🧫 Current Pheromone Map\n\n'):decay_start]
decay_section = gb_content[decay_start:]

blocks = re.split(r'(### \[[^\]]+\](?: - Location:.*)?\n)', current_map_section)

existing_map_blocks = []
decay_blocks_to_add = []

i = 1
while i < len(blocks):
    header = blocks[i]
    body = blocks[i+1] if i+1 < len(blocks) else ""

    # Find the target if it exists
    target_match = re.search(r'Location:\s*([^\n]+)', header)
    loc = None
    if target_match:
        loc = target_match.group(1).strip().strip('`')
        if loc == "chimera-lang" or loc == "experiments/chimera-lang/README.md" or loc == "chimera-lang/README.md" or "chimera-lang" in loc:
            loc = "experiments/chimera-lang"
    elif "MARKETPLACE.md" in header:
        loc = "MARKETPLACE.md"
    elif "README.md" in header:
        loc = "README.md"

    if "Scent Origin:** Echo" in body:
        # We process it and DO NOT append to existing_map_blocks
        status_match = re.search(r'\*\*Status:\*\*\s*(.*?)\n', body, re.DOTALL)
        if status_match and loc:
            old_status_raw = status_match.group(1).strip()
            # It could be single or multiple
            if "Multiple friction points:" in old_status_raw:
                parts = old_status_raw.replace("Multiple friction points: ", "").split(" | ")
                for p in parts:
                    p = p.strip()
                    if p and p not in normalized_reports[loc]:
                        normalized_reports[loc].append(p)
            else:
                if old_status_raw and old_status_raw not in normalized_reports[loc]:
                    normalized_reports[loc].append(old_status_raw)
    else:
        # Keep non-Echo entries
        existing_map_blocks.append(header + body)

    i += 2

# 3. Generate New Echo Pheromones
pheromones = []
for target, realities in normalized_reports.items():
    if len(realities) > 1:
        level = "CRITICAL MASS"
    else:
        level = "STABLE TRAIL"

    if len(realities) > 1:
        status = "Multiple friction points: " + " | ".join(realities)
    else:
        status = realities[0] if realities else "Friction detected."

    pheromone = f"### [Concentration Level: {level}] - Location: {target}\n- **Scent Origin:** Echo 🗣️\n- **Status:** {status}\n\n"
    pheromones.append(pheromone)

# 4. Construct Final Document
new_current_map = "".join(existing_map_blocks) + "".join(pheromones)
new_content = pre_map + new_current_map + "\n" + decay_section

with open('GUESTBOOK.md', 'w') as f:
    f.write(new_content)
print("Done")
