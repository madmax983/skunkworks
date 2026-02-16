use nile_scheduler::Scheduler;
use num_rational::Ratio;

#[test]
fn test_scheduler_allocation() {
    let mut scheduler = Scheduler::new();

    // Allocate 1/2
    let alloc1 = scheduler.allocate(Ratio::new(1, 2)).unwrap();
    assert_eq!(alloc1.parts, vec![2]);
    assert_eq!(scheduler.free_space.to_ratio(), Ratio::new(1, 2));

    // Allocate 1/4
    let alloc2 = scheduler.allocate(Ratio::new(1, 4)).unwrap();
    assert_eq!(alloc2.parts, vec![4]);
    assert_eq!(scheduler.free_space.to_ratio(), Ratio::new(1, 4));

    // Allocate 1/5 (from remaining 1/4)
    // 1/4 - 1/5 = 1/20
    let alloc3 = scheduler.allocate(Ratio::new(1, 5)).unwrap();
    assert_eq!(alloc3.parts, vec![5]);
    assert_eq!(scheduler.free_space.to_ratio(), Ratio::new(1, 20));
}

#[test]
fn test_scheduler_oversubscribe() {
    let mut scheduler = Scheduler::new();
    let res = scheduler.allocate(Ratio::new(2, 1)); // 2.0 > 1.0
    assert!(res.is_err());
}
