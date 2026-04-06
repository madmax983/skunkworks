use origami::{generate_miura_mesh, MiuraParams, Orientation};

#[test]
#[should_panic]
fn test_havoc_miura_mesh_capacity_panic() {
    let params = MiuraParams {
        a: 1.0,
        b: 1.0,
        gamma: 1.4,
        orientation: Orientation::Horizontal,
    };

    // 👺 Havoc: Pass usize::MAX to trigger integer overflow on capacity calculation
    // expecting "Integer overflow during grid capacity calculation"
    // Sentry will need to fix this bounds issue.
    let _mesh = generate_miura_mesh(params, (usize::MAX, usize::MAX), 0.5);
}
