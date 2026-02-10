
import os
import sys

def scan_experiment(path):
    score = 100
    issues = []

    # Check README
    readme_path = os.path.join(path, "README.md")
    if not os.path.exists(readme_path):
        score -= 50
        issues.append("Missing README.md")
    elif os.path.getsize(readme_path) < 100:
        score -= 20
        issues.append("README.md too small")

    # Check Cargo.toml
    cargo_path = os.path.join(path, "Cargo.toml")
    if not os.path.exists(cargo_path):
        score -= 100
        issues.append("Missing Cargo.toml")

    # Check src/main.rs or src/lib.rs
    src_path = os.path.join(path, "src")
    main_rs = os.path.join(src_path, "main.rs")
    lib_rs = os.path.join(src_path, "lib.rs")

    if not os.path.exists(src_path):
        score -= 50
        issues.append("Missing src/ directory")
    elif not (os.path.exists(main_rs) or os.path.exists(lib_rs)):
        score -= 50
        issues.append("Missing main.rs/lib.rs")
    else:
        # Check TODOs
        todo_count = 0
        file_size = 0
        target_file = main_rs if os.path.exists(main_rs) else lib_rs
        with open(target_file, 'r', errors='ignore') as f:
            content = f.read()
            todo_count = content.count("TODO") + content.count("todo!")
            file_size = len(content)

        if todo_count > 5:
            score -= 10
            issues.append(f"High TODO count ({todo_count})")
        if file_size < 500:
            score -= 20
            issues.append(f"Small source file ({file_size} bytes)")

    return score, issues

def main():
    root = "experiments"
    results = []

    for name in os.listdir(root):
        path = os.path.join(root, name)
        if os.path.isdir(path):
            score, issues = scan_experiment(path)
            results.append((score, name, issues))

    # Sort by score (ascending)
    results.sort(key=lambda x: x[0])

    print(f"{'Score':<10} {'Name':<30} {'Issues'}")
    print("-" * 80)
    for score, name, issues in results[:10]: # Top 10 worst
        print(f"{score:<10} {name:<30} {', '.join(issues)}")

if __name__ == "__main__":
    main()
