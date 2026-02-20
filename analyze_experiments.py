
import os
import pathlib

def analyze_experiments():
    root = pathlib.Path("experiments")
    results = []

    if not root.exists():
        print("No experiments directory found.")
        return

    for item in root.iterdir():
        if item.is_dir():
            score = 100
            name = item.name

            # Check README
            readme = item / "README.md"
            if not readme.exists():
                score -= 50
                readme_status = "MISSING"
            else:
                try:
                    content = readme.read_text()
                    if len(content) < 100:
                        score -= 20
                        readme_status = "WEAK"
                    else:
                        readme_status = "OK"
                except:
                    score -= 50
                    readme_status = "UNREADABLE"

            # Check Src
            src_main = item / "src" / "main.rs"
            src_lib = item / "src" / "lib.rs"
            has_code = False
            loc = 0

            if src_main.exists():
                has_code = True
                try:
                    loc = len(src_main.read_text().splitlines())
                except:
                    pass
            elif src_lib.exists():
                has_code = True
                try:
                    loc = len(src_lib.read_text().splitlines())
                except:
                    pass

            if not has_code:
                score -= 50
                code_status = "MISSING"
            else:
                if loc < 50:
                    score -= 30
                    code_status = "SKELETAL"
                elif loc < 200:
                    score -= 10
                    code_status = "THIN"
                else:
                    code_status = "OK"

            # Check Cargo
            cargo = item / "Cargo.toml"
            if not cargo.exists():
                score -= 50
                cargo_status = "MISSING"
            else:
                cargo_status = "OK"

            results.append({
                "name": name,
                "score": score,
                "readme": readme_status,
                "code": code_status,
                "loc": loc
            })

    # Sort by score ascending (worst first)
    results.sort(key=lambda x: x["score"])

    print(f"{'NAME':<30} {'SCORE':<10} {'README':<10} {'CODE':<10} {'LOC':<5}")
    print("-" * 70)
    for r in results[:15]:  # Show bottom 15
        print(f"{r['name']:<30} {r['score']:<10} {r['readme']:<10} {r['code']:<10} {r['loc']:<5}")

if __name__ == "__main__":
    analyze_experiments()
