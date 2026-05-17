with open("Cargo.toml", "r") as f:
    text = f.read()

# I also need to exclude poincare-resonance from testing as it requires X11, unless we handle it properly. Actually we added the --headless flag to it successfully.

with open("Cargo.toml", "w") as f:
    f.write(text)
