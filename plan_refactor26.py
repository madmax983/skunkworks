# Verify it compiles!
import subprocess
import shutil
import os

shutil.copy("test_state.rs", "experiments/chimera-lang/src/tui/state.rs")
shutil.copy("test_navigation_clean.rs", "experiments/chimera-lang/src/tui/app/handlers/normal/navigation.rs")

res = subprocess.run(["cargo", "check", "-p", "chimera-lang", "--all-features"], capture_output=True, text=True)
if res.returncode == 0:
    print("Compilation successful!")
else:
    print("Compilation failed.")
    print(res.stderr)
