import os
import sys
import shutil

def execute_experiment(name, diagnosis):
    print(f"Executing {name}...")

    # Move to graveyard
    shutil.move(f"experiments/{name}", f"graveyard/{name}")

    # Update ARCHIVE.md
    with open("ARCHIVE.md", "r") as f:
        archive = f.read()

    # Remove from Condemned
    archive_lines = archive.splitlines()
    new_archive = []
    in_condemned = False
    for line in archive_lines:
        if line == "## ☠️ Condemned (Awaiting Execution)":
            in_condemned = True
            new_archive.append(line)
        elif line.startswith("## ") and in_condemned:
            in_condemned = False
            new_archive.append(line)
        elif in_condemned and line.startswith(f"- **{name}**:"):
            pass # Skip it
        else:
            new_archive.append(line)

    archive = "\n".join(new_archive)

    # Add to Executed
    new_entry = f"- **{name}**: Specimen executed. Diagnosis: {diagnosis}. Grace period expired. Moved to graveyard."
    if "## Executed\n" in archive:
        archive = archive.replace("## Executed\n", f"## Executed\n{new_entry}\n")
    else:
        archive += f"\n## Executed\n{new_entry}\n"

    with open("ARCHIVE.md", "w") as f:
        f.write(archive)

    # Update GUESTBOOK.md
    guestbook_entry = f"""### [Concentration Level: EVAPORATING] - Location: graveyard/{name} (Executed)
- **Scent Origin:** The Reaper ☠️
- **Status:** Specimen executed. Diagnosis: {diagnosis}. Grace period expired. Moved to graveyard.
"""
    with open("GUESTBOOK.md", "r") as f:
        guestbook = f.read()

    # Remove old entry
    old_marker = f"Location: experiments/{name}"
    guestbook_lines = guestbook.splitlines()
    new_guestbook = []
    skip = False
    for line in guestbook_lines:
        if old_marker in line:
            skip = True
        elif skip and line.startswith("### "):
            skip = False
        if not skip:
            new_guestbook.append(line)

    guestbook = "\n".join(new_guestbook)

    idx = guestbook.find("## 🍂 History/Decay")
    if idx != -1:
        # insert after the header
        lines = guestbook.splitlines()
        for i, l in enumerate(lines):
            if l.strip() == "## 🍂 History/Decay":
                lines.insert(i + 1, guestbook_entry)
                break
        guestbook = "\n".join(lines)
    else:
        guestbook += f"\n## 🍂 History/Decay\n{guestbook_entry}"

    with open("GUESTBOOK.md", "w") as f:
        f.write(guestbook)

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Usage: python tools/execute.py <experiment_name> <diagnosis>")
        sys.exit(1)
    execute_experiment(sys.argv[1], sys.argv[2])
