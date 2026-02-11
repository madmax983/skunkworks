# Packet Rhythm ⚛️🥁

**"The Internet is a Drum Machine"**

A Moonshot experiment that maps network latency to rhythmic patterns.

## Concept
- **Network Packet Timing + Rhythmic Patterns**
- Instead of a perfect metronome, the rhythm is governed by the Round Trip Time (RTT) of TCP packets sent to various servers.
- **Distance = Latency**: In the visualization, the distance of a node from the center is proportional to its latency.
- **Audio Event = Packet Return**: The drum sound triggers when the packet *returns*, not when it is sent. This means high latency creates a "lazy" beat, and jitter creates "swing".

## Usage
Run with default features (Visuals only):
```bash
cargo run -p packet-rhythm
```

Run with Audio (Requires ALSA/Sound drivers):
```bash
cargo run -p packet-rhythm --features audio
```

## How it Works
1.  **Sequencer**: A metronome triggers pings to target hosts (Google, Cloudflare, etc.) on a rhythmic grid.
2.  **PingAgent**: Asynchronously connects to the target (TCP Handshake) to measure RTT.
3.  **Visuals**: A projectile is launched. Its travel time is calibrated to the expected RTT.
4.  **Feedback**: When the ping returns, the target node flashes, updates its radius (latency), and triggers a sound.

## Controls
- Watch and Listen.
- **Kick**: Google DNS
- **Snare**: Cloudflare
- **HiHat**: GitHub
- **Bass**: Localhost
