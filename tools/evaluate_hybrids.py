import subprocess
import os
import re

def get_hybrid_experiments():
    hybrids = []
    try:
        with open("Cargo.toml", "r") as f:
            content = f.read()
            # Simple regex to find strings inside members list
            # Note: This is fragile but works for the current format
            # Looking for "experiments/..." inside quotes
            matches = re.findall(r'"experiments/([^"]+)"', content)
            hybrids = matches
    except Exception as e:
        print(f"Error reading Cargo.toml: {e}")
    return hybrids

results = {}

print("🧬 Evaluator: Beginning assessment of ALL hybrids...")

hybrids = get_hybrid_experiments()

if not hybrids:
    print("No hybrids found in Cargo.toml workspace members.")
else:
    print(f"Found {len(hybrids)} hybrids.")

for hybrid in hybrids:
    print(f"Checking {hybrid}...")
    try:
        # Check if directory exists
        if not os.path.exists(f"experiments/{hybrid}"):
            results[hybrid] = "MISSING DIRECTORY"
            continue

        # Try to compile
        cmd = ["cargo", "build", "-p", hybrid, "--quiet"]
        process = subprocess.run(cmd, capture_output=True, text=True, check=False)

        if process.returncode == 0:
            results[hybrid] = "COMPILES"
        else:
            # Short error summary
            err_lines = process.stderr.strip().split('\n')
            last_err = err_lines[-1] if err_lines else "Unknown error"
            results[hybrid] = f"FAILED: {last_err[:100]}"

    except Exception as e:
        results[hybrid] = f"ERROR: {str(e)}"

print("\n🧬 Evaluation Results:")
for h, r in results.items():
    print(f"- {h}: {r}")
