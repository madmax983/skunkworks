import os
import re

def check_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()

    lines = content.split('\n')
    has_mod_doc = any(line.strip().startswith('//! ') for line in lines)
    if not has_mod_doc and ('pub struct' in content or 'pub fn' in content or 'pub trait' in content or 'pub enum' in content):
        print(f"{filepath} might be missing module-level docs (//! ).")

for root, dirs, files in os.walk('crates'):
    for file in files:
        if file.endswith('.rs'):
            check_file(os.path.join(root, file))
