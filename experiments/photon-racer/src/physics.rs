use locus::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MirrorType {
    Slash,     // '/'
    Backslash, // '\'
}

/// Reflects a velocity vector off a mirror.
///
/// Assumptions:
/// - Velocity is axis-aligned (1,0), (-1,0), (0,1), (0,-1).
/// - Mirrors are 45 degrees.
pub fn reflect(velocity: Vec2, mirror: MirrorType) -> Vec2 {
    match mirror {
        MirrorType::Slash => {
            // '/' reflects (x, y) -> (-y, -x)
            Vec2::new(-velocity.y, -velocity.x)
        }
        MirrorType::Backslash => {
            // '\' reflects (x, y) -> (y, x)
            Vec2::new(velocity.y, velocity.x)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reflect_slash() {
        let right = Vec2::new(1.0, 0.0);
        let up = Vec2::new(0.0, -1.0);
        let left = Vec2::new(-1.0, 0.0);
        let down = Vec2::new(0.0, 1.0);

        // Right -> Up
        assert_eq!(reflect(right, MirrorType::Slash), up);
        // Up -> Right
        assert_eq!(reflect(up, MirrorType::Slash), right);
        // Left -> Down
        assert_eq!(reflect(left, MirrorType::Slash), down);
        // Down -> Left
        assert_eq!(reflect(down, MirrorType::Slash), left);
    }

    #[test]
    fn test_reflect_backslash() {
        let right = Vec2::new(1.0, 0.0);
        let up = Vec2::new(0.0, -1.0);
        let left = Vec2::new(-1.0, 0.0);
        let down = Vec2::new(0.0, 1.0);

        // Right -> Down
        assert_eq!(reflect(right, MirrorType::Backslash), down);
        // Down -> Right
        assert_eq!(reflect(down, MirrorType::Backslash), right);
        // Left -> Up
        assert_eq!(reflect(left, MirrorType::Backslash), up);
        // Up -> Left
        assert_eq!(reflect(up, MirrorType::Backslash), left);
    }
}
