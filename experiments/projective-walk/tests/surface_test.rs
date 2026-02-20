use projective_walk::surface::generate_mesh;

#[test]
fn test_mesh_generation() {
    let res = 10;
    let (vertices, indices) = generate_mesh(res);

    // Grid is (res+1) * (res+1)
    let expected_verts = (res + 1) * (res + 1);
    assert_eq!(vertices.len(), expected_verts);

    // Indices: res * res * 2 triangles * 3 vertices
    let expected_indices = res * res * 6;
    assert_eq!(indices.len(), expected_indices);
}
