with open("experiments/neuro-git/src/main.rs", "r") as f:
    content = f.read()

# Fix clippy warnings
content = content.replace("for i in 0..num_neurons {\n                                if rng.gen_bool(0.3) {\n                                    ext_inputs[i] += insertion_current;\n                                }", "for input in ext_inputs.iter_mut() {\n                                if rng.gen_bool(0.3) {\n                                    *input += insertion_current;\n                                }")
content = content.replace("for i in 0..num_neurons {\n                                if rng.gen_bool(0.3) {\n                                    ext_inputs[i] -= deletion_current;\n                                }", "for input in ext_inputs.iter_mut() {\n                                if rng.gen_bool(0.3) {\n                                    *input -= deletion_current;\n                                }")

with open("experiments/neuro-git/src/main.rs", "w") as f:
    f.write(content)
