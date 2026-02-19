import os

def check_experiment(path):
    name = os.path.basename(path)
    readme_path = os.path.join(path, "README.md")
    cargo_path = os.path.join(path, "Cargo.toml")
    src_path = os.path.join(path, "src")
    main_rs_path = os.path.join(src_path, "main.rs")

    has_readme = os.path.exists(readme_path)
    readme_size = os.path.getsize(readme_path) if has_readme else 0

    has_cargo = os.path.exists(cargo_path)
    cargo_valid = False
    dependencies = []

    if has_cargo:
        try:
            with open(cargo_path, 'r') as f:
                content = f.read()
                if '[package]' in content:
                    cargo_valid = True
        except:
            pass

    file_count = 0
    total_size = 0
    has_main = os.path.exists(main_rs_path)
    main_content = ""
    if has_main:
        with open(main_rs_path, 'r') as f:
            main_content = f.read()

    for root, _, files in os.walk(path):
        for file in files:
            if "target" in root: continue
            file_count += 1
            total_size += os.path.getsize(os.path.join(root, file))

    # Heuristics
    score = 0
    reasons = []

    if not has_readme:
        score += 20
        reasons.append("Documentation Void (No README)")
    elif readme_size < 100:
        score += 10
        reasons.append("Skeletal Documentation")

    if not has_cargo:
        score += 50
        reasons.append("Not a Rust Project (No Cargo.toml)")
    elif not cargo_valid:
        score += 30
        reasons.append("Broken Configuration (Invalid Cargo.toml)")

    if file_count < 3:
        score += 15
        reasons.append("Skeletal Structure (<3 files)")

    if "TODO" in main_content:
        score += 5
        reasons.append("Unfinished Logic (TODOs)")

    if main_content.strip() == 'fn main() {\n    println!("Hello, world!");\n}':
        score += 40
        reasons.append("Terminal Genericism (Hello World)")

    return {
        "name": name,
        "score": score,
        "reasons": reasons,
        "file_count": file_count,
        "readme_size": readme_size,
        "path": path
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

    # Sort by score (highest is worst)
    results.sort(key=lambda x: x["score"], reverse=True)

    print(f"{'Name':<30} {'Score':<5} {'Reasons'}")
    print("-" * 80)
    for res in results[:20]: # Show top 20 worst
        reasons_str = ", ".join(res["reasons"])
        print(f"{res['name']:<30} {res['score']:<5} {reasons_str}")

if __name__ == "__main__":
    main()
