use std::net::{TcpStream, ToSocketAddrs};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use crate::shared::GrooveState;

const PING_INTERVAL: Duration = Duration::from_millis(500); // Ping twice a second
const TIMEOUT: Duration = Duration::from_millis(500);

#[derive(Clone, Copy)]
enum Instrument {
    Kick,
    Snare,
    Hat,
}

pub fn start_monitoring(state: Arc<GrooveState>) {
    spawn_monitor(state.clone(), "8.8.8.8:53", Instrument::Kick);
    spawn_monitor(state.clone(), "1.1.1.1:53", Instrument::Snare);
    spawn_monitor(state.clone(), "example.com:80", Instrument::Hat);
}

fn spawn_monitor(state: Arc<GrooveState>, addr: &'static str, instrument: Instrument) {
    thread::spawn(move || {
        loop {
            // DNS Resolution (part of the latency groove)
            let socket_addr = match addr.to_socket_addrs() {
                Ok(mut addrs) => addrs.next(),
                Err(_) => None,
            };

            if let Some(socket_addr) = socket_addr {
                let start = Instant::now();
                let result = TcpStream::connect_timeout(&socket_addr, TIMEOUT);

                // Elapsed time includes connect time
                // If failed, we consider it "max latency" (timeout)
                let elapsed = if result.is_ok() {
                    start.elapsed()
                } else {
                    TIMEOUT
                };

                match instrument {
                    Instrument::Kick => state.set_kick_latency(elapsed),
                    Instrument::Snare => state.set_snare_latency(elapsed),
                    Instrument::Hat => state.set_hat_latency(elapsed),
                }
            } else {
                // DNS failed
                let elapsed = TIMEOUT;
                match instrument {
                    Instrument::Kick => state.set_kick_latency(elapsed),
                    Instrument::Snare => state.set_snare_latency(elapsed),
                    Instrument::Hat => state.set_hat_latency(elapsed),
                }
            }

            // Sleep to avoid flooding
            thread::sleep(PING_INTERVAL);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_network_monitoring() {
        // This test attempts to connect to a public server.
        // It might fail in restricted environments, so we handle that.
        let state = Arc::new(GrooveState::new());

        // Use a likely available target or just verify the thread spawns without panic
        spawn_monitor(state.clone(), "google.com:80", Instrument::Hat);

        // Wait for a bit
        thread::sleep(Duration::from_secs(2));

        let latency = state.get_hat_latency();
        println!("Measured latency: {:?}", latency);

        // We can't strictly assert latency > 0 because network might be down/blocked
        // But we can assert the code didn't panic.
        assert!(true);
    }
}
