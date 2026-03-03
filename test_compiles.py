import os
import subprocess

exp_dir = 'experiments'
failed = []

for entry in os.listdir(exp_dir):
    full_path = os.path.join(exp_dir, entry)
    if not os.path.isdir(full_path): continue

    # Run cargo check
    result = subprocess.run(['cargo', 'check', '-p', entry], capture_output=True, text=True)
    if result.returncode != 0:
        failed.append(entry)
        print(f"FAILED: {entry}")

print("Total failed:", len(failed))
