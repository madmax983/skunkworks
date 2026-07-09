# Jules triage — merge-wave outcomes (2026-07-09)
Closes (agent-executed): 1,953 Phase A dedup + 1,572 Phase B triage + 653 scale close-all + 4 review closes = 4,182 total.
Merges: abrash 19 · autumn-harvest 8 (trunk-dev) · glossa 1 · logos 3 · hitz 4 · doom-rs 8 · nes 7 · duke 7 · orpheus 1 · relvar 2 = 60 merged by sessions. arthropod 11 (+optional #2206) and skunkworks #2943/#3133 (+close #3673) handed to the owner for manual one-click merge. hitz #928 (MMIO REX.W fix) newly authored, merge pending owner decision.
Excluded by owner instruction: autumn (5 keepers) and aletheiadb (34 keepers) — handled in other projects, PRs left open.
Known still-open bugs with no correct PR: abrash AVX2 culling bug (culls visible geometry); relvar image::apply_kernel convolution overflow; duke native.rs unclamped fields[1..=size] slice.
Residue: duplicates/borderline PRs deliberately left open everywhere except scale (closed wholesale, 653). Jules bot still actively generating new PRs as of 2026-07-09.

---

## This session — doom-rs / nes / orpheus / duke / relvar (verified merges)

All merges squash; `trunk` never force-pushed; each fix verified locally (`cargo fmt` + `clippy -D warnings` + tests) and, where CI exists, its checks were non-required advisory. Superseded/wrong-premise PRs were closed with a one-line comment; duplicates were left open (not closed).

### doom-rs — 8 merged, 0 closed
| PR# | outcome | commit |
|---|---|---|
| 645 | merged | 2e0b03f |
| 691 | merged | 0f5ee7b |
| 730 | merged | 45a88d0 |
| 785 | merged | 2556052 |
| 993 | merged | 5bd089d |
| 1065 | merged | 79be720 |
| 1223 | merged | 9553a83 |
| 1270 | merged | 4224ad9 |

Dups left open: #587, #724. Note: #1223 also cleared a pre-existing workspace clippy `-D warnings` failure; #691's bundled `from_name` `split_once('M')` was verified to be a harmless refactor (not a real panic fix); #785 is coverage, not a source bugfix.

### nes — 7 merged, 3 closed
| PR# | outcome | commit / reason |
|---|---|---|
| 827 | merged | 78a99e4 (bounded read_line OOM DoS) |
| 891 | merged | 9b13757 |
| 905 | merged | 6e6e4c9 |
| 913 | merged | 63917c1 |
| 953 | merged | af1b9b3 |
| 962 | merged | fa041be |
| 972 | merged | abb78c9 |
| 788 | closed | wrong-premise: wasm32-only, breaks clippy, bad wasm-bindgen-test pin |
| 935 | closed | targets dead orphan assembler.rs; live emit_expr_byte already correct |
| 960 | closed | empty branch; mcp-host gates already landed via #827 |

### duke — 7 merged, 1 closed
| PR# | outcome | commit / reason |
|---|---|---|
| 806 | merged | 7f1d50b (nova build/import fix) |
| 946 | merged | 85116ad (JDWP OOM DoS clamp) |
| 959 | merged | 9262d77 (JImageReader capacity-overflow) |
| 969 | merged | 2c99901 (regex char-boundary panic) |
| 996 | merged | 0adbc14 (String indexOf/lastIndexOf OOB) |
| 1045 | merged | f93558d (zip-bomb bypass) |
| 1052 | merged | 4453078 (empty-slice 1..=0 panic) |
| 725 | closed | superseded — trunk already fixes empty-telemetry formatting |

Out-of-scope follow-up (unfixed): `native.rs` bare unclamped `fields[1..=a_size]`/`[1..=b_size]` — same panic class as #1052 in a different function.

### orpheus — 1 merged
| PR# | outcome | commit |
|---|---|---|
| 1327 | merged | 5a2b101 (delay-buffer OOM cap; redundant VST3 hunk auto-dropped in rebase) |

Dups left open: #1316, #1319. Everything else in the report's ~9 was already absorbed by trunk / wrong-premise.

### relvar — 2 merged
| PR# | outcome | commit / reason |
|---|---|---|
| 928 | merged | dc80d16 (recursion stack-overflow DoS in Query/ConstraintExpression deserialization; chosen over #948) |
| 809 | merged | f762ee1 (RwLock-poisoning DoS in PersistentEngine) |
| 948 | left open | inferior duplicate of #928 (mirror-enum churn, junk file, 47% patch cov) |
| 938 | skipped | redundant coverage — target already covered on trunk |

Further recursion-DoS dups left open: #845, #867, #869, #878, #889, #918. Out-of-scope follow-up (unfixed): `image::apply_kernel` convolution overflow — no clean PR exists.
