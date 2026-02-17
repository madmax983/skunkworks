use crate::grid::HiddenLayer;

pub fn generate_pattern(width: usize, height: usize) -> HiddenLayer {
    let mut layer = HiddenLayer::new(width, height);

    let cx = width as f32 / 2.0;
    let cy = height as f32 / 2.0;
    let radius = (width.min(height) as f32) * 0.35;

    for y in 0..height {
        for x in 0..width {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let dist = (dx*dx + dy*dy).sqrt();

            // Draw a Circle
            if (dist - radius).abs() < 1.5 {
                layer.set(x, y, true);
            }

            // Draw an X
            if (dx.abs() - dy.abs()).abs() < 1.5 && dist < radius {
               layer.set(x, y, true);
            }
        }
    }

    layer
}
