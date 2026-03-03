use std::sync::Arc;

fn main() {
    let mut arc_vec = Arc::new(vec![0; 100]);
    Arc::make_mut(&mut arc_vec)[0] = 1;
    let b = arc_vec.clone();
    Arc::make_mut(&mut arc_vec)[1] = 2; // this will clone the vec
    println!("b: {:?}", b[0]);
}
