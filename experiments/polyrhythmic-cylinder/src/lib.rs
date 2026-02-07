pub mod audio;
pub mod mechanism;

#[cfg(test)]
mod tests {
    use super::mechanism::*;

    #[test]
    fn test_collision_detection() {
        let mut cylinder = Cylinder::new(10.0);
        let mut comb = Comb::new(1);

        // Add a pin at angle 0.1
        cylinder.add_pin(0.1, 0);

        // Set rotation speed to reach the pin in 1 second
        cylinder.angular_velocity = 0.1;

        // Pin is at 0.1 relative.
        // We want world angle (angle + pin) to cross 0 (or 2PI).
        // Start: angle = -0.15. World = -0.05. Floor(-0.05/2PI) = -1.
        // End: angle increases. World becomes > 0. Floor(>0/2PI) = 0.
        cylinder.angle = -0.15;
        let events = cylinder.tick(1.0, &mut comb);

        assert!(!events.is_empty(), "Should have triggered a pluck event");
        assert_eq!(events[0].track_index, 0);
    }
}
