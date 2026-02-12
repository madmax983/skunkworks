use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Meaning {
    Greetings,
    Ack,
    Trade,
    Threat,
}

#[derive(Debug, Clone)]
pub struct Agent {
    pub vocabulary: HashMap<Meaning, u8>,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub cooldown: f32,
    pub last_interaction_result: Option<bool>, // true=success, false=fail
}

impl Agent {
    pub fn new(x: f32, y: f32) -> Self {
        let mut rng = rand::thread_rng();
        let mut vocabulary = HashMap::new();
        // Assign random symbols to meanings
        vocabulary.insert(Meaning::Greetings, rng.gen());
        vocabulary.insert(Meaning::Ack, rng.gen());
        vocabulary.insert(Meaning::Trade, rng.gen());
        vocabulary.insert(Meaning::Threat, rng.gen());

        Self {
            vocabulary,
            x,
            y,
            vx: rng.gen_range(-1.0..1.0),
            vy: rng.gen_range(-1.0..1.0),
            cooldown: 0.0,
            last_interaction_result: None,
        }
    }

    pub fn update_pos(&mut self, dt: f32, width: f32, height: f32) {
        self.x += self.vx * 60.0 * dt; // Speed scaling
        self.y += self.vy * 60.0 * dt;

        // Bounce off walls
        if self.x < 0.0 || self.x > width {
            self.vx *= -1.0;
            self.x = self.x.clamp(0.0, width);
        }
        if self.y < 0.0 || self.y > height {
            self.vy *= -1.0;
            self.y = self.y.clamp(0.0, height);
        }

        if self.cooldown > 0.0 {
            self.cooldown -= dt;
        }
    }

    // Returns true if handshake is successful
    pub fn interact(&mut self, other: &mut Agent) -> bool {
        // Step 1: Initiator (self) sends Greetings
        let greeting = match self.vocabulary.get(&Meaning::Greetings) {
            Some(s) => *s,
            None => return false, // Cannot greet
        };

        // Step 2: Receiver (other) interprets
        let interpreted_greeting = other.interpret(greeting);

        if interpreted_greeting != Some(Meaning::Greetings) {
            // Receiver failed to understand greeting.
            // Context: Initiator approached receiver, so it was likely a greeting.
            // Receiver learns: greeting_symbol -> Meaning::Greetings
            other.learn(greeting, Meaning::Greetings);
            return false;
        }

        // Step 3: Receiver sends Ack
        let ack = match other.vocabulary.get(&Meaning::Ack) {
            Some(s) => *s,
            None => return false, // Cannot ack
        };

        // Step 4: Initiator interprets Ack
        let interpreted_ack = self.interpret(ack);

        if interpreted_ack != Some(Meaning::Ack) {
            // Initiator failed to understand ack.
            // Context: Initiator just greeted, so expected an Ack.
            // Initiator learns: ack_symbol -> Meaning::Ack
            self.learn(ack, Meaning::Ack);
            return false;
        }

        // Success!
        true
    }

    fn interpret(&self, symbol: u8) -> Option<Meaning> {
        for (m, s) in &self.vocabulary {
            if *s == symbol {
                return Some(*m);
            }
        }
        None
    }

    pub fn learn(&mut self, symbol: u8, meaning: Meaning) {
        // Remove any existing mapping for this symbol to avoid ambiguity
        // We need to collect keys first to avoid borrowing issues
        let keys_to_remove: Vec<Meaning> = self
            .vocabulary
            .iter()
            .filter(|(_, s)| **s == symbol)
            .map(|(m, _)| *m)
            .collect();

        for k in keys_to_remove {
            self.vocabulary.remove(&k);
        }

        self.vocabulary.insert(meaning, symbol);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handshake_mismatch() {
        let mut a = Agent::new(0.0, 0.0);
        let mut b = Agent::new(0.0, 0.0);

        // Force protocols to be different
        a.vocabulary.insert(Meaning::Greetings, 1);
        b.vocabulary.insert(Meaning::Greetings, 2);

        // B interprets 1 (A's greeting) -> ?? (Likely None or Wrong)
        // So interaction should fail
        assert_eq!(a.interact(&mut b), false);
    }

    #[test]
    fn test_learning() {
        let mut a = Agent::new(0.0, 0.0);
        let mut b = Agent::new(0.0, 0.0);

        // A uses 1 for Greetings
        a.vocabulary.insert(Meaning::Greetings, 1);

        // B uses 2 for Greetings (doesn't know 1)
        b.vocabulary.insert(Meaning::Greetings, 2);

        // A also uses 3 for Ack, B uses 4 for Ack
        a.vocabulary.insert(Meaning::Ack, 3);
        b.vocabulary.insert(Meaning::Ack, 4);

        // 1. First interaction: A greets with 1. B fails to understand. B learns 1 -> Greetings.
        assert_eq!(a.interact(&mut b), false);

        // Check B learned 1 -> Greetings
        assert_eq!(b.vocabulary.get(&Meaning::Greetings), Some(&1));
        // Note: `interpret` does reverse lookup.
        assert_eq!(b.interpret(1), Some(Meaning::Greetings));

        // 2. Second interaction: A greets with 1. B understands (Greetings).
        //    B replies with 4 (Ack). A fails to understand. A learns 4 -> Ack.
        assert_eq!(a.interact(&mut b), false);

        // Check A learned 4 -> Ack
        assert_eq!(a.vocabulary.get(&Meaning::Ack), Some(&4));

        // 3. Third interaction: A greets (1). B understands.
        //    B replies (4). A understands.
        //    Success!
        assert_eq!(a.interact(&mut b), true);
    }
}
