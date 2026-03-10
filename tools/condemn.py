import os
import sys

def condemn_experiment(name, diagnosis, grace_period="24h"):
    print(f"Condemning {name}...")

    # Create forensic report
    report_content = f"""# Reaper Report: `{name}`

## Forensic Diagnosis
Specimen `{name}` exhibits terminal characteristics.

- {diagnosis}

## Execution Criteria
- **Staleness**: High.
- **Swarm Activity**: None detected. Silent in the GUESTBOOK.
- **Execution Quality**: Minimal. Skeletal logic.
- **Persona Alignment**: Generic.

## Conclusion
**CONDEMN**.
Grace period: {grace_period}.
"""
    with open(f"experiments/{name}/.reaper-report.md", "w") as f:
        f.write(report_content)

    # Update ARCHIVE.md
    with open("ARCHIVE.md", "r") as f:
        archive = f.read()

    new_entry = f"- **{name}**: Specimen condemned. Diagnosis: {diagnosis}. Grace period: {grace_period}."
    if "## ☠️ Condemned (Awaiting Execution)\n" in archive:
        archive = archive.replace("## ☠️ Condemned (Awaiting Execution)\n", f"## ☠️ Condemned (Awaiting Execution)\n{new_entry}\n")
    else:
        archive = f"## ☠️ Condemned (Awaiting Execution)\n{new_entry}\n\n" + archive

    with open("ARCHIVE.md", "w") as f:
        f.write(archive)

    # Update GUESTBOOK.md
    guestbook_entry = f"""### [Concentration Level: EVAPORATING] - Location: experiments/{name}
- **Scent Origin:** The Reaper ☠️
- **Status:** Specimen condemned. Diagnosis: {diagnosis}. Grace period: {grace_period}.
"""
    with open("GUESTBOOK.md", "r") as f:
        guestbook = f.read()

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
        print("Usage: python tools/condemn.py <experiment_name> <diagnosis>")
        sys.exit(1)
    condemn_experiment(sys.argv[1], sys.argv[2])
