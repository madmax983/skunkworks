#[derive(Debug, Clone, Copy)]
pub enum RotorType {
    I,
    II,
    III,
}

#[derive(Debug, Clone)]
pub struct Rotor {
    wiring: Vec<usize>,
    inverse_wiring: Vec<usize>,
    notch: usize,
    pub position: usize,     // 0-25, represents 'A'-'Z' displayed
    pub ring_setting: usize, // 0-25, ring position relative to core
}

impl Rotor {
    pub fn new(rtype: RotorType, position: usize, ring_setting: usize) -> Self {
        let (wiring_str, notch_char) = match rtype {
            // Standard Enigma I Walzen
            RotorType::I => ("EKMFLGDQVZNTOWYHXUSPAIBRCJ", 'Q'),
            RotorType::II => ("AJDKSIRUXBLHWTMCQGZNPYFVOE", 'E'),
            RotorType::III => ("BDFHJLCPRTXVZNYEIWGAKMUSQO", 'V'),
        };

        let wiring: Vec<usize> = wiring_str
            .chars()
            .map(|c| (c as u8 - b'A') as usize)
            .collect();

        let mut inverse_wiring = vec![0; 26];
        for (i, &output) in wiring.iter().enumerate() {
            inverse_wiring[output] = i;
        }

        let notch = (notch_char as u8 - b'A') as usize;

        Self {
            wiring,
            inverse_wiring,
            notch,
            position,
            ring_setting,
        }
    }

    pub fn step(&mut self) {
        self.position = (self.position + 1) % 26;
    }

    pub fn is_at_notch(&self) -> bool {
        self.position == self.notch
    }

    // Signal passes Right to Left
    pub fn forward(&self, input: usize) -> usize {
        let offset = (self.position + 26 - self.ring_setting) % 26;
        let enter_idx = (input + offset) % 26;
        let exit_idx = self.wiring[enter_idx];
        (exit_idx + 26 - offset) % 26
    }

    // Signal passes Left to Right (after reflector)
    pub fn backward(&self, input: usize) -> usize {
        let offset = (self.position + 26 - self.ring_setting) % 26;
        let enter_idx = (input + offset) % 26;
        let exit_idx = self.inverse_wiring[enter_idx];
        (exit_idx + 26 - offset) % 26
    }
}

#[derive(Debug, Clone)]
pub struct Reflector {
    wiring: Vec<usize>,
}

impl Default for Reflector {
    fn default() -> Self {
        Self::new()
    }
}

impl Reflector {
    pub fn new() -> Self {
        // Reflector B (Wide)
        let wiring_str = "YRUHQSLDPXNGOKMIEBFZCWVJAT";
        let wiring: Vec<usize> = wiring_str
            .chars()
            .map(|c| (c as u8 - b'A') as usize)
            .collect();
        Self { wiring }
    }

    pub fn reflect(&self, input: usize) -> usize {
        self.wiring[input]
    }
}

#[derive(Debug, Clone)]
pub struct Plugboard {
    mapping: Vec<usize>,
}

impl Default for Plugboard {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugboard {
    pub fn new() -> Self {
        Self {
            mapping: (0..26).collect(),
        }
    }

    pub fn add_cable(&mut self, a: char, b: char) {
        let idx_a = (a.to_ascii_uppercase() as u8 - b'A') as usize;
        let idx_b = (b.to_ascii_uppercase() as u8 - b'A') as usize;
        self.mapping[idx_a] = idx_b;
        self.mapping[idx_b] = idx_a;
    }

    pub fn process(&self, input: usize) -> usize {
        self.mapping[input]
    }
}

#[derive(Debug, Clone)]
pub struct EnigmaMachine {
    pub left: Rotor,
    pub middle: Rotor,
    pub right: Rotor,
    reflector: Reflector,
    plugboard: Plugboard,
}

impl Default for EnigmaMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl EnigmaMachine {
    pub fn new() -> Self {
        Self {
            left: Rotor::new(RotorType::I, 0, 0),
            middle: Rotor::new(RotorType::II, 0, 0),
            right: Rotor::new(RotorType::III, 0, 0),
            reflector: Reflector::new(),
            plugboard: Plugboard::new(),
        }
    }

    // Standard Wehrmacht Enigma stepping (Double Step)
    fn step_rotors(&mut self) {
        let r_at_notch = self.right.is_at_notch();
        let m_at_notch = self.middle.is_at_notch();

        // Right rotor always steps
        // Middle rotor steps if Right is at notch OR Middle is at notch (double step)
        // Left rotor steps if Middle is at notch

        // We determine stepping flags BEFORE actually moving any rotor
        let step_middle = r_at_notch || m_at_notch;
        let step_left = m_at_notch;

        self.right.step();
        if step_middle {
            self.middle.step();
        }
        if step_left {
            self.left.step();
        }
    }

