---

# Echo's DX Audit Log 🗣️

**Target:** `experiments/chimera-lang/README.md`
**Date:** 2026-07-11

## 🔍 Experience - The Walkthrough

**Scenario:** "I am a new user trying to add `Nova`'s story feature."
**Action:** Try to use the API based *only* on the public docs/examples.

## 🚧 Stumble - The Friction Points

1.  **Missing Feature:** Tried to run the `story_demo`. Compiler said `NarrativeGenerator` not found.
    - *Impact:* Compilation error (`use of undeclared type or module NarrativeGenerator`).
    - *Fix:* Enable the `nova` feature, which is required for this module.

## 📢 Report - The Complaint

**Title:** 🗣️ Echo: Getting Started example is broken

**Description:**
*   🤦 **The Confusion:** "Tried to run the `story_demo`. Compiler said `NarrativeGenerator` not found."
*   🕵️ **The Reality:** "Turns out I needed to enable feature `nova`."
*   💡 **The Fix:** "Add a huge banner in README saying 'REQUIRES FEATURE NOVA'."
