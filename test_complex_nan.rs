use num_complex::Complex;

fn main() {
    let a = Complex::new(std::f64::NAN, 0.0);
    println!("a.is_nan(): {}", a.is_nan());
}
