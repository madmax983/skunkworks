
import os

def audit_experiments():
    root = "experiments"
    experiments = sorted([d for d in os.listdir(root) if os.path.isdir(os.path.join(root, d))])

    candidates = []

    for exp in experiments:
        path = os.path.join(root, exp)
        readme_path = os.path.join(path, "README.md")
        src_path = os.path.join(path, "src")
        main_path = os.path.join(src_path, "main.rs")

        has_readme = os.path.exists(readme_path)
        main_size = 0
        if os.path.exists(main_path):
            main_size = os.path.getsize(main_path)

        # Score: 0 is worst.
        # +1 for README
        # +1 for main.rs > 500 bytes (arbitrary 'hello world' threshold)

        score = 0
        if has_readme: score += 1
        if main_size > 500: score += 1

        candidates.append({
            "name": exp,
            "has_readme": has_readme,
            "main_size": main_size,
            "score": score
        })

    # Sort by score (ascending), then main_size (ascending)
    candidates.sort(key=lambda x: (x["score"], x["main_size"]))

    print(f"{'Name':<30} | {'README':<6} | {'Main Size':<10} | {'Score'}")
    print("-" * 60)
    for c in candidates[:20]: # Show top 20 worst
        print(f"{c['name']:<30} | {str(c['has_readme']):<6} | {c['main_size']:<10} | {c['score']}")

if __name__ == "__main__":
    audit_experiments()
