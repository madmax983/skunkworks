sed -i '1i //! # Narrative API\n//!\n//! Provides a fluent builder pattern to simplify the construction of a `ChimeraVM`\n//! populated with narrative elements, logic grids, and genes.\n//!\n//! This module abstracts away the low-level complexities of raw grid and AST manipulation.' experiments/chimera-lang/src/narrative.rs

cat << 'EOF2' > fix_bin_docs.py
import os

def fix_bin_docs():
    for root, dirs, files in os.walk("experiments"):
        if "src" in dirs:
            main_path = os.path.join(root, "src", "main.rs")
            readme_path = os.path.join(root, "README.md")

            if os.path.exists(main_path) and os.path.exists(readme_path):
                with open(main_path, "r") as f:
                    content = f.read()

                if not content.startswith("//!"):
                    with open(readme_path, "r") as f:
                        readme = f.read()

                    doc_comments = "\n".join([f"//! {line}" if line.strip() else "//!" for line in readme.split("\n")]) + "\n"

                    with open(main_path, "w") as f:
                        f.write(doc_comments + content)
                        print(f"Fixed {main_path}")

fix_bin_docs()
EOF2
python3 fix_bin_docs.py

cat << 'EOF4' > .jules/bard.md.new
## 2026-07-06 - [Module-Level Docs and README Sync for Executables]
**Confusion:** Building documentation for a binary crate using strict rustdoc flags (`-W rustdoc::missing_crate_level_docs -D warnings`) will fail if the `src/main.rs` file does not include a `//!` crate-level doc comment block.
**Clarification:** To satisfy `cargo doc` for binary crates, parse the `README.md` contents and inject them as `//!` block comments at the very top of `src/main.rs`. This ensures the overarching story for the executable is documented and the documentation build passes.

EOF4
cat .jules/bard.md >> .jules/bard.md.new
mv .jules/bard.md.new .jules/bard.md
