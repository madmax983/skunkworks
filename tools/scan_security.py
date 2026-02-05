import os
import re

KEYWORDS = [
    r"unsafe\s*\{",
    r"mem::transmute",
    r"static\s+mut",
    r"extern\s+\"C\"",
    r"no_mangle",
    r"from_raw_parts",
    r"std::ptr",
]

def scan_file(filepath):
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            lines = f.readlines()

        found = []
        for i, line in enumerate(lines):
            for keyword in KEYWORDS:
                if re.search(keyword, line):
                    found.append((i + 1, keyword, line.strip()))
        return found
    except Exception as e:
        print(f"Error reading {filepath}: {e}")
        return []

def main():
    print("Starting Security Scan...")
    vulns = {}

    for root, dirs, files in os.walk("."):
        if "target" in dirs:
            dirs.remove("target")
        if ".git" in dirs:
            dirs.remove(".git")

        for file in files:
            if file.endswith(".rs"):
                filepath = os.path.join(root, file)
                results = scan_file(filepath)
                if results:
                    vulns[filepath] = results

    if vulns:
        print(f"Found potential issues in {len(vulns)} files:")
        for filepath, issues in vulns.items():
            print(f"\n{filepath}:")
            for line_num, keyword, content in issues:
                print(f"  Line {line_num}: [{keyword}] {content}")
    else:
        print("No obvious security keywords found.")

if __name__ == "__main__":
    main()
