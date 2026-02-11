use locus::Topology;

#[test]
fn test_topology_normalize_zero_width() {
    let t = Topology::Torus;
    assert_eq!(t.normalize(0, 0, 0, 10), None);
}

#[test]
fn test_topology_normalize_zero_height() {
    let t = Topology::Torus;
    assert_eq!(t.normalize(0, 0, 10, 0), None);
}
