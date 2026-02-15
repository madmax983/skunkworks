use std::sync::mpsc::{Receiver, Sender};

#[derive(Debug, Clone)]
pub enum Event {
    Enter {
        id: usize,
        name: String,
        args: Vec<String>,
        parent_id: Option<usize>,
    },
    Exit {
        id: usize,
        result: String,
    },
}

pub struct Tracer {
    tx: Sender<Event>,
    rx: Receiver<()>,
    next_id: usize,
    stack: Vec<usize>,
}

impl Tracer {
    pub fn new(tx: Sender<Event>, rx: Receiver<()>) -> Self {
        Self {
            tx,
            rx,
            next_id: 0,
            stack: Vec::new(),
        }
    }

    pub fn enter(&mut self, name: &str, args: Vec<String>) {
        // Wait for permission to proceed
        if self.rx.recv().is_err() {
            // Channel closed, stop execution (panic or just return)
            // Panicking is fine as it kills the runner thread
            panic!("Tracer disconnected");
        }

        let id = self.next_id;
        self.next_id += 1;
        let parent_id = self.stack.last().copied();

        let event = Event::Enter {
            id,
            name: name.to_string(),
            args,
            parent_id,
        };

        if self.tx.send(event).is_err() {
            panic!("Tracer disconnected");
        }

        self.stack.push(id);
    }

    pub fn exit(&mut self, result: String) {
        // Wait for permission to proceed
        if self.rx.recv().is_err() {
            panic!("Tracer disconnected");
        }

        let id = self.stack.pop().expect("Stack underflow");
        let event = Event::Exit { id, result };

        if self.tx.send(event).is_err() {
            panic!("Tracer disconnected");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;
    use std::thread;

    #[test]
    fn test_tracer_flow() {
        let (tx_event, rx_event) = channel();
        let (tx_token, rx_token) = channel();

        let mut tracer = Tracer::new(tx_event, rx_token);

        thread::spawn(move || {
            tracer.enter("test", vec![]);
            tracer.exit("done".to_string());
        });

        // Send token for enter
        tx_token.send(()).unwrap();
        match rx_event.recv().unwrap() {
            Event::Enter { id, name, .. } => {
                assert_eq!(id, 0);
                assert_eq!(name, "test");
            }
            _ => panic!("Expected Enter"),
        }

        // Send token for exit
        tx_token.send(()).unwrap();
        match rx_event.recv().unwrap() {
            Event::Exit { id, result } => {
                assert_eq!(id, 0);
                assert_eq!(result, "done");
            }
            _ => panic!("Expected Exit"),
        }
    }
}
