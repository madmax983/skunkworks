use sched_market::model::{Scheduler, Thread};

#[test]
fn test_single_thread_success() {
    let mut sched = Scheduler::new();
    // 5 ticks of work, 10 ticks deadline, huge budget
    let t = Thread::new(1, 5, 10, 100.0, 1000.0, 0);
    sched.add_thread(t);

    for _ in 0..10 {
        sched.tick();
    }

    assert_eq!(sched.completed_threads.len(), 1);
    assert_eq!(sched.threads.len(), 0);
    assert_eq!(sched.failed_threads.len(), 0);

    let t = &sched.completed_threads[0];
    assert_eq!(t.id, 1);
    assert_eq!(t.work_needed, 0);
    // Should have paid something
    assert!(t.budget < 100.0);
}

#[test]
fn test_competition() {
    let mut sched = Scheduler::new();

    // Thread 1: Urgent (work=5, deadline=6), Budget=100
    // Thread 2: Relaxed (work=5, deadline=20), Budget=100

    // Actually, let's test High Budget vs Low Budget.
    // Thread A: Budget 1000, Work 5.
    // Thread B: Budget 10, Work 5.
    // Both same urgency.

    let ta = Thread::new(1, 5, 20, 1000.0, 1000.0, 0);
    let tb = Thread::new(2, 5, 20, 10.0, 1000.0, 0);

    sched.add_thread(ta);
    sched.add_thread(tb);

    // Tick 1: Expect A to win because it can bid higher (budget/work is higher).
    sched.tick();

    let history = sched.history.last().unwrap();
    assert_eq!(history.winner_id, Some(1)); // 1 should win
    assert!(history.price > 1.0); // Price should be > 1.0 (tb bids ~2.0, ta bids ~200.0)
}

#[test]
fn test_deadline_failure() {
    let mut sched = Scheduler::new();
    // 5 ticks of work, deadline 3. impossible (can only run at 0, 1, 2).
    let t = Thread::new(1, 5, 3, 100.0, 1000.0, 0);
    sched.add_thread(t);

    for _ in 0..10 {
        sched.tick();
    }

    assert_eq!(sched.failed_threads.len(), 1);
    assert_eq!(sched.completed_threads.len(), 0);
}
