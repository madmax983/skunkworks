import os

def analyze_experiments():
    root = "experiments"
    weak_candidates = []

    if not os.path.exists(root):
        print(f"Directory {root} does not exist.")
        return

    for exp in sorted(os.listdir(root)):
        path = os.path.join(root, exp)
        if not os.path.isdir(path):
            continue

        has_readme = os.path.exists(os.path.join(path, "README.md"))
        has_cargo = os.path.exists(os.path.join(path, "Cargo.toml"))

        src_path = os.path.join(path, "src")
        rs_files = 0
        if os.path.exists(src_path):
            for _, _, files in os.walk(src_path):
                rs_files += sum(1 for f in files if f.endswith(".rs"))

        if not has_readme or rs_files <= 1:
            weak_candidates.append({
                "name": exp,
                "has_readme": has_readme,
                "rs_files": rs_files,
                "has_cargo": has_cargo
            })

    print(f"Found {len(weak_candidates)} weak candidates:")
    for c in weak_candidates:
        print(f"{c['name']}: README={c['has_readme']}, RS_FILES={c['rs_files']}, CARGO={c['has_cargo']}")

if __name__ == "__main__":
    analyze_experiments()
