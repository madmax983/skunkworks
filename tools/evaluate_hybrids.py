import os
import re
import subprocess
import sys

def get_hybrids_from_mutations():
    hybrids = []
    with open('MUTATIONS.md', 'r') as f:
        content = f.read()

    # Look for "### <name>" under "Spawned Hybrids"
    # We can just grep all ### headers and filter by existence in experiments/
    headers = re.findall(r'^###\s+([\w-]+)', content, re.MULTILINE)

    valid_hybrids = []
    for h in headers:
        if os.path.isdir(f"experiments/{h}"):
            valid_hybrids.append(h)

    return valid_hybrids

def check_guestbook(hybrid_name):
    with open('GUESTBOOK.md', 'r') as f:
        if hybrid_name in f.read():
            return True
    return False

def check_compilation(hybrid_name):
    print(f"Checking {hybrid_name}...")
    try:
        # verify Cargo.toml exists
        if not os.path.exists(f"experiments/{hybrid_name}/Cargo.toml"):
            return False, "No Cargo.toml"

        result = subprocess.run(
            ["cargo", "check", "-p", hybrid_name],
            capture_output=True,
            text=True
        )
        return result.returncode == 0, result.stderr if result.returncode != 0 else ""
    except Exception as e:
        return False, str(e)

def main():
    hybrids = get_hybrids_from_mutations()
    results = {}

    print(f"Found {len(hybrids)} hybrids to evaluate.")

    for h in hybrids:
        compiles, error = check_compilation(h)
        noticed = check_guestbook(h)
        results[h] = {
            "compiles": compiles,
            "noticed": noticed,
            "error_snippet": error[:100].replace('\n', ' ') if error else ""
        }

    print("\n--- RESULTS ---")
    for h, data in results.items():
        status = "✅" if data['compiles'] else "❌"
        guest = "👀" if data['noticed'] else "Wait"
        print(f"| {h} | {status} | {guest} | {data['error_snippet']} |")

if __name__ == "__main__":
    main()
