# 🧬 BioCoin: Living Blockchain

**A blockchain where validators are living organisms that evolve.**

## Concept

Traditional blockchains use Proof-of-Work or Proof-of-Stake. BioCoin uses **Proof-of-Life**:

- **Validators = Chimera organisms** with DNA encoding validation logic
- **Consensus = Chemical signaling** via hormone secretion
- **Security = Natural selection** - honest validators reproduce, malicious ones starve
- **Finality = Evolution** - dead validators can't revert history

## Features

### Biological Consensus
- Validators vote by secreting hormones on channels (block hashes)
- Consensus reached when hormone level exceeds threshold
- Asynchronous, weighted voting with natural timeouts (hormone decay)

### Natural Selection
- Successful validators earn fees → gain energy → reproduce (mitosis)
- Failed validators lose energy → starve → die (apoptosis)
- DNA mutations during reproduction create genetic variation (5% rate)
- Network evolves toward optimal validation strategies through genetic drift

### Byzantine Fault Tolerance via Evolution
- Malicious validators waste energy on invalid votes
- Honest validators earn fees and reproduce
- Population naturally trends toward honesty over time

## Running

### Evolution Demo (Text Output)
```bash
cargo run --release
```

### Byzantine Fault Tolerance Test
```bash
cargo run --release -- --byzantine
```

Runs a test with 60% honest + 40% malicious validators.
Watch evolution eliminate the Byzantine actors!

### Live TUI Visualization
```bash
cargo run --release -- --ui
```

Watch in real-time:
- Population evolution (honest vs malicious)
- Energy dynamics
- Validator reproduction and death
- Block finalization
- Natural selection in action

**Controls:**
- `SPACE`: Pause/Resume
- `Q`: Quit

## Architecture

See `ARCHITECTURE.md` for detailed design.

## Status

**Working prototype with proven Byzantine fault tolerance.**

Implemented features:
- [x] Architecture design (ARCHITECTURE.md)
- [x] Validator organisms (ChimeraVM with DNA)
- [x] Chemical voting (hormone-based consensus)
- [x] Natural selection (mitosis/apoptosis)
- [x] DNA mutations (genetic variation during reproduction)
- [x] Byzantine fault tolerance (proven via evolution)
- [x] TUI visualization (real-time population tracking)

## Research Goals

Can biological consensus actually work? Can evolution create Byzantine fault tolerance?

**Let's find out.** 🧬⛓️
