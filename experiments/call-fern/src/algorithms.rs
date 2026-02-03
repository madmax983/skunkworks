use crate::model::Event;
use std::sync::mpsc::Sender;
use std::time::Duration;

pub fn fibonacci(n: u64, tx: &Sender<Event>) -> u64 {
    // Send Call event
    let _ = tx.send(Event::Call {
        name: "fib".to_string(),
        args: format!("{}", n),
    });

    // Sleep for visualization
    let _ = tx.send(Event::Sleep(Duration::from_millis(100)));
    std::thread::sleep(Duration::from_millis(10)); // Actual thread sleep to not flood queue instantly

    let result = if n <= 1 {
        n
    } else {
        fibonacci(n - 1, tx) + fibonacci(n - 2, tx)
    };

    // Send Return event
    let _ = tx.send(Event::Return {
        value: format!("{}", result),
    });

    let _ = tx.send(Event::Sleep(Duration::from_millis(100)));
    std::thread::sleep(Duration::from_millis(10));

    result
}

pub fn run_fib(n: u64, tx: Sender<Event>) {
    std::thread::spawn(move || {
        fibonacci(n, &tx);
    });
}
