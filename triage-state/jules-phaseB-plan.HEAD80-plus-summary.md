AC == ec.residue: True 192 192
AC - extres: [] extres - AC: []
# Jules Phase B — Stage 1+2 Triage Plan (PLANNING ONLY)

Generated 2026-07-08 from jules-survivors.md (6,138 survivors). No GitHub writes performed.

## Summary

| Repo | Survivors | Stage1 closes | perf_micro | doc_sweep | Residue |
|---|---|---|---|---|---|
| abrash | 745 | 2 | 407 | 10 | 326 |
| scale | 669 | 5 | 12 | 7 | 645 |
| arthropod | 629 | 15 | 60 | 5 | 549 |
| orpheus | 484 | 3 | 84 | 11 | 386 |
| doom-rs | 432 | 3 | 78 | 13 | 338 |
| nes | 406 | 0 | 83 | 9 | 314 |
| skunkworks | 376 | 6 | 22 | 6 | 342 |
| hitz | 340 | 0 | 68 | 3 | 269 |
| glossa | 338 | 1 | 37 | 4 | 296 |
| duke | 325 | 0 | 54 | 6 | 265 |
| aletheiadb | 317 | 4 | 68 | 22 | 223 |
| logos | 303 | 2 | 31 | 7 | 263 |
| autumn | 248 | 0 | 31 | 6 | 211 |
| autumn-harvest | 216 | 1 | 40 | 7 | 168 |
| relvar | 200 | 7 | 32 | 4 | 157 |
| rust-interview-practice | 110 | 8 | 43 | 0 | 59 |
| **TOTAL** | **6138** | **57** | **1150** | **120** | **4811** |

Total planned closes (stage1 + perf_micro + doc_sweep): **1327**; residue for Stage 3 review: **4811**.

## Largest Stage-1 fuzzy clusters

| Repo | Size | Kept PR | Closed PRs (sample) | Kept title | Sample closed title |
|---|---|---|---|---|---|
| arthropod | 9 | 3109 | [3092, 3077, 2657, 2631, 2503, 2440] | 📜 Codex: ADR 012 & Architecture Diagram Update | 📜 Codex: ADR 041 & Architecture Diagram Update |
| skunkworks | 6 | 3035 | [3034, 2950, 2946, 2941, 2631] | 📜 Codex: ADR 076 &amp; Architecture Diagram Update | 📜 Codex: ADR 012 &amp; Architecture Diagram Update |
| relvar | 4 | 1050 | [957, 823, 816] | 🗺️ Atlas: [architectural change] Fixing Public Module Leaks  | 🗺️ Atlas: [architectural change] Fixing public module leak i |
| scale | 3 | 2081 | [2079, 1575] | 📜 Codex: ADR 046 &amp; Architecture Diagram Update | 📜 Codex: ADR 043 &amp; Architecture Diagram Update |
| rust-interview-practice | 3 | 438 | [429, 418] | feat: add LeetCode 572 Subtree of Another Tree | feat: add LeetCode 572 (Subtree of Another Tree) |
| rust-interview-practice | 3 | 375 | [358, 349] | Add LeetCode 143 Reorder List | feat: add LeetCode 143 Reorder List |
| arthropod | 3 | 3108 | [3046, 2949] | ⚔️ Elenchus: flux_state Test Quality Audit | ⚔️ Elenchus: flux-state Test Quality Audit |
| aletheiadb | 2 | 3309 | [3277] | 🪶 Bard: [documentation update] Fix rustdoc warnings in db mo | 🎻 Bard: Fix rustdoc warnings in db module |
| aletheiadb | 2 | 3292 | [3284] | 🎻 Bard: [documentation update] Fix broken and redundant intr | 🎸 Bard: Fix broken and redundant intra-doc links |
| aletheiadb | 2 | 3289 | [3020] | ⚖️ Elenchus: query planner rules Test Quality Audit | ⚔️ Elenchus: `query::planner::rules` Test Quality Audit |
| aletheiadb | 2 | 3261 | [3247] | 🎻 Bard: [documentation update] Fix broken intra-doc links | 🎸 Bard: Fix broken intra-doc links |
| autumn-harvest | 2 | 897 | [888] | ⚒️ Forge: Remove redundant `.clone()` on `Copy` types | ⚒️ Forge: Remove redundant clone on Copy types |
| orpheus | 2 | 1303 | [1259] | 🎨 Mosaic: UI Polish for [PluginPattern] | 🎨 Mosaic: UI Polish for PluginPattern |
| orpheus | 2 | 1000 | [939] | 🎨 Mosaic: UI Polish for PluginPatternValue Explain | 🎨 Mosaic: UI Polish for [PluginPatternValue Explain] |
| orpheus | 2 | 909 | [861] | 🎨 Mosaic: UI Polish for [Plugin Pattern Explain] | 🎨 Mosaic: UI Polish for Plugin Pattern Explain |
| scale | 2 | 3859 | [3851] | feat(layer1): complete dead internet system | feat(layer3): complete 1110 dead internet system |
| scale | 2 | 3262 | [2211] | feat(layer1): complete bio digital ascendancy system | feat(layer1): complete bio-digital ascendancy system |
| scale | 2 | 1160 | [1155] | feat(layer1): implement hypno learning (Spec 255) | feat(layer1): implement Hypno-Learning (Spec 255) |
| skunkworks | 2 | 3180 | [2436] | 📜 Codex: ADR 077, ADR 078 &amp; Architecture Diagram Update | 📜 Codex: ADR 063 &amp; ADR 064 Architecture Diagram Update |
| logos | 2 | 1133 | [1044] | 🎨 Mosaic: UI Polish for analytics net-worth | 🎨 Mosaic: UI Polish for Analytics Net Worth |

