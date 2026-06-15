use num_complex::Complex;
use poincare_disk::Geodesic;
use proptest::prelude::*;

proptest! {
    #[test]
    fn havoc_poincare_geodesic(
        re1 in any::<f64>(),
        im1 in any::<f64>(),
        re2 in any::<f64>(),
        im2 in any::<f64>(),
    ) {
        if re1.is_finite() && im1.is_finite() && re1*re1 + im1*im1 < 1.0 && re2.is_finite() && im2.is_finite() && re2*re2 + im2*im2 < 1.0 {
            let p1 = Complex::new(re1, im1);
            let p2 = Complex::new(re2, im2);
            let geo = Geodesic::new(p1, p2);
            let _ = geo.euclidean_circle();
        }
    }
}
//
