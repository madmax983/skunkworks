use std::sync::{Arc, Barrier, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use crossbeam::channel::bounded;
use crossbeam::queue::SegQueue;
use crate::audio::Note;

pub struct Groove {
    #[allow(dead_code)]
    handles: Vec<JoinHandle<()>>,
    pub active_flags: Arc<ActiveFlags>,
    stop_signal: Arc<AtomicBool>,
}

pub struct ActiveFlags {
    pub kick: AtomicBool,
    pub snare: AtomicBool,
    pub hat: AtomicBool,
    pub clap: AtomicBool,
}

impl Groove {
    pub fn start(event_queue: Arc<SegQueue<Note>>) -> Self {
        let stop_signal = Arc::new(AtomicBool::new(false));
        let active_flags = Arc::new(ActiveFlags {
            kick: AtomicBool::new(false),
            snare: AtomicBool::new(false),
            hat: AtomicBool::new(false),
            clap: AtomicBool::new(false),
        });

        let barrier = Arc::new(Barrier::new(2)); // Kick + Snare sync point
        let rw_lock = Arc::new(RwLock::new(())); // Hat contends with Kick/Snare
        let (tx, rx) = bounded(4); // Kick/Snare send tokens to Clap

        let mut handles = Vec::new();

        // Kick Thread (500ms base)
        {
            let stop = stop_signal.clone();
            let queue = event_queue.clone();
            let flags = active_flags.clone();
            let barrier = barrier.clone();
            let lock = rw_lock.clone();
            let tx = tx.clone();

            handles.push(thread::spawn(move || {
                let mut count = 0;
                while !stop.load(Ordering::Relaxed) {
                    flags.kick.store(true, Ordering::Relaxed);

                    // Hold lock briefly to contend with Hat
                    {
                        let _guard = lock.write().unwrap();
                        queue.push(Note::Kick);
                        thread::sleep(Duration::from_millis(50));
                    }

                    if count % 2 == 0 {
                        let _ = tx.try_send(()); // Trigger clap occasionally
                    }

                    flags.kick.store(false, Ordering::Relaxed);

                    thread::sleep(Duration::from_millis(450));

                    count += 1;
                    if count % 4 == 0 {
                        // Polymetric Sync Point
                        barrier.wait();
                    }
                }
            }));
        }

        // Snare Thread (700ms base - 7/5 polyrhythm)
        {
            let stop = stop_signal.clone();
            let queue = event_queue.clone();
            let flags = active_flags.clone();
            let barrier = barrier.clone();
            let tx = tx.clone();

            handles.push(thread::spawn(move || {
                let mut count = 0;
                while !stop.load(Ordering::Relaxed) {
                    flags.snare.store(true, Ordering::Relaxed);

                    queue.push(Note::Snare);
                    if count % 3 == 0 {
                         let _ = tx.try_send(());
                    }

                    thread::sleep(Duration::from_millis(50)); // Flash visual
                    flags.snare.store(false, Ordering::Relaxed);

                    thread::sleep(Duration::from_millis(650));

                    count += 1;
                    if count % 3 == 0 {
                        barrier.wait(); // Sync with Kick every 3 snares vs 4 kicks
                    }
                }
            }));
        }

        // Hat Thread (Fast 125ms - 1/4 note if 120bpm is 500ms)
        {
            let stop = stop_signal.clone();
            let queue = event_queue.clone();
            let flags = active_flags.clone();
            let lock = rw_lock.clone();

            handles.push(thread::spawn(move || {
                while !stop.load(Ordering::Relaxed) {
                    flags.hat.store(true, Ordering::Relaxed);

                    // Try to play clean hat
                    if let Ok(_guard) = lock.try_read() {
                        queue.push(Note::HatClosed);
                    } else {
                        // Contention! Splashy open hat
                        queue.push(Note::HatOpen);
                    }

                    thread::sleep(Duration::from_millis(20));
                    flags.hat.store(false, Ordering::Relaxed);

                    thread::sleep(Duration::from_millis(105));
                }
            }));
        }

        // Clap Thread (Consumer)
        {
            let stop = stop_signal.clone();
            let queue = event_queue.clone();
            let flags = active_flags.clone();

            handles.push(thread::spawn(move || {
                while !stop.load(Ordering::Relaxed) {
                    if let Ok(_) = rx.recv_timeout(Duration::from_millis(100)) {
                        flags.clap.store(true, Ordering::Relaxed);
                        queue.push(Note::Clap);
                        thread::sleep(Duration::from_millis(50));
                        flags.clap.store(false, Ordering::Relaxed);
                    }
                }
            }));
        }

        Self {
            handles,
            active_flags,
            stop_signal,
        }
    }
}

impl Drop for Groove {
    fn drop(&mut self) {
        self.stop_signal.store(true, Ordering::Relaxed);
    }
}
