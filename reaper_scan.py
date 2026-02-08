import os
import glob

def scan_experiments():
    root = "experiments"
    results = []

    if not os.path.exists(root):
        print("No experiments directory found.")
        return

    for exp in os.listdir(root):
        exp_path = os.path.join(root, exp)
        if not os.path.isdir(exp_path):
            continue

        readme_exists = os.path.exists(os.path.join(exp_path, "README.md"))
        cargo_exists = os.path.exists(os.path.join(exp_path, "Cargo.toml"))

        loc = 0
        todos = 0
        src_dir = os.path.join(exp_path, "src")
        if os.path.exists(src_dir):
            for r, d, f in os.walk(src_dir):
                for file in f:
                    if file.endswith(".rs"):
                        try:
                            with open(os.path.join(r, file), 'r', encoding='utf-8', errors='ignore') as rs:
                                content = rs.readlines()
                                loc += len(content)
                                for line in content:
                                    if "TODO" in line:
                                        todos += 1
                        except:
                            pass

        results.append({
            "name": exp,
            "readme": readme_exists,
            "cargo": cargo_exists,
            "loc": loc,
            "todos": todos
        })

    # Sort by LOC (ascending) and then by missing README
    results.sort(key=lambda x: (x["readme"], x["loc"]))

    print(f"{'Name':<30} | {'README':<6} | {'Cargo':<6} | {'LOC':<6} | {'TODOs':<6}")
    print("-" * 70)
    for r in results[:20]:
        print(f"{r['name']:<30} | {str(r['readme']):<6} | {str(r['cargo']):<6} | {r['loc']:<6} | {r['todos']:<6}")

if __name__ == "__main__":
    scan_experiments()
