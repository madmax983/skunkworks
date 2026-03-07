fn main() {
    let mut q = quipu::Quipu::new();
    let mut parent_cord = quipu::Cord::from(100);
    let mut child_cord = quipu::Cord::from(10);
    let grandchild_cord = quipu::Cord::from(1);
    child_cord.subsidiaries.push(grandchild_cord);
    parent_cord.subsidiaries.push(child_cord);
    q.add_cord(parent_cord);
    println!("{}", q);
}
