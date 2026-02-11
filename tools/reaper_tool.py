import os
import re

def get_archive_status():
    archive_path = "ARCHIVE.md"
    condemned = set()
    executed = set()

    if os.path.exists(archive_path):
        with open(archive_path, "r", encoding="utf-8") as f:
            content = f.read()

            # Simple parsing - this might be brittle but sufficient for now
            # Look for lines starting with "- **name**:"

            # We need to distinguish sections.
            # Split by headers
            sections = re.split(r'^##\s+', content, flags=re.MULTILINE)

            for section in sections:
                lines = section.split('\n')
                if not lines: continue
                header = lines[0].strip().lower()

                names = []
                for line in lines[1:]:
                    match = re.search(r'-\s*\*\*([a-zA-Z0-9_-]+)\*\*:', line)
                    if match:
                        names.append(match.group(1))

                if "condemned" in header:
                    condemned.update(names)
                elif "executed" in header:
                    executed.update(names)

    return condemned, executed

def scan_experiment(path):
    name = os.path.basename(path)
    readme_path = os.path.join(path, "README.md")
    cargo_path = os.path.join(path, "Cargo.toml")
    src_path = os.path.join(path, "src")

    has_readme = os.path.exists(readme_path)
    has_cargo = os.path.exists(cargo_path)

    loc = 0
    todo_count = 0

    if os.path.exists(src_path):
        for root, _, files in os.walk(src_path):
            for file in files:
                if file.endswith(".rs"):
                    try:
                        with open(os.path.join(root, file), "r", encoding="utf-8", errors="ignore") as f:
                            lines = f.readlines()
                            loc += len(lines)
                            for line in lines:
                                if "TODO" in line:
                                    todo_count += 1
                    except:
                        pass

    return {
        "name": name,
        "path": path,
        "has_readme": has_readme,
        "has_cargo": has_cargo,
        "loc": loc,
        "todos": todo_count
    }

def main():
    experiments_dir = "experiments"
    if not os.path.exists(experiments_dir):
        print("Error: experiments directory not found")
        return

    condemned, executed = get_archive_status()
    print(f"Known Condemned: {len(condemned)}")
    print(f"Known Executed: {len(executed)}")

    experiments = []
    for entry in os.listdir(experiments_dir):
        full_path = os.path.join(experiments_dir, entry)
        if os.path.isdir(full_path):
            if entry in condemned:
                print(f"Skipping {entry} (Condemned)")
                continue
            if entry in executed:
                print(f"Skipping {entry} (Executed - Zombie?)")
                continue

            experiments.append(scan_experiment(full_path))

    # Sort by score (subjective mix of LOC and README)
    experiments.sort(key=lambda x: (x["has_readme"], x["loc"]))

    print(f"\n{'Name':<30} | {'README':<6} | {'Cargo':<6} | {'LOC':<6} | {'TODOs':<6}")
    print("-" * 75)

    for exp in experiments[:20]:
        print(f"{exp['name']:<30} | {str(exp['has_readme']):<6} | {str(exp['has_cargo']):<6} | {exp['loc']:<6} | {exp['todos']:<6}")

if __name__ == "__main__":
    main()
