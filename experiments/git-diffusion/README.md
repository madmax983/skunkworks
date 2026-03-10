# Git Diffusion

A hybrid of `experiments/git-harmonograph` and `crates/gray-scott`.

This experiment visualizes the Git commit history as a sequence of Turing patterns.
The hash of each commit is used to generate the `feed` and `kill` parameters of a
reaction-diffusion simulation.

As the commit history changes, the morphological patterns evolve—some commits produce
spots and corals, while others weave into stripes and mazes. The repository's history
becomes a biological footprint.

## Parents
* `experiments/git-harmonograph` - Provides the TUI wrapper and Git history scraping mechanism.
* `crates/gray-scott` - Provides the parallelized reaction-diffusion engine.

## Instructions
* Run with `cargo run -p git-diffusion`.
* Use Arrow Keys (`Left`/`Right`) to navigate commit history.
* Press `Space` to toggle Auto-Play.
* Press `R` to reset the chemical grid completely.
