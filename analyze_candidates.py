
import os
import re

def analyze_experiment(path):
    score = 0
    report = []

    # Check README
    readme_path = os.path.join(path, "README.md")
    if not os.path.exists(readme_path):
        score += 50
        report.append("MISSING_README")
    else:
        with open(readme_path, 'r', errors='ignore') as f:
            content = f.read()
            if len(content) < 100:
                score += 20
                report.append("SKELETAL_README")
            if "TODO" in content:
                score += 10
                report.append("TODO_IN_README")
            if "generic" in content.lower():
                score += 5
                report.append("SELF_DESCRIBED_GENERIC")

    # Check Src
    src_path = os.path.join(path, "src")
    if not os.path.exists(src_path):
        score += 100
        report.append("MISSING_SRC")
    else:
        file_count = 0
        sloc = 0
        for root, dirs, files in os.walk(src_path):
            for file in files:
                if file.endswith(".rs"):
                    file_count += 1
                    with open(os.path.join(root, file), 'r', errors='ignore') as f:
                        sloc += len(f.readlines())

        if file_count <= 1:
            score += 15
            report.append("SINGLE_FILE")
        if sloc < 50:
            score += 30
            report.append("EXTREMELY_LOW_SLOC")
        elif sloc < 200:
            score += 10
            report.append("LOW_SLOC")

    return score, report

def main():
    experiments_dir = "experiments"
    candidates = []

    for name in os.listdir(experiments_dir):
        path = os.path.join(experiments_dir, name)
        if os.path.isdir(path):
            score, report = analyze_experiment(path)
            candidates.append((score, name, report))

    # Sort by score (descending = worst)
    candidates.sort(key=lambda x: x[0], reverse=True)

    print(f"{'SCORE':<5} {'NAME':<30} {'ISSUES'}")
    print("-" * 60)
    for score, name, report in candidates[:15]:
        print(f"{score:<5} {name:<30} {', '.join(report)}")

if __name__ == "__main__":
    main()
