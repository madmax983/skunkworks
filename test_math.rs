fn main() {
    for inner_y in 0u16..100u16 {
        for inner_h in 1u16..100u16 {
            let full_blocks = 200u16;
            for y in 0..full_blocks {
                let inner_bottom = inner_y.wrapping_add(inner_h);
                let draw_y = inner_bottom.saturating_sub(1).saturating_sub(y);
                if draw_y >= inner_bottom {
                    println!("Found! inner_y={} inner_h={} y={} draw_y={} bottom={}", inner_y, inner_h, y, draw_y, inner_bottom);
                    return;
                }
            }
        }
    }

    // What if inner_y + inner_h wraps to 0?
    let inner_y = u16::MAX;
    let inner_h = 1u16;
    let inner_bottom = inner_y.wrapping_add(inner_h); // 0
    let y = 0;
    let draw_y = inner_bottom.saturating_sub(1).saturating_sub(y); // 0
    println!("wrapped to 0 -> cond: {}", draw_y >= inner_bottom); // 0 >= 0 is true!
}
