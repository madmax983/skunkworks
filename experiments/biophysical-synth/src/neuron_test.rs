
use super::HodgkinHuxley;

#[test]
fn test_neuron_initialization() {
    let n = HodgkinHuxley::new();
    assert_eq!(n.v, -65.0);
}

#[test]
fn test_neuron_step() {
    let mut n = HodgkinHuxley::new();
    // Inject current to cause depolarization
    n.i_inj = 20.0;
    let v_start = n.v;

    // Step for 1ms
    for _ in 0..100 {
        n.step(0.01);
    }

    // Voltage should rise
    assert!(n.v > v_start);
}
