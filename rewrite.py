import re

with open("crates/git-associates/src/lib.rs", "r") as f:
    content = f.read()

content = content.replace("""        for i in 0..diff.deltas().len() {
            let Some(patch) = git2::Patch::from_diff(diff, i).ok().flatten() else {
                continue;
            };""", """        for i in 0..diff.deltas().len() {
            // A Patch object lets us examine the hunks and lines of a delta
            let Ok(Some(patch)) = git2::Patch::from_diff(diff, i) else {
                continue;
            };""")

with open("crates/git-associates/src/lib.rs", "w") as f:
    f.write(content)
