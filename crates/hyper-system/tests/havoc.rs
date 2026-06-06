use hyper_system::PbdSystem4D;
use locus::Vec4;
use proptest::prelude::*;

proptest! {
    #[test]
    fn havoc_hyper_system(
        p1_x in any::<f32>(), p1_y in any::<f32>(), p1_z in any::<f32>(), p1_w in any::<f32>(), p1_m in any::<f32>(),
        p2_x in any::<f32>(), p2_y in any::<f32>(), p2_z in any::<f32>(), p2_w in any::<f32>(), p2_m in any::<f32>(),
        dist in any::<f32>(),
        dt in any::<f32>(), iters in any::<usize>(), friction in any::<f32>()
    ) {
        let mut sys = PbdSystem4D::new();
        if p1_x.is_finite() && p1_y.is_finite() && p1_z.is_finite() && p1_w.is_finite() && p1_m.is_finite() && p1_m >= 0.0 &&
           p2_x.is_finite() && p2_y.is_finite() && p2_z.is_finite() && p2_w.is_finite() && p2_m.is_finite() && p2_m >= 0.0 {
            if let Ok(p1) = sys.add_particle(Vec4::new(p1_x, p1_y, p1_z, p1_w), p1_m) {
                if let Ok(p2) = sys.add_particle(Vec4::new(p2_x, p2_y, p2_z, p2_w), p2_m) {
                    if sys.add_distance_constraint(p1, p2, dist.abs()).is_ok() && dt.is_finite() && iters < 100 && friction.is_finite() {
                        sys.step(dt, iters, friction);
                    }
                }
            }
        }
    }
}

#[test]
fn havoc_hyper_system_panic() {
    let mut sys = PbdSystem4D::new();
    let p1 = sys.add_particle(Vec4::new(0.0, 0.0, 0.0, 0.0), 1.0).unwrap();
    let p2 = sys.add_particle(Vec4::new(1.0, 0.0, 0.0, 0.0), 1.0).unwrap();

    // Messing with invalid index
    sys.add_distance_constraint(p1, 9999, 1.0).unwrap_err();
    sys.add_distance_constraint(9999, p2, 1.0).unwrap_err();
}
