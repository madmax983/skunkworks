import os

def check_experiment(path):
    has_readme = os.path.exists(os.path.join(path, "README.md"))
    has_cargo = os.path.exists(os.path.join(path, "Cargo.toml"))
    file_count = 0
    for _, _, files in os.walk(path):
        file_count += len(files)

    return {
        "path": path,
        "has_readme": has_readme,
        "has_cargo": has_cargo,
        "file_count": file_count
    }

def main():
    experiments_dir = "experiments"
    results = []
    if not os.path.exists(experiments_dir):
        print("No experiments directory found.")
        return

    for name in os.listdir(experiments_dir):
        path = os.path.join(experiments_dir, name)
        if os.path.isdir(path):
            results.append(check_experiment(path))

    # Sort by health (worst first)
    # Penalize missing README, missing Cargo, low file count
    def score(e):
        s = 0
        if e["has_readme"]: s += 10
        if e["has_cargo"]: s += 10
        s += min(e["file_count"], 10) # Cap file count contribution
        return s

    results.sort(key=score)

    print(f"{'Path':<40} {'README':<10} {'Cargo':<10} {'Files':<10} {'Score':<10}")
    print("-" * 80)
    for res in results[:20]: # Show top 20 worst
        print(f"{res['path']:<40} {str(res['has_readme']):<10} {str(res['has_cargo']):<10} {res['file_count']:<10} {score(res):<10}")

if __name__ == "__main__":
    main()
