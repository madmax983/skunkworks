fn main() {
    let a: (i32, i32, i32, i32) = (2147483600, 0, 100, 100);
    let b: (i32, i32, i32, i32) = (0, 0, 100, 100);

    let x1 = a.0.max(b.0);
    let x2 = (a.0 + a.2).min(b.0 + b.2);

    println!("x1: {}", x1);
    println!("x2: {}", x2);
    println!("width: {}", (x2 - x1).max(0));
}