    pub fn encrypt_char(&mut self, c: char) -> char {
        if !c.is_ascii_alphabetic() {
            return c;
        }

        let input = (c.to_ascii_uppercase() as u8 - b'A') as usize;

        // 1. Step Rotors
        self.step_rotors();

        // 2. Plugboard
        let mut signal = self.plugboard.process(input);

        // 3. Right -> Left
        signal = self.right.forward(signal);
        signal = self.middle.forward(signal);
        signal = self.left.forward(signal);

        // 4. Reflector
        signal = self.reflector.reflect(signal);

        // 5. Left -> Right
        signal = self.left.backward(signal);
        signal = self.middle.backward(signal);
        signal = self.right.backward(signal);

        // 6. Plugboard
        signal = self.plugboard.process(signal);

        (signal as u8 + b'A') as char
    }

    pub fn encrypt_string(&mut self, text: &str) -> String {
        text.chars().map(|c| self.encrypt_char(c)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symmetry() {
        let mut enigma = EnigmaMachine::new();
        let input = "HELLOWORLD";
        let encrypted = enigma.encrypt_string(input);

        // Reset machine to decrypt
        let mut enigma2 = EnigmaMachine::new();
        let decrypted = enigma2.encrypt_string(&encrypted);

        assert_eq!(input, decrypted);
    }

    #[test]
    fn test_stepping() {
        // Enigma I, II, III.
        // Start: A A A
        // Notch positions: I=Q(16), II=E(4), III=V(21)
        // Right rotor (III) steps every time.
        // It hits V (21) -> next step triggers middle.

        let mut enigma = EnigmaMachine::new();
        // Right is III, pos 0. Notch at V (21).

        // Advance 21 times -> Right at V (21)
        for _ in 0..21 {
            enigma.step_rotors();
        }
        assert_eq!(enigma.right.position, 21); // V
        assert_eq!(enigma.middle.position, 0); // A

        // Next step: Right passes notch (V->W), triggers Middle step (A->B)
        enigma.step_rotors();
        assert_eq!(enigma.right.position, 22); // W
        assert_eq!(enigma.middle.position, 1); // B
    }

    #[test]
    fn test_known_vector() {
        // Source: https://en.wikipedia.org/wiki/Enigma_machine#Example
        // Settings: Rotors I, II, III. Start AAA. Rings AAA. No plugs.
        // Input:  AAAAA AAAAA
        // Output: BDZGO ... (Let's check just a few)

        // My default is I, II, III, AAA.
        let mut enigma = EnigmaMachine::new();
        let out = enigma.encrypt_string("AAAAA");
        assert_eq!(out, "BDZGO");
    }

    #[test]
    fn test_plugboard_swap() {
        let mut enigma = EnigmaMachine::new();
        enigma.plugboard.add_cable('A', 'C'); // Swap A and C (A->B is tricky if A maps to B naturally)

        // Verify the mapping directly
        assert_eq!(enigma.plugboard.process(0), 2); // A -> C
        assert_eq!(enigma.plugboard.process(2), 0); // C -> A
        assert_eq!(enigma.plugboard.process(1), 1); // B -> B

        // Verify encryption effect
        let clean_machine = EnigmaMachine::new();
        let mut swapped_machine = EnigmaMachine::new();
        swapped_machine.plugboard.add_cable('A', 'C');

        let out_clean = clean_machine.clone().encrypt_char('A');
        let out_swapped = swapped_machine.encrypt_char('A');

        assert_ne!(out_clean, out_swapped);
    }

    #[test]
    fn test_double_stepping() {
        let mut enigma = EnigmaMachine::new();
        // Setup:
        // Right (III): Notch V (21). Set to U (20).
        // Middle (II): Notch E (4). Set to D (3).
        // Left (I): Notch Q (16). Set to A (0).

        enigma.right.position = 20; // 'U'
        enigma.middle.position = 3; // 'D'
        enigma.left.position = 0;   // 'A'

        // Step 1: Right U->V. Middle stays D.
        enigma.step_rotors();
        assert_eq!(enigma.right.position, 21); // V
        assert_eq!(enigma.middle.position, 3); // D
        assert_eq!(enigma.left.position, 0);   // A

        // Step 2: Right V->W. Middle pushed D->E.
        enigma.step_rotors();
        assert_eq!(enigma.right.position, 22); // W
        assert_eq!(enigma.middle.position, 4); // E
        assert_eq!(enigma.left.position, 0);   // A

        // Step 3: Right W->X. Middle (at Notch E) steps E->F. Left steps A->B.
        // This is the "Double Step" - Middle stepped in Step 2 AND Step 3.
        enigma.step_rotors();
        assert_eq!(enigma.right.position, 23); // X
        assert_eq!(enigma.middle.position, 5); // F
        assert_eq!(enigma.left.position, 1);   // B
    }

    #[test]
    fn test_ring_settings() {
        let mut enigma1 = EnigmaMachine::new();
        let mut enigma2 = EnigmaMachine::new();

        // Change ring settings on enigma2
        enigma2.right.ring_setting = 1;
        enigma2.middle.ring_setting = 5;
        enigma2.left.ring_setting = 10;

        let out1 = enigma1.encrypt_string("AAAAA");
        let out2 = enigma2.encrypt_string("AAAAA");

        assert_ne!(out1, out2);
    }
}
