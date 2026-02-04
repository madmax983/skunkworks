use crate::simulation::Particle;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Position {
    Long,
    Short,
}

#[derive(Debug)]
pub struct Trader {
    pub x: f64,
    pub y: f64,
    pub balance: f64,
    pub position: Position,
}

impl Trader {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            balance: 10000.0,
            position: Position::Long,
        }
    }

    pub fn toggle_position(&mut self) {
        self.position = match self.position {
            Position::Long => Position::Short,
            Position::Short => Position::Long,
        };
    }

    /// Process collision with a particle.
    /// Returns true if the particle was consumed.
    /// Modifies trader position (y) and balance.
    pub fn interact(&mut self, particle: Particle) -> bool {
        match (self.position, particle) {
            (Position::Long, Particle::Bid) => {
                // Profit + Up
                self.balance += 100.0;
                self.y -= 1.0;
                true
            }
            (Position::Long, Particle::Ask) => {
                // Loss + Down
                self.balance -= 100.0;
                self.y += 1.0;
                true
            }
            (Position::Short, Particle::Bid) => {
                // Loss + Up
                self.balance -= 100.0;
                self.y -= 1.0;
                true
            }
            (Position::Short, Particle::Ask) => {
                // Profit + Down
                self.balance += 100.0;
                self.y += 1.0;
                true
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_long_interaction() {
        let mut trader = Trader::new(10.0, 10.0);
        // Long + Bid = Profit + Up (y decreases)
        trader.interact(Particle::Bid);
        assert_eq!(trader.balance, 10100.0);
        assert_eq!(trader.y, 9.0);

        // Long + Ask = Loss + Down (y increases)
        trader.interact(Particle::Ask);
        assert_eq!(trader.balance, 10000.0);
        assert_eq!(trader.y, 10.0);
    }

    #[test]
    fn test_short_interaction() {
        let mut trader = Trader::new(10.0, 10.0);
        trader.toggle_position(); // Switch to Short

        // Short + Bid = Loss + Up (y decreases)
        trader.interact(Particle::Bid);
        assert_eq!(trader.balance, 9900.0);
        assert_eq!(trader.y, 9.0);

        // Short + Ask = Profit + Down (y increases)
        trader.interact(Particle::Ask);
        assert_eq!(trader.balance, 10000.0);
        assert_eq!(trader.y, 10.0);
    }
}
