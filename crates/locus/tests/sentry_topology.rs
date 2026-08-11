use locus::Topology;

#[test]
fn test_topology_sphere_projective_examples() {
    let topo_sphere = Topology::Sphere;
    assert_eq!(topo_sphere.normalize(-1, 2, 10, 10), Some((0, 7)));

    let topo_proj = Topology::Projective;
    assert_eq!(topo_proj.normalize(2, 10, 10, 10), Some((7, 0)));
}
