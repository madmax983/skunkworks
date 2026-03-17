# Git DDoS 🧬

**Parents**: `experiments/git-harmonograph` × `experiments/locust-ddos`

A hybrid experiment creating **Codebase Cyberwarfare**.

## 🔬 Concept

The simulation acts as a visual representation of codebase churn. The swarm of DDoS packets targets the "hotspots" of developer activity in the local git repository. Active files (with high commit counts) act as strong attractors (servers), drawing the botnet's fire. The packets swarm the codebase structure, breaking through simple defensive walls (firewalls).

## 🧬 Novel Trait

**Codebase Cyberwarfare:** The target of the swarm intelligence isn't a single arbitrary point, but a mathematically distributed representation of the Git repository's history. The more a file is changed, the stronger its gravitational pull on the attacking botnet.

## 🚀 Run

```bash
cargo run -p git-ddos
```

## 🎮 Controls

*   **View**: Watch the swarm intelligence attack the codebase topography.

## 📦 Lineage

*   **Git Harmonograph**: Provides the Git repository parsing and file metadata extraction.
*   **Locust DDoS**: Provides the Macroquad-based swarm intelligence and parallelized boid-like physics targeting and firewall degradation mechanics.
