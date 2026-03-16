import os

with open("MUTATIONS.md", "r") as f:
    mutations = f.read()

for root, dirs, files in os.walk("experiments"):
    for d in dirs:
        if "-" in d and d not in mutations:
             # Look for "A hybrid experiment splicing" in README
             readme_path = os.path.join(root, d, "README.md")
             if os.path.exists(readme_path):
                  with open(readme_path, "r") as r:
                       if "Parent A" in r.read():
                            print(d)

for root, dirs, files in os.walk("graveyard"):
    for d in dirs:
        if "-" in d and d not in mutations:
             # Look for "A hybrid experiment splicing" in README
             readme_path = os.path.join(root, d, "README.md")
             if os.path.exists(readme_path):
                  with open(readme_path, "r") as r:
                       if "Parent A" in r.read():
                            print(d)
