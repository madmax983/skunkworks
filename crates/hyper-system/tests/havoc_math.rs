use hyper_system::math::Vec4;
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_normalize_havoc(
        x in prop::num::f32::ANY,
        y in prop::num::f32::ANY,
        z in prop::num::f32::ANY,
        w in prop::num::f32::ANY
    ) {
        let v = Vec4::new(x, y, z, w);
        let normalized = v.normalize();

        // Havoc Assert: Result should not contain NaNs unless input contained NaNs.
        // Actually, even if input has NaNs, a robust system might handle it (e.g. return Zero).
        // But definitely, if input is finite (even large), output should not be NaN.

        let input_finite = x.is_finite() && y.is_finite() && z.is_finite() && w.is_finite();

        if input_finite {
            // If input is finite, output MUST be finite (no NaNs, no Infs).
            // This catches the case where len_sq overflows to Inf, then Inf * 0.0 = NaN.
            prop_assert!(normalized.x.is_finite(), "x is not finite: {:?} for input {:?}", normalized.x, v);
            prop_assert!(normalized.y.is_finite(), "y is not finite: {:?} for input {:?}", normalized.y, v);
            prop_assert!(normalized.z.is_finite(), "z is not finite: {:?} for input {:?}", normalized.z, v);
            prop_assert!(normalized.w.is_finite(), "w is not finite: {:?} for input {:?}", normalized.w, v);
        }
    }

    #[test]
    fn test_project_to_3d_havoc(
        x in prop::num::f32::ANY,
        y in prop::num::f32::ANY,
        z in prop::num::f32::ANY,
        w in prop::num::f32::ANY,
        camera_w in prop::num::f32::ANY
    ) {
        let v = Vec4::new(x, y, z, w);
        let proj = v.project_to_3d(camera_w);

        // Havoc Assert: Result should handle NaN inputs gracefully?
        // Or at least, if inputs are finite, output should be finite.

        let input_finite = x.is_finite() && y.is_finite() && z.is_finite() && w.is_finite() && camera_w.is_finite();

        if input_finite {
             prop_assert!(proj.x.is_finite(), "proj.x is not finite: {:?} for input {:?}, cam {}", proj.x, v, camera_w);
             prop_assert!(proj.y.is_finite(), "proj.y is not finite: {:?} for input {:?}, cam {}", proj.y, v, camera_w);
             prop_assert!(proj.z.is_finite(), "proj.z is not finite: {:?} for input {:?}, cam {}", proj.z, v, camera_w);
        }
    }
}
