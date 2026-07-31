**Esolang Evolution**
**Learning:** Integrated PrologueEsolang (Mad Scientist Mode) and Orca directly into the `chimera-esolang` grammar, bridging the standalone frontend with the core VM's esoteric capabilities. The `operator` rule was placed before `identifier` in the PEG grammar to correctly map literal strings to operator enums.
**Action:** Mapped `"madness"`, `"prologue"`, and `"orca"` operators in `chimera-esolang` to `OpCode::PrologueEsolang`, `OpCode::Prologue`, and `OpCode::Orca`.
