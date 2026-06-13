import re

with open("experiments/git-cantata/src/main.rs", "r") as f:
    content = f.read()

content = re.sub(r'let args: Vec<String> = env::args\(\)\.collect\(\);\n    let args: Vec<String> = env::args\(\)\.collect\(\);', 'let args: Vec<String> = env::args().collect();', content)

with open("experiments/git-cantata/src/main.rs", "w") as f:
    f.write(content)
