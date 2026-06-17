import re

files_to_check = [
    "experiments/thermo-market/src/world.rs",
    "experiments/chimera-lang/src/matrix_rain.rs",
    "experiments/chimera-lang/src/vm/chimera_chaos.rs",
    "experiments/chimera-lang/src/vm/neuron.rs",
    "experiments/chimera-lang/src/vm/nova_fractal.rs",
    "experiments/chimera-lang/src/vm/blackbox.rs",
    "experiments/chimera-lang/src/vm/nova_garden.rs",
    "experiments/chimera-lang/src/vm/nova_egregore.rs",
    "experiments/chimera-lang/src/vm/retina.rs",
    "experiments/chimera-lang/src/vm/nova_attractor.rs",
    "experiments/chimera-lang/src/vm/akashic.rs",
    "experiments/chimera-lang/src/vm/nova_biomesh.rs",
    "experiments/chimera-lang/src/vm/nova_arcana.rs",
    "experiments/chimera-lang/src/vm/babel_chaos.rs",
    "experiments/chimera-lang/src/vm/cladistics.rs",
    "experiments/chimera-lang/src/vm/prologue/state.rs",
    "experiments/chimera-lang/src/vm/prologue/alchemist.rs",
    "experiments/chimera-lang/src/vm/prologue/logos.rs",
    "experiments/chimera-lang/src/vm/prologue/oneiric.rs",
    "experiments/chimera-lang/src/vm/prologue/hyper.rs",
    "experiments/spqr-rsa/src/roman.rs",
    "experiments/hyper-flock/src/main.rs",
    "experiments/cosmic-strings/src/render.rs",
    "experiments/heap-arena/src/game.rs",
    "experiments/harmonic-engine/src/physics.rs",
    "experiments/dependency-karst/src/tui.rs",
    "experiments/dependency-karst/src/simulation.rs",
    "experiments/system-attractor/src/simulation.rs",
    "experiments/cargo-rocket/src/physics.rs",
    "experiments/phonetic-flock/src/phonology.rs",
    "experiments/git-cantata/src/audio.rs",
    "crates/neuro-sim/src/physics.rs",
    "crates/hyper-system/src/monitor.rs"
]

pattern = re.compile(r"impl Default for (\w+)\s*\{\s*fn default\(\)\s*->\s*Self\s*\{\s*Self::new\(\)\s*\}\s*\}", re.MULTILINE)
new_pattern = re.compile(r"pub fn new\(\)\s*->\s*Self\s*\{([^\}]*)\}")

for file in files_to_check:
    with open(file, 'r') as f:
        content = f.read()

    matches = pattern.finditer(content)
    for match in matches:
        struct_name = match.group(1)
        # Check if the new method has default fields.
        new_match = new_pattern.search(content)
        if new_match:
            new_impl = new_match.group(1)
            print(f"File: {file}, Struct: {struct_name}")
            print(new_impl.strip())
            print("---")
