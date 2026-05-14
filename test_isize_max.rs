fn main() {
    println!("{}", (isize::MAX as usize) / 32);
    println!("{}", usize::MAX);
    println!("{}", std::mem::size_of::<[f32; 3]>());
}
