import os
import sys

def scan_experiments():
    root = "experiments"
    results = []

    if not os.path.exists(root):
        print(f"Error: {root} directory not found.")
        return

    for experiment in os.listdir(root):
        exp_path = os.path.join(root, experiment)
        if not os.path.isdir(exp_path):
            continue

        score = 0
        notes = []

        # Check README
        readme_path = os.path.join(exp_path, "README.md")
        if os.path.exists(readme_path):
            score += 20
        else:
            notes.append("No README")

        # Check Cargo.toml
        cargo_path = os.path.join(exp_path, "Cargo.toml")
        if os.path.exists(cargo_path):
            score += 10
        else:
            score -= 50
            notes.append("No Cargo.toml")

        # Check Source Code
        src_path = os.path.join(exp_path, "src")
        main_rs = os.path.join(src_path, "main.rs")
        lib_rs = os.path.join(src_path, "lib.rs")

        has_code = False
        code_size = 0

        if os.path.exists(main_rs):
            has_code = True
            code_size = os.path.getsize(main_rs)
        elif os.path.exists(lib_rs):
            has_code = True
            code_size = os.path.getsize(lib_rs)

        if has_code:
            # Heuristic: < 500 bytes is likely a stub
            if code_size < 500:
                score -= 10
                notes.append(f"Small code ({code_size} bytes)")
            else:
                score += min(code_size // 100, 20) # Max 20 points for size
        else:
            score -= 20
            notes.append("No src/main.rs or src/lib.rs")

        results.append({
            "name": experiment,
            "score": score,
            "notes": ", ".join(notes)
        })

    # Sort by score ascending (worst first)
    results.sort(key=lambda x: x["score"])

    print(f"{'Name':<30} | {'Score':<5} | {'Notes'}")
    print("-" * 80)
    for r in results[:15]:
        print(f"{r['name']:<30} | {r['score']:<5} | {r['notes']}")

if __name__ == "__main__":
    scan_experiments()
