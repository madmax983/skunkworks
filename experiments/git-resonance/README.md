# Git Resonance 🔔

**Parents:** `git-associates` × `resonance-audio`
**Brought by:** Vesper, The Night Listener 🦇 — *"Strike it gently. Listen honestly."*

## Concept

A repository, rung like a bell.

`git-associates` supplies the past: commits with hashes, parent counts, and
churn. `resonance-audio` supplies the physics of hearing: an FDTD wave grid
where energy propagates, reflects, and interferes. Neither parent can do
what the hybrid does — a diff cannot ring, and an empty grid has nothing to
say.

## How a commit becomes a note

- **Position** — from the first four hex digits of the commit hash, mapped
  into the grid. Deterministic: the same commit always rings in the same
  place.
- **Strength** — from churn (insertions + deletions) on a sqrt curve: a
  one-line fix whispers at ~0.4, a 400-line refactor rings at ~1.3, nothing
  exceeds 2.6.
- **Merges strike twice** — once at their hash position, once at the center,
  because two histories arriving at once is a different kind of event.
- **Order** — oldest first. History is a story; you ring it forward.

A listener at the grid's center records only what physically arrives at its
own cell: peak amplitude, total energy, and a 16-point sample of the decay
after the final commit — the **chord fingerprint**.

## Phenotype (and how to falsify it)

The fingerprint belongs to the history that produced it. Run two different
repositories — or one repository at two different eras — and the chords
differ. The claim is **falsified** if materially different histories produce
the same fingerprint; a unit test (`different_histories_different_chords`)
guards a small version of exactly that.

## Usage

```bash
# TUI — rings the current repository, commit ticker at the bottom
cargo run -p git-resonance

# Headless — prints the ledger, no terminal touched
cargo run -p git-resonance -- --headless . 120
cargo run -p git-resonance -- --headless /path/to/other/repo 200
```

If the path won't open as a repository, a twelve-note synthetic demo
history is rung instead and the ledger says so.

Controls: `r` ring it again · `q` quit. The listener is the white `+`.
