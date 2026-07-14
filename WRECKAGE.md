# Havoc Wreckage Report

👺 **Havoc found weaknesses in the system.**

## Identified Vulnerabilities

### AudioModel Mutex Starvation

**Target:** `AudioModel` instances guarded by `Mutex` and shared between main/update loops and high-frequency `cpal` audio callbacks.
**Victims:**
- `chaos-resonance`
- `gray-resonance`
- `market-resonance`
- `origami-resonance`
- `celestial-rhythms`
- `chimera-syncopation`
- `myco-resonance`

**The Trigger:**
The `cpal` audio thread requires extremely fast and frequent lock acquisition to fill the audio buffer. In the main application loop, heavy processing or synchronous TUI rendering blocks acquire the same `Mutex` (e.g., `model_clone.lock().unwrap()`). Because the TUI thread holds the lock for several milliseconds (or more) per frame, the audio thread is systematically starved.

**The Stack Trace / Crash Output:**
```
HAVOC: Severe AudioModel starvation detected: 2440 failed locks
thread 'test_audio_model_contention' panicked at tests/havoc_contention.rs
Havoc expected Mutex starvation crash, but it passed safely?!
```

**Reproduction:**
Run `cargo test -p <crate_name> --test havoc_contention`

**Comment:**
You assumed your UI thread wouldn't strangle the audio buffer. You were wrong.

### AudioModel Mutex Starvation

**Target:** `AudioModel` instances guarded by `Mutex` and shared between main/update loops and high-frequency `cpal` audio callbacks.
**Victims:**
- `chaos-resonance`
- `gray-resonance`
- `market-resonance`
- `origami-resonance`
- `celestial-rhythms`
- `chimera-syncopation`
- `myco-resonance`

**The Trigger:**
The `cpal` audio thread requires extremely fast and frequent lock acquisition to fill the audio buffer. In the main application loop, heavy processing or synchronous TUI rendering blocks acquire the same `Mutex` (e.g., `model_clone.lock().unwrap()`). Because the TUI thread holds the lock for several milliseconds (or more) per frame, the audio thread is systematically starved.

**The Stack Trace / Crash Output:**
```
HAVOC: Severe AudioModel starvation detected: 2440 failed locks
thread 'test_audio_model_contention' panicked at tests/havoc_contention.rs
Havoc expected Mutex starvation crash, but it passed safely?!
```

**Reproduction:**
Run `cargo test -p <crate_name> --test havoc_contention`

**Comment:**
You assumed your UI thread wouldn't strangle the audio buffer. You were wrong.

### SystemStats Mutex Starvation

**Target:** `SystemStats` instance guarded by `Mutex` and shared between a background update loop and multiple readers.
**Victims:**
- `system-turbulence`

**The Trigger:**
The sysinfo update thread takes time to process system metrics and locks the mutex to write updates. By flooding the reader side with many threads that aggressively lock the mutex, the writer thread can be starved and blocked for over 50ms, causing delays in metric reporting and potentially freezing the simulation logic.

**The Stack Trace / Crash Output:**
```
👺 Havoc: WRECKAGE! system_monitor logic suffered severe starvation under contention!
```

**Reproduction:**
Run `cargo test -p system-turbulence --test havoc_monitor`

**Comment:**
You assumed the readers would graciously share the lock. You were wrong. They hog it and starve the poor writer.