## Residue breakdown by persona (per repo)

- **scale** (645): Jules (generic): 359, Integrator: 71, Builder: 54, Architect: 36, Designer: 25, Warden: 18, Razor: 18, Atlas: 17, Sentry: 12, Nova: 9, Forge: 8, Ludwig: 7, Echo: 3, Bard: 2, Mosaic: 2, Lore Master: 2, Codex: 2
- **arthropod** (549): Havoc: 84, Vantage: 64, Forge: 64, Warden: 56, Nova: 53, Razor: 45, Echo: 39, Core: 34, Atlas: 27, Mosaic: 25, Elenchus: 23, Sentry: 19, Bard: 7, Codex: 6, Jules: 2, docs: 1
- **orpheus** (386): Forge: 70, Mosaic: 69, Sentry: 62, Atlas: 53, Havoc: 38, Vantage: 37, Bard: 29, Nova: 25, Atlas?: 2, unknown: 1
- **skunkworks** (342): Reaper: 79, Splice: 64, Jules (generic): 34, Havoc: 27, Atlas: 26, Warden: 18, Sentry: 16, Prologue: 14, Mosaic: 12, Razor: 11, Genesis: 11, Forge: 10, Mycelium: 6, Echo: 6, Codex: 4, Bard: 3, Verge Computer: 1
- **doom-rs** (338): Forge: 80, Atlas: 64, Sentry: 42, Havoc: 39, Vantage: 33, Nova: 30, Bard: 24, Mosaic: 24, Jules: 1, chore: 1
- **abrash** (326): Nova: 85, Forge: 38, Razor: 33, Bolt: 33, Sentry: 27, Atlas: 27, Havoc: 25, Warden: 21, Mosaic: 18, (none): 13, Bard: 6
- **nes** (314): Forge: 84, Vantage: 41, Havoc: 35, Sentinel: 35, Nova: 31, Mosaic: 27, Echo: 21, Atlas: 18, Sentry: 16, Bolt: 4, Bard: 2
- **glossa** (296): Razor: 61, Nova: 41, Forge: 33, Mosaic: 32, Havoc: 30, Atlas: 24, Echo: 20, Warden: 20, Sentry: 15, Bard: 10, Codex: 9, test: 1
- **hitz** (269): Nova: 52, Warden: 39, Forge: 37, Vantage: 37, Atlas: 30, Sentry: 28, Havoc: 23, Mosaic: 15, Bard: 7, Bolt: 1
- **duke** (265): Forge: 80, Havoc: 61, Atlas: 38, Vantage: 31, Nova: 25, Sentry: 20, Jules (generic): 7, Bard: 3
- **logos** (263): Forge: 44, Vantage: 39, Nova: 39, Mosaic: 28, Havoc: 26, Echo: 23, Atlas: 21, Sentry: 20, Sentinel: 18, Bard: 4, Jules (generic): 1
- **aletheiadb** (223): Nova: 42, Razor: 30, Elenchus: 29, Vantage: 27, Atlas: 25, Havoc: 22, Sentry: 20, Echo: 14, Bard: 10, Sentry?: 1, Bolt: 1, Elenchus?: 1, unknown: 1
- **autumn** (211): Nova: 33, Forge: 30, Havoc: 29, Atlas: 24, Warden: 21, Sentinel: 21, Echo: 20, Eris: 18, Sentry: 10, unknown: 2, =` CSS selectors | Sentry: 1, Bolt: 1, Bard: 1
- **autumn-harvest** (168): Nova: 48, Forge: 43, Warden: 34, Havoc: 29, Sentry: 12, Bolt: 1, Bard: 1
- **relvar** (157): Forge: 41, Warden: 29, Nova: 28, Atlas: 26, Razor: 21, Sentry: 7, Bard: 5
- **rust-interview-practice** (59): (none): 59

## Spot-check findings (read-only GitHub MCP, 19 calls)

### perf_micro closes — 8 samples across 8 repos
Benchmark-evidence rate: **0/8 (0%)** carried before/after benchmark numbers; **1/8 (12.5%)**
(abrash#2244) added a criterion bench file but reported no numbers. Below the 25% flag
threshold — the perf_micro category rule is NOT too aggressive on benchmark-evidence grounds.
Typical bodies say "Run cargo bench to verify" / "tests verify no regression" (nes#1129,
logos#828, aletheiadb#3035, glossa#1354, relvar#804) — speculative Vec::with_capacity /
# Phase B plan reconciliation — open-state refresh

Refreshed via GitHub API on 2026-07-08 (after external closes observed ~16:48-16:53Z in madmax983/autumn).

| repo | open PRs (total) | planned closes still open | residue still open | externally closed (planned) | externally closed (residue) | externally closed (total) |
|---|---|---|---|---|---|---|
| aletheiadb | 346 | 94/94 | 223/223 | 0 | 0 | 0 |
| autumn-harvest | 219 | 48/48 | 168/168 | 0 | 0 | 0 |
| orpheus | 486 | 98/98 | 386/386 | 0 | 0 | 0 |
| autumn | 28 | 0/37 | 19/211 | 37 | 192 | 229 |
| scale | 670 | 24/24 | 645/645 | 0 | 0 | 0 |
| skunkworks | 375 | 34/34 | 341/342 | 0 | 1 | 1 |
| logos | 303 | 40/40 | 263/263 | 0 | 0 | 0 |
| duke | 325 | 60/60 | 265/265 | 0 | 0 | 0 |
| abrash | 745 | 419/419 | 326/326 | 0 | 0 | 0 |
| rust-interview-practice | 110 | 51/51 | 59/59 | 0 | 0 | 0 |
| relvar | 200 | 43/43 | 157/157 | 0 | 0 | 0 |
| nes | 406 | 92/92 | 314/314 | 0 | 0 | 0 |
| glossa | 338 | 42/42 | 296/296 | 0 | 0 | 0 |
| arthropod | 629 | 80/80 | 549/549 | 0 | 0 | 0 |
| hitz | 340 | 71/71 | 269/269 | 0 | 0 | 0 |
| doom-rs | 432 | 94/94 | 338/338 | 0 | 0 | 0 |

**Totals:** 5952 open PRs across 16 repos; externally closed: 37 planned + 193 residue = 230.

Notes:
- "Externally closed" = PR number present in the Phase B plan but no longer open on GitHub at refresh time.
- Phase A closes (1,955 PRs) are not in the Phase B plan, so they cannot appear here; any plan number now closed is the external actor or drift.
- Open lists include only plan intersection; repos may have additional open PRs not in the plan (e.g., created after the plan snapshot).
