use macroquad::prelude::*;

#[derive(Debug)]
pub struct QuadTree {
    pub root: Node,
}

#[derive(Debug)]
pub enum Node {
    Leaf { rect: Rect, color: Color, error: f32 },
    Branch { rect: Rect, children: Box<[Node; 4]>, error: f32 },
}

impl QuadTree {
    pub fn from_image(image: &Image, base_threshold: f32, focus: Option<(Vec2, f32)>) -> Self {
        let rect = Rect::new(0.0, 0.0, image.width as f32, image.height as f32);
        let root = Self::build(image, rect, base_threshold, focus);
        QuadTree { root }
    }

    fn build(image: &Image, rect: Rect, base_threshold: f32, focus: Option<(Vec2, f32)>) -> Node {
        // Determine local threshold based on focus
        let mut local_threshold = base_threshold;

        if let Some((pos, radius)) = focus {
            // Distance from center of rect to focus point
            let center = vec2(rect.x + rect.w * 0.5, rect.y + rect.h * 0.5);
            let dist = center.distance(pos);

            // If inside radius, reduce threshold
            if dist < radius {
                let factor = (dist / radius).max(0.01); // Don't go to exactly 0, keeps *some* compression
                // Use a non-linear falloff for smoother effect
                let factor = factor * factor;
                local_threshold *= factor;
            }
        }

        let (color, error) = calculate_stats(image, rect);

        // Recursion base cases:
        // 1. Error is acceptable
        // 2. Resolution is too small (1x1 pixel)
        if error <= local_threshold || rect.w <= 1.0 || rect.h <= 1.0 {
            return Node::Leaf { rect, color, error };
        }

        let w2 = rect.w / 2.0;
        let h2 = rect.h / 2.0;

        // Top Left
        let c1 = Self::build(image, Rect::new(rect.x, rect.y, w2, h2), base_threshold, focus);
        // Top Right
        let c2 = Self::build(image, Rect::new(rect.x + w2, rect.y, w2, h2), base_threshold, focus);
        // Bottom Left
        let c3 = Self::build(image, Rect::new(rect.x, rect.y + h2, w2, h2), base_threshold, focus);
        // Bottom Right
        let c4 = Self::build(image, Rect::new(rect.x + w2, rect.y + h2, w2, h2), base_threshold, focus);

        Node::Branch {
            rect,
            children: Box::new([c1, c2, c3, c4]),
            error,
        }
    }
}

fn calculate_stats(image: &Image, rect: Rect) -> (Color, f32) {
    let start_x = rect.x as u32;
    let start_y = rect.y as u32;
    let end_x = (rect.x + rect.w) as u32;
    let end_y = (rect.y + rect.h) as u32;

    let mut r_sum = 0.0;
    let mut g_sum = 0.0;
    let mut b_sum = 0.0;
    let mut count = 0.0;

    // Safety clamp
    let max_w = image.width as u32;
    let max_h = image.height as u32;

    for y in start_y..end_y {
        if y >= max_h { break; }
        for x in start_x..end_x {
            if x >= max_w { break; }
            let p = image.get_pixel(x, y);
            r_sum += p.r;
            g_sum += p.g;
            b_sum += p.b;
            count += 1.0;
        }
    }

    if count == 0.0 {
        return (BLACK, 0.0);
    }

    let avg_r = r_sum / count;
    let avg_g = g_sum / count;
    let avg_b = b_sum / count;
    let avg_color = Color::new(avg_r, avg_g, avg_b, 1.0);

    // Calculate Variance (MSE)
    let mut error_sum = 0.0;
    for y in start_y..end_y {
        if y >= max_h { break; }
        for x in start_x..end_x {
            if x >= max_w { break; }
            let p = image.get_pixel(x, y);
            let dr = p.r - avg_r;
            let dg = p.g - avg_g;
            let db = p.b - avg_b;
            error_sum += dr*dr + dg*dg + db*db;
        }
    }

    let mse = error_sum / count;
    (avg_color, mse.sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniform_image() {
        let image = Image::gen_image_color(2, 2, WHITE);
        let tree = QuadTree::from_image(&image, 0.1, None);

        match tree.root {
            Node::Leaf { rect, color, error } => {
                assert_eq!(rect.w, 2.0);
                assert_eq!(rect.h, 2.0);
                assert_eq!(color, WHITE);
                assert!(error < 0.0001);
            },
            _ => panic!("Expected a single leaf node for uniform image"),
        }
    }

    #[test]
    fn test_split_image() {
        let mut image = Image::gen_image_color(2, 2, WHITE);
        image.set_pixel(0, 0, BLACK);

        let tree = QuadTree::from_image(&image, 0.001, None);

        match tree.root {
            Node::Branch { children, .. } => {
                assert_eq!(children.len(), 4);
            },
            _ => panic!("Expected a branch node for non-uniform image"),
        }
    }
}
