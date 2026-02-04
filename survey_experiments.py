import os
import time

experiments_dir = "experiments"
results = []

for name in os.listdir(experiments_dir):
    path = os.path.join(experiments_dir, name)
    if os.path.isdir(path):
        has_readme = os.path.exists(os.path.join(path, "README.md"))
        has_cargo = os.path.exists(os.path.join(path, "Cargo.toml"))

        # Get last modified time of the directory (not recursive, but a proxy)
        # Better: get mtime of src/main.rs or similar if it exists
        mtime = os.path.getmtime(path)

        # Check for src directory
        has_src = os.path.isdir(os.path.join(path, "src"))

        results.append({
            "name": name,
            "has_readme": has_readme,
            "has_cargo": has_cargo,
            "has_src": has_src,
            "mtime": mtime
        })

# Sort by mtime (oldest first)
results.sort(key=lambda x: x["mtime"])

print(f"{'Name':<30} {'README':<10} {'Cargo':<10} {'Src':<10}")
print("-" * 60)
for r in results:
    print(f"{r['name']:<30} {str(r['has_readme']):<10} {str(r['has_cargo']):<10} {str(r['has_src']):<10}")
