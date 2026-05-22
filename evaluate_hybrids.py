import subprocess

hybrids_to_check = [
    'poincare-flock',
    'poincare-fluid',
    'myco-resonance',
    'quipu-poincare',
    'market-poincare',
    'locus-flock'
]

results = {}
for hybrid in hybrids_to_check:
    print(f"Checking {hybrid}...")
    res = subprocess.run(["cargo", "build", "-p", hybrid], capture_output=True)
    results[hybrid] = "Compiled" if res.returncode == 0 else "Failed"

print(results)
