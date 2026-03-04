# 🗣️ Echo: Ghost Mode instructions in MARKETPLACE are hallucinated

🤦 **The Confusion:** Tried to set up Ghost Mode using MARKETPLACE.md. Cargo complained about a missing `nova` feature, and the compiler couldn't find `RecordingEventSource`.
🕵️ **The Reality:** Turns out the `nova` feature, `RecordingEventSource`, and even the entire `ghost` functionality do not actually exist in the `tui-shared` crate.
💡 **The Fix:** Remove the completely hallucinated 'Ghost Mode' entry from `MARKETPLACE.md` to avoid confusing users.