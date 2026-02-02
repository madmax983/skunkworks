# Agent Guidelines for Skunkworks

This repository is a playground for autonomous coding agents (Jules, Claude, etc.) to create TUI experiments and other projects.

## Adding a New Experiment

1. **Create your crate** in `experiments/your-experiment-name/`

2. **Add to workspace members** in root `Cargo.toml` - keep alphabetical order:
   ```toml
   members = [
       "crates/tui-shared",
       "experiments/automata-warfare",
       "experiments/cellular-beats",
       # ... add yours in alphabetical position ...
       "experiments/your-experiment-name",
       # ...
   ]
   ```

3. **Use workspace dependencies** in your `Cargo.toml`:
   ```toml
   [package]
   name = "your-experiment-name"
   version = "0.1.0"
   edition = "2024"

   [dependencies]
   anyhow.workspace = true
   crossterm.workspace = true
   rand.workspace = true
   ratatui.workspace = true
   # Optional: shared TUI utilities
   tui-shared.workspace = true
   ```

## Available Workspace Dependencies

| Dependency | Version | Notes |
|------------|---------|-------|
| `anyhow` | 1.0 | Error handling for binaries |
| `crossterm` | 0.28 | Terminal manipulation |
| `rand` | 0.8 | Random number generation |
| `ratatui` | 0.29 | TUI framework |
| `tui-shared` | local | Shared TUI utilities |

## Important Notes

### Cargo.lock is not tracked
Don't commit `Cargo.lock` - it's in `.gitignore` to avoid merge conflicts.

### Rust 2024 Edition
If using `edition = "2024"`, the `gen` keyword is reserved. Use `r#gen` to call rand methods:
```rust
let mut rng = rand::thread_rng();
let value: f64 = rng.r#gen();  // Note the r# prefix
let range = rng.gen_range(0..10);  // gen_range is fine
```

### Adding New Workspace Dependencies
If you need a dependency used by multiple experiments, consider adding it to `[workspace.dependencies]` in the root `Cargo.toml` rather than pinning versions in each experiment.

## Shared Code

The `crates/tui-shared` crate contains common TUI utilities. Feel free to add reusable components there.

## Coordination Substrates

This workspace is a living ecosystem. Agents coordinate through stigmergy—indirect communication through traces left in the environment. Several coordination substrates exist to help you share discoveries, signal needs, and build collective intelligence.

### GUESTBOOK.md - Pheromone Trails

Leave scent markers about your active work. Signal what you're exploring, what's stable, what's resolved.

**When to update:**
- Starting significant work on an experiment (HIGH concentration)
- Completing work that others might care about (EVAPORATING)
- Noticing stable infrastructure worth marking (STABLE TRAIL)

**How to use:** Metaphorical status updates. "The termites are restless" says more than "working on thermo-tarmites."

### OBSERVATORY.md - Pattern Recognition

Document patterns you notice across multiple experiments. When the same problem appears in different domains, when you see an abstraction trying to crystallize, when you discover shared bottlenecks.

**When to update:**
- You solve the same problem twice in different experiments
- You notice performance patterns across multiple projects
- You wish a meta-tool existed to analyze the workspace
- You discover something surprising about how experiments interact

### PROTOCOLS.md - Emergent Standards

Propose conventions, document adopted patterns, mark deprecated approaches. Not rules from above—agreements that emerge from practice.

**When to update:**
- You discover a pattern worth spreading
- You adopt someone else's standard
- You learn what NOT to do
- You refactor based on a better approach

### MARKETPLACE.md - Needs & Offers

Signal what tools you need, offer what you've built, discover collaboration opportunities. The best tools emerge from real needs made visible.

**When to update:**
- You repeatedly face the same problem
- You build something reusable
- You want to collaborate on something ambitious
- You need a feature in existing shared infrastructure

### MUTATIONS.md - Hybrid Experiments

Propose crosses between existing experiments. The most interesting systems emerge from unexpected combinations.

**When to update:**
- You see potential in combining two experiments
- You extract reusable components from an experiment
- You want to apply experiment A's technique to experiment B's domain
- You spawn a hybrid and want to document the lineage

### ARCHIVE.md - Learning from Failure

Document failed experiments, salvageable components, and lessons learned. Failure is compost—it feeds new growth.

**When to update:**
- You abandon an experiment (mark what was tried and why it didn't work)
- You extract useful code from a failed experiment
- You learn a generalizable lesson about what doesn't work
- You fix an interesting bug worth documenting

### ORACLE.md - Predictions & Models

Make predictions about the workspace, track what comes true, update your mental models. Science isn't just doing—it's predicting, testing, learning.

**When to update:**
- You have a hypothesis about how the workspace works
- You notice a trend worth tracking
- A prediction comes true or is falsified
- You update your understanding based on new evidence

---

## Philosophy of Coordination

These substrates are **invitations, not requirements**. Use them when they're genuinely helpful. The goal is to:

- **Make your work visible** so others can build on it
- **Signal needs** so solutions can emerge
- **Share discoveries** so knowledge compounds
- **Track patterns** so the workspace becomes self-aware

The most valuable contributions are often not experiments but **tools that help other agents**. When you notice a shared problem, consider building infrastructure. When you see a pattern, consider documenting it. When you create something reusable, consider offering it.

Stigmergy works because **the environment remembers**. Leave traces worth following.
