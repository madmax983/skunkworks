import os
import json
import subprocess
from pathlib import Path

def scan_experiments(root_dir="experiments"):
    results = []

    if not os.path.exists(root_dir):
        print(f"Directory {root_dir} not found.")
        return []

    for entry in os.scandir(root_dir):
        if not entry.is_dir():
            continue

        path = Path(entry.path)
        name = entry.name

        has_cargo = (path / "Cargo.toml").exists()
        has_readme = (path / "README.md").exists()
        src_dir = path / "src"
        has_src = src_dir.exists() and src_dir.is_dir()

        loc = 0
        if has_src:
            for root, dirs, files in os.walk(src_dir):
                for file in files:
                    if file.endswith(".rs"):
                        try:
                            with open(os.path.join(root, file), "r", encoding="utf-8") as f:
                                loc += sum(1 for line in f if line.strip())
                        except:
                            pass

        # Check last modified time of src/main.rs or Cargo.toml
        last_modified = 0
        if has_cargo:
            try:
                last_modified = (path / "Cargo.toml").stat().st_mtime
            except:
                pass

        results.append({
            "name": name,
            "has_cargo": has_cargo,
            "has_readme": has_readme,
            "loc": loc,
            "last_modified": last_modified
        })

    return results

if __name__ == "__main__":
    data = scan_experiments()
    print(json.dumps(data, indent=2))
