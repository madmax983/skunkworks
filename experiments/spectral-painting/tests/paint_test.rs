use spectral_painting::paint::Canvas;

#[test]
fn test_paint_physics() {
    let width = 10;
    let height = 10;
    let mut canvas = Canvas::new(width, height);

    // 1. Apply brush at y=0 (Top)
    // Intensity 1.0 ensures brush_count > 0
    canvas.apply_brush(0, 1.0, (255, 0, 0));

    // Check that some pixels at y=0 are painted
    let mut found_paint = false;
    for x in 0..width {
        let p = canvas.get(x, 0);
        if p.r > 0 {
            found_paint = true;
            break;
        }
    }
    assert!(found_paint, "Brush should have painted pixels at y=0");

    // 2. Tick physics (Gravity)
    // We tick multiple times to ensure flow
    for _ in 0..5 {
        canvas.tick_physics();
    }

    // Check that paint has moved to y=1 or lower
    let mut found_drip = false;
    for y in 1..height {
        for x in 0..width {
            let p = canvas.get(x, y);
            if p.r > 0 {
                found_drip = true;
                break;
            }
        }
    }
    assert!(found_drip, "Paint should have dripped down to y>0");
}
