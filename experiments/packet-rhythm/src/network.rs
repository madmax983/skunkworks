use std::net::{TcpStream, ToSocketAddrs};
use std::time::{Duration, Instant};
use std::thread;
use crossbeam_channel::Sender;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TargetId(pub usize);

pub struct PingAgent {
    sender: Sender<(TargetId, Option<u128>)>,
}

impl PingAgent {
    pub fn new(sender: Sender<(TargetId, Option<u128>)>) -> Self {
        Self { sender }
    }

    pub fn ping(&self, target_id: TargetId, address: String) {
        let sender = self.sender.clone();
        thread::spawn(move || {
            // Resolve address
            let addr = match address.to_socket_addrs() {
                Ok(mut addrs) => addrs.next(),
                Err(_) => None,
            };

            if let Some(addr) = addr {
                let start = Instant::now();
                // Use connect_timeout
                let result = TcpStream::connect_timeout(
                    &addr,
                    Duration::from_millis(500), // 500ms timeout to keep rhythm tight
                );

                let elapsed = start.elapsed().as_millis();

                if result.is_ok() {
                    let _ = sender.send((target_id, Some(elapsed)));
                } else {
                    let _ = sender.send((target_id, None));
                }
            } else {
                 let _ = sender.send((target_id, None));
            }
        });
    }
}
