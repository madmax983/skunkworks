use macroquad::prelude::*;

pub struct TargetVisual {
    pub id: usize,
    pub name: String,
    pub angle: f32,
    pub radius: f32,
    pub last_rtt: Option<u128>,
    pub flash: f32, // 0.0 to 1.0
}

pub struct PacketVisual {
    pub target_id: usize,
    pub progress: f32, // 0.0 to 1.0 (0=Home, 1=Target)
    pub return_trip: bool,
}

pub fn draw_system(center: Vec2, targets: &[TargetVisual], packets: &[PacketVisual]) {
    // Draw Home
    draw_circle(center.x, center.y, 20.0, BLUE);

    // Draw Targets
    for target in targets {
        let pos = center + Vec2::new(target.angle.cos(), target.angle.sin()) * target.radius;

        // Flash logic
        let mut radius_mod = 0.0;
        let color = if target.flash > 0.0 {
            radius_mod = target.flash * 10.0;
            Color::new(1.0, 1.0 - target.flash, 1.0 - target.flash, 1.0)
        } else if target.flash < 0.0 {
            // Red flash for error
            let f = -target.flash;
            radius_mod = f * 5.0;
            Color::new(1.0, 0.0, 0.0, 1.0)
        } else {
            if target.last_rtt.is_some() { GREEN } else { GRAY }
        };

        // Draw Connection Line
        draw_line(center.x, center.y, pos.x, pos.y, 1.0, Color::new(0.5, 0.5, 0.5, 0.3));

        // Draw Node
        draw_circle(pos.x, pos.y, 15.0 + radius_mod, color);

        // Draw Label
        draw_text(&target.name, pos.x - 20.0, pos.y - 25.0, 20.0, WHITE);

        // Draw RTT
        if let Some(rtt) = target.last_rtt {
            draw_text(&format!("{}ms", rtt), pos.x - 20.0, pos.y + 35.0, 16.0, LIGHTGRAY);
        }
    }

    // Draw Packets
    for packet in packets {
        if let Some(target) = targets.iter().find(|t| t.id == packet.target_id) {
            let target_pos = center + Vec2::new(target.angle.cos(), target.angle.sin()) * target.radius;

            // Linear interpolation
            let p = if packet.return_trip {
                1.0 - packet.progress
            } else {
                packet.progress
            };

            let pos = center.lerp(target_pos, p);
            draw_circle(pos.x, pos.y, 5.0, YELLOW);
        }
    }
}
