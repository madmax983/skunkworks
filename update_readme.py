import re
import sys

def update_readme(readme_path, lib_rs_path):
    with open(lib_rs_path, 'r') as f:
        lib_content = f.read()

    # Extract module level docs
    module_docs = []
    for line in lib_content.splitlines():
        if line.startswith('//!'):
            # Strip the `//!` and leading space if it exists
            doc_line = line[3:]
            if doc_line.startswith(' '):
                doc_line = doc_line[1:]
            module_docs.append(doc_line)
        elif not line.strip() and len(module_docs) == 0:
            continue # ignore leading blank lines
        elif not line.startswith('//!') and len(module_docs) > 0:
            break

    module_docs_str = '\n'.join(module_docs)

    with open(readme_path, 'w') as f:
        f.write(module_docs_str)

update_readme('crates/locus/README.md', 'crates/locus/src/lib.rs')
update_readme('crates/resonance-audio/README.md', 'crates/resonance-audio/src/lib.rs')
update_readme('crates/ferrous-core/README.md', 'crates/ferrous-core/src/lib.rs')
update_readme('crates/tui-shared/README.md', 'crates/tui-shared/src/lib.rs')
update_readme('crates/quipu/README.md', 'crates/quipu/src/lib.rs')
