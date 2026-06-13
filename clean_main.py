with open("experiments/git-cantata/src/main.rs", "r") as f:
    content = f.read()

content = content.replace("    let args: Vec<String> = env::args().collect();\n    let path = if args.len() > 1 && args[1] != \"--headless\" { &args[1] } else { \".\" };", "    let path = if args.len() > 1 && args[1] != \"--headless\" { &args[1] } else { \".\" };")

with open("experiments/git-cantata/src/main.rs", "w") as f:
    f.write(content)
