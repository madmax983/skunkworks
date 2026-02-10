use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Meaning {
    Greetings,
    Ack,
    Trade,
    Threat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Symbol(pub u8);

#[derive(Debug, Clone)]
pub struct Protocol {
    pub vocabulary: HashMap<Meaning, Symbol>,
}

impl Protocol {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            vocabulary: HashMap::new(),
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let mut vocabulary = HashMap::new();
        // Assign random symbols to meanings
        vocabulary.insert(Meaning::Greetings, Symbol(rng.gen()));
        vocabulary.insert(Meaning::Ack, Symbol(rng.gen()));
        vocabulary.insert(Meaning::Trade, Symbol(rng.gen()));
        vocabulary.insert(Meaning::Threat, Symbol(rng.gen()));
        Self { vocabulary }
    }

    pub fn get_symbol(&self, meaning: Meaning) -> Option<Symbol> {
        self.vocabulary.get(&meaning).cloned()
    }

    pub fn interpret(&self, symbol: Symbol) -> Option<Meaning> {
        // Reverse lookup
        for (m, s) in &self.vocabulary {
            if *s == symbol {
                return Some(*m);
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct Agent {
    #[allow(dead_code)]
    pub id: usize,
    pub protocol: Protocol,
    #[allow(dead_code)]
    pub energy: f32,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub cooldown: f32,
    pub last_interaction_result: Option<bool>, // true=success, false=fail
}

impl Agent {
    pub fn new(id: usize, x: f32, y: f32) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            id,
            protocol: Protocol::random(),
            energy: 100.0,
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
        let greeting_symbol = match self.protocol.get_symbol(Meaning::Greetings) {
            Some(s) => s,
            None => return false, // Cannot greet
        };

        // Step 2: Receiver (other) interprets
        let interpreted_greeting = other.protocol.interpret(greeting_symbol);

        if interpreted_greeting != Some(Meaning::Greetings) {
            // Receiver failed to understand greeting.
            // Context: Initiator approached receiver, so it was likely a greeting.
            // Receiver learns: greeting_symbol -> Meaning::Greetings
            other.learn(greeting_symbol, Meaning::Greetings);
            return false;
        }

        // Step 3: Receiver sends Ack
        let ack_symbol = match other.protocol.get_symbol(Meaning::Ack) {
            Some(s) => s,
            None => return false, // Cannot ack
        };

        // Step 4: Initiator interprets Ack
        let interpreted_ack = self.protocol.interpret(ack_symbol);

        if interpreted_ack != Some(Meaning::Ack) {
            // Initiator failed to understand ack.
            // Context: Initiator just greeted, so expected an Ack.
            // Initiator learns: ack_symbol -> Meaning::Ack
            self.learn(ack_symbol, Meaning::Ack);
            return false;
        }

        // Success!
        true
    }

    pub fn learn(&mut self, symbol: Symbol, meaning: Meaning) {
        // Remove any existing mapping for this symbol to avoid ambiguity
        let keys_to_remove: Vec<Meaning> = self
            .protocol
            .vocabulary
            .iter()
            .filter(|(_, s)| **s == symbol)
            .map(|(m, _)| *m)
            .collect();

        for k in keys_to_remove {
            self.protocol.vocabulary.remove(&k);
        }

        self.protocol.vocabulary.insert(meaning, symbol);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handshake_mismatch() {
        let mut a = Agent::new(1, 0.0, 0.0);
        let mut b = Agent::new(2, 0.0, 0.0);

        // Force protocols to be different
        a.protocol.vocabulary.insert(Meaning::Greetings, Symbol(1));
        b.protocol.vocabulary.insert(Meaning::Greetings, Symbol(2));

        // B interprets Symbol(1) (A's greeting) -> ?? (Likely None or Wrong)
        // So interaction should fail
        assert_eq!(a.interact(&mut b), false);
    }

    #[test]
    fn test_learning() {
        let mut a = Agent::new(1, 0.0, 0.0);
        let mut b = Agent::new(2, 0.0, 0.0);

        // A uses Symbol(1) for Greetings
        a.protocol.vocabulary.insert(Meaning::Greetings, Symbol(1));

        // B uses Symbol(2) for Greetings (doesn't know Symbol(1))
        b.protocol.vocabulary.insert(Meaning::Greetings, Symbol(2));

        // A also uses Symbol(3) for Ack, B uses Symbol(4) for Ack
        a.protocol.vocabulary.insert(Meaning::Ack, Symbol(3));
        b.protocol.vocabulary.insert(Meaning::Ack, Symbol(4));

        // 1. First interaction: A greets with Symbol(1). B fails to understand. B learns Symbol(1) -> Greetings.
        assert_eq!(a.interact(&mut b), false);

        // Check B learned Symbol(1) -> Greetings
        assert_eq!(b.protocol.get_symbol(Meaning::Greetings), Some(Symbol(1)));
        // Note: `interpret` does reverse lookup.
        assert_eq!(b.protocol.interpret(Symbol(1)), Some(Meaning::Greetings));

        // 2. Second interaction: A greets with Symbol(1). B understands (Greetings).
        //    B replies with Symbol(4) (Ack). A fails to understand. A learns Symbol(4) -> Ack.
        assert_eq!(a.interact(&mut b), false);

        // Check A learned Symbol(4) -> Ack
        assert_eq!(a.protocol.get_symbol(Meaning::Ack), Some(Symbol(4)));

        // 3. Third interaction: A greets (Symbol 1). B understands.
        //    B replies (Symbol 4). A understands.
        //    Success!
        assert_eq!(a.interact(&mut b), true);
    }
}
