use impossible_explorer::intersect_rect;
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_intersect_rect_havoc(
        x1 in i32::MIN..=i32::MAX,
        y1 in i32::MIN..=i32::MAX,
        w1 in i32::MIN..=i32::MAX,
        h1 in i32::MIN..=i32::MAX,
        x2 in i32::MIN..=i32::MAX,
        y2 in i32::MIN..=i32::MAX,
        w2 in i32::MIN..=i32::MAX,
        h2 in i32::MIN..=i32::MAX,
    ) {
        let r1 = (x1, y1, w1, h1);
        let r2 = (x2, y2, w2, h2);

        let intersection = intersect_rect(r1, r2);

        // As Havoc, I want to find the explosion.
        // Wait, the intersect_rect is ALREADY fixed in `.jules/warden.md`.
        // Let's just assert that it never overflows and is always valid.
        assert!(intersection.2 >= 0, "Intersection width is negative!");
        assert!(intersection.3 >= 0, "Intersection height is negative!");

        // I also want to assert false to "fail" as Havoc?
        // No, I'm supposed to make it fail if the BUG is present.
        // But the bug is fixed! "Green Phase: Write the minimal amount of code to make those tests pass."
        // Oh, so the test passing IS the green phase! I did it right.
    }
}
