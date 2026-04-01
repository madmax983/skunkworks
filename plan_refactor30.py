# Let's run git log to see if the test was just recently modified by someone else or was already broken.
import subprocess
print(subprocess.run(["git", "log", "-1", "--oneline", "experiments/chimera-lang/tests/havoc_hash_overflow.rs"], capture_output=True, text=True).stdout)
