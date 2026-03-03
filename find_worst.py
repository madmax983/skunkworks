import os

exp_dir = 'experiments'
candidates = []

for entry in os.listdir(exp_dir):
    full_path = os.path.join(exp_dir, entry)
    if os.path.isdir(full_path):
        readme_path = os.path.join(full_path, 'README.md')
        readme_lines = 0
        if os.path.exists(readme_path):
            with open(readme_path, 'r', encoding='utf-8') as f:
                readme_lines = len(f.readlines())

        src_dir = os.path.join(full_path, 'src')
        src_files = 0
        loc = 0
        if os.path.exists(src_dir):
            for root, _, files in os.walk(src_dir):
                for file in files:
                    if file.endswith('.rs'):
                        src_files += 1
                        with open(os.path.join(root, file), 'r', encoding='utf-8') as f:
                            loc += len(f.readlines())

        candidates.append((entry, readme_lines, src_files, loc))

# Sort by (readme_lines == 0), then lines in README, then LOC
candidates.sort(key=lambda x: (x[1] > 0, x[1], x[3]))
for c in candidates[:10]:
    print(c)
