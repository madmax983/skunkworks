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

                    doc_comments = "\n".join([f"//! {line}" for line in readme.split("\n")]) + "\n"

                    with open(main_path, "w") as f:
                        f.write(doc_comments + content)
                        print(f"Fixed {main_path}")

fix_bin_docs()
