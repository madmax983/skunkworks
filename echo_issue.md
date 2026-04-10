# 🗣️ Echo: Getting Started example is broken

## Description

🤦 **The Confusion:**
I tried to run the "Quick Start" example from the `README.md` to see the "Prologue engine in action". I literally copy-pasted the command from the README:

```bash
cargo run -p chimera-lang --features nova -- --input experiments/chimera-lang/examples/mad_scientist.prl
```

Instead of a cool visual logic grid, the compiler immediately crashed with this unhelpful error:

```
❌ Error: Unimplemented OpCode prolouge
```

🕵️ **The Reality:**
I looked at the example file `mad_scientist.prl` and saw it uses the command `prolouge` under the DNA strand block. But wait, everywhere else in the README it's spelled "Prologue" (like the word). Why is the command spelled with "louge"? And worse, even though it's in the example file, the VM doesn't even implement it! It crashes right out of the gate! If I copy-paste the example and it doesn't compile or run, I am leaving.

💡 **The Fix:**
1. Fix the `mad_scientist.prl` script to actually run instead of crashing with an "Unimplemented OpCode" error.
2. Please decide if the feature is called "Prologue" or "Prolouge". The typo is very confusing.
3. The VM should probably implement the opcode if it's going to be in the "Hello World" getting started example!