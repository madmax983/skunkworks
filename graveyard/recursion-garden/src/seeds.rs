use crate::tracer::Tracer;

pub fn fibonacci(n: u32, tracer: &mut Tracer) -> u32 {
    tracer.enter("fib", vec![n.to_string()]);

    let res = if n <= 1 {
        1
    } else {
        fibonacci(n - 1, tracer) + fibonacci(n - 2, tracer)
    };

    tracer.exit(res.to_string());
    res
}

pub fn merge_sort(mut arr: Vec<i32>, tracer: &mut Tracer) -> Vec<i32> {
    tracer.enter("merge_sort", vec![format!("{:?}", arr)]);

    let res = if arr.len() <= 1 {
        arr
    } else {
        let mid = arr.len() / 2;
        let left = arr.drain(0..mid).collect();
        let right = arr; // remaining

        let sorted_left = merge_sort(left, tracer);
        let sorted_right = merge_sort(right, tracer);

        let mut merged = Vec::with_capacity(sorted_left.len() + sorted_right.len());
        let (mut i, mut j) = (0, 0);

        while i < sorted_left.len() && j < sorted_right.len() {
            if sorted_left[i] <= sorted_right[j] {
                merged.push(sorted_left[i]);
                i += 1;
            } else {
                merged.push(sorted_right[j]);
                j += 1;
            }
        }
        merged.extend_from_slice(&sorted_left[i..]);
        merged.extend_from_slice(&sorted_right[j..]);
        merged
    };

    tracer.exit(format!("{:?}", res));
    res
}

pub fn collatz(n: u64, tracer: &mut Tracer) -> u64 {
    tracer.enter("collatz", vec![n.to_string()]);

    let res = if n == 1 {
        1
    } else if n.is_multiple_of(2) {
        collatz(n / 2, tracer)
    } else {
        collatz(3 * n + 1, tracer)
    };

    tracer.exit(res.to_string());
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;
    use std::thread;

    #[test]
    fn test_fibonacci() {
        // This test would deadlock if run on main thread without a separate pump.
        // So we spawn a thread for the runner.

        let (tx_token, rx_token) = channel();
        let (tx_event, rx_event) = channel();

        thread::spawn(move || {
            let mut tracer = Tracer::new(tx_event, rx_token);
            fibonacci(2, &mut tracer);
        });

        // fib(2) calls:
        // enter(2)
        //   enter(1) -> exit(1)
        //   enter(0) -> exit(1)
        // exit(2)

        // We need to send tokens for each enter/exit.
        // Total enters: 3 (fib(2), fib(1), fib(0))
        // Total exits: 3

        for _ in 0..6 {
            tx_token.send(()).unwrap();
            rx_event.recv().unwrap(); // consume event
        }
    }
}
