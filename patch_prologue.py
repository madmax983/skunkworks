import re

file_path = "experiments/chimera-lang/src/prologue_compiler.rs"
with open(file_path, "r") as f:
    content = f.read()

# Replace the specific lines inside definitions_section matching
search_block = """                            // Wrap content in a strand definition for the compiler
                            let wrapped_content =
                                format!("strand rune_{} {{ {} }}", custom_runes.len(), content);
                            let compiled_def =
                                crate::compiler::compile(&wrapped_content, base_path)?;"""

replace_block = """                            // The content already defines a strand (e.g., `strand alpha { ... }`)
                            let compiled_def =
                                crate::compiler::compile(content, base_path)?;"""

if search_block in content:
    content = content.replace(search_block, replace_block)
    with open(file_path, "w") as f:
        f.write(content)
    print("Patched successfully!")
else:
    print("Could not find block to patch")
