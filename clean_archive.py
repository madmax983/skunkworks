with open("ARCHIVE.md", "r") as f:
    content = f.read()

import re
content = re.sub(r'\n{3,}', '\n\n', content)

with open("ARCHIVE.md", "w") as f:
    f.write(content)
