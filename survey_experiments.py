import os

def analyze_experiments(root_dir="experiments"):
    results = []
    if not os.path.exists(root_dir):
        print(f"Directory {root_dir} not found.")
        return

    for exp in os.listdir(root_dir):
        exp_path = os.path.join(root_dir, exp)
        if not os.path.isdir(exp_path):
            continue

        readme_path = os.path.join(exp_path, "README.md")
        cargo_path = os.path.join(exp_path, "Cargo.toml")
        reaper_path = os.path.join(exp_path, ".reaper-report.md")
        src_dir = os.path.join(exp_path, "src")

        has_readme = os.path.exists(readme_path)
        has_cargo = os.path.exists(cargo_path)
        is_condemned = os.path.exists(reaper_path)

        loc = 0
        todo_count = 0

        if os.path.exists(src_dir):
            for root, _, files in os.walk(src_dir):
                for file in files:
                    if file.endswith(".rs"):
                        try:
                            with open(os.path.join(root, file), "r", encoding="utf-8") as f:
                                lines = f.readlines()
                                loc += len(lines)
                                for line in lines:
                                    if "TODO" in line:
                                        todo_count += 1
                        except Exception:
                            pass

        results.append({
            "name": exp,
            "has_readme": has_readme,
            "loc": loc,
            "todo_count": todo_count,
            "has_cargo": has_cargo,
            "is_condemned": is_condemned
        })

    # Sort by "worst" qualities: No README, Low LOC (sketch), High TODOs
    results.sort(key=lambda x: (x["has_readme"], x["loc"], -x["todo_count"]))

    print(f"{'Name':<25} | {'README':<6} | {'LOC':<5} | {'TODOs':<5} | {'Cargo':<5} | {'Condemned'}")
    print("-" * 80)
    for r in results:
        condemned_mark = "☠️" if r["is_condemned"] else ""
        print(f"{r['name']:<25} | {str(r['has_readme']):<6} | {r['loc']:<5} | {r['todo_count']:<5} | {str(r['has_cargo']):<5} | {condemned_mark}")

if __name__ == "__main__":
    analyze_experiments()
