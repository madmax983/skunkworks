with open('ARCHIVE.md', 'r') as f:
    content = f.read()

print("Condemned experiments:")
condemned = content.split('## ☠️ Condemned (Awaiting Execution)')[1].split('## Pardoned')[0]
print(condemned)
