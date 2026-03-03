import os

exp_dir = 'experiments'
results = []

for entry in os.listdir(exp_dir):
    full_path = os.path.join(exp_dir, entry)
    if not os.path.isdir(full_path):
        continue

    readme_path = os.path.join(full_path, 'README.md')
    readme_len = 0
    if os.path.exists(readme_path):
        with open(readme_path, 'r', encoding='utf-8') as f:
            readme_len = len(f.read().strip())

    src_dir = os.path.join(full_path, 'src')
    loc = 0
    if os.path.exists(src_dir):
        for root, _, files in os.walk(src_dir):
            for file in files:
                if file.endswith('.rs'):
                    with open(os.path.join(root, file), 'r', encoding='utf-8') as f:
                        loc += len(f.readlines())

    results.append((entry, readme_len, loc))

# Sort primarily by lowest LOC, then by lowest README
results.sort(key=lambda x: (x[2], x[1]))

print("Smallest by LOC:")
for r in results[:10]:
    print(r)
