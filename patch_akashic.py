import re

with open("experiments/chimera-lang/tests/akashic_test.rs", "r") as f:
    content = f.read()

content = content.replace(".chimera_akashic.json", ".chimera_akashic_test_manual.json")

with open("experiments/chimera-lang/tests/akashic_test.rs", "w") as f:
    f.write(content)
