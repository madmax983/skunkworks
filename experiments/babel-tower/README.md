# 🧬 Babel Tower

> "Come, let us go down and confuse their language so they will not understand each other."

A hybrid experiment combining **Thread Frequency** and **Glossolalia**.

## 🔬 Concept

Several threads (Speakers) are attempting to recite the story of the Tower of Babel. However, they must contend for a single shared resource (the `Tower`) to speak a word.

The **Wait Time** for the lock determines the "Clarity" of the speech:
- **Short Wait**: The word is spoken clearly.
- **Long Wait**: The word undergoes **Phonological Evolution** (Grimm's Law, Vowel Shifts, Palatalization). The more contention, the more the language drifts into gibberish.

The result is a visual and sonic representation of race conditions and lock contention.

## 🧬 Lineage

- **Parent A**: `experiments/thread-frequency`
  - *Traits Inherited*: Thread-based musicians, audio rhythm, lock contention simulation.
- **Parent B**: `experiments/glossolalia`
  - *Traits Inherited*: Phonological evolution engine, linguistic mutation rules.

## 🧪 Usage

```bash
cargo run -p babel-tower
```

- **Top Pane**: The Transcript. Watch as the text becomes more garbled when threads fight for the lock.
- **Bottom Pane**: Speaker Stats. See which thread is waiting the longest.
- **Audio**: (Optional) Hear the contention. Distorted tones indicate high wait times.

## 🔮 Observations

- With 8 threads, contention is moderate. The text starts clear but occasionally drifts.
- The "Intensity" of the mutation is directly proportional to the `wait_time` for the Mutex.

## 📜 License

MIT
