use std::collections::VecDeque;
use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub struct Gear {
    pub teeth: u32,
    pub angle: f32,
    pub radius: f32,
    pub children: Vec<usize>,
    pub parent: Option<usize>,
}

impl Gear {
    pub fn new(teeth: u32, radius: f32) -> Self {
        Self {
            teeth,
            angle: 0.0,
            radius,
            children: Vec::new(),
            parent: None,
        }
    }
}

pub struct GearTrain {
    pub gears: Vec<Gear>,
}

impl GearTrain {
    pub fn new() -> Self {
        Self { gears: Vec::new() }
    }

    pub fn add_gear(&mut self, gear: Gear) -> usize {
        let id = self.gears.len();
        self.gears.push(gear);
        id
    }

    pub fn connect(&mut self, driver_id: usize, driven_id: usize) {
        if let Some(driver) = self.gears.get_mut(driver_id) {
            driver.children.push(driven_id);
        }
        if let Some(driven) = self.gears.get_mut(driven_id) {
            driven.parent = Some(driver_id);
        }
    }

    pub fn rotate_input(&mut self, start_gear_id: usize, initial_delta: f32) {
        let mut queue = VecDeque::new();
        queue.push_back((start_gear_id, initial_delta));

        let mut iterations = 0;
        let max_iterations = 1000;

        while let Some((current_id, d_theta)) = queue.pop_front() {
            if iterations > max_iterations {
                break;
            }
            iterations += 1;

            let (teeth_in, children) = {
                if let Some(g) = self.gears.get_mut(current_id) {
                    g.angle += d_theta;
                    (g.teeth, g.children.clone())
                } else {
                    continue;
                }
            };

            for child_id in children {
                if let Some(child) = self.gears.get(child_id) {
                    let child_teeth = child.teeth;
                    let ratio = -(teeth_in as f32) / (child_teeth as f32);
                    let child_delta = d_theta * ratio;
                    queue.push_back((child_id, child_delta));
                }
            }
        }
    }
}

pub struct CipherMachine {
    pub train: GearTrain,
    pub drive_shaft: usize,  // Advances 1 tick per operation
    pub key_rotor: usize,    // Driven by shaft
    pub input_handle: usize, // User input
    pub diff_output: usize,  // Result
    pub current_input_char: char,
}

impl CipherMachine {
    pub fn new() -> Self {
        let mut train = GearTrain::new();

        // 1. Input Handle (26 teeth) - User rotates this.
        let input_handle = train.add_gear(Gear::new(26, 50.0));

        // 2. Drive Shaft (26 teeth) - Advances time.
        let drive_shaft = train.add_gear(Gear::new(26, 50.0));

        // 3. Key Rotor (Prime teeth, e.g. 19). Driven by Drive Shaft.
        // Ratio: 26 -> 19. Rotation propagates.
        let key_rotor = train.add_gear(Gear::new(19, 40.0));
        train.connect(drive_shaft, key_rotor);

        // 4. Differential Output (26 teeth).
        // It sums rotation from Input Handle and Key Rotor.
        // Input Handle -> Output (Ratio 26:26 = -1)
        // Key Rotor -> Output (Ratio 19:26 = -19/26)
        let output = train.add_gear(Gear::new(26, 50.0));

        train.connect(input_handle, output);
        train.connect(key_rotor, output);

        Self {
            train,
            drive_shaft,
            key_rotor,
            input_handle,
            diff_output: output,
            current_input_char: 'A',
        }
    }

    pub fn encrypt(&mut self, c: char) -> char {
        // 1. Advance Time (Drive Shaft)
        // Rotate by 1 unit (2PI/26)
        let unit_angle = 2.0 * PI / 26.0;
        self.train.rotate_input(self.drive_shaft, unit_angle);

        // 2. Rotate Input Handle to match 'c'
        let target_idx = (c as u8).saturating_sub(b'A') as i32;
        let current_idx = (self.current_input_char as u8).saturating_sub(b'A') as i32;
        let diff = target_idx - current_idx;
        let rotation = diff as f32 * unit_angle;

        self.train.rotate_input(self.input_handle, rotation);
        self.current_input_char = c;

        // 3. Read Output
        // Normalize angle to 0..2PI
        let angle = self.train.gears[self.diff_output].angle;
        let normalized = (angle % (2.0 * PI) + 2.0 * PI) % (2.0 * PI);

        // Map to 26 buckets
        // Note: Gears invert direction, so we might be moving backwards.
        // Angle 0 -> 'A'.
        // Step size = 2PI/26.
        let step = 2.0 * PI / 26.0;
        let idx = (normalized / step).round() as i32 % 26;

        (b'A' + idx as u8) as char
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gear_ratio_simple() {
        let mut train = GearTrain::new();
        let input = train.add_gear(Gear::new(10, 10.0));
        let output = train.add_gear(Gear::new(20, 20.0));
        train.connect(input, output);
        train.rotate_input(input, PI);
        let expected = -PI / 2.0;
        assert!((train.gears[output].angle - expected).abs() < 1e-5);
    }

    #[test]
    fn test_gear_chain() {
        let mut train = GearTrain::new();
        let g1 = train.add_gear(Gear::new(10, 10.0));
        let g2 = train.add_gear(Gear::new(10, 10.0));
        let g3 = train.add_gear(Gear::new(10, 10.0));
        train.connect(g1, g2);
        train.connect(g2, g3);
        train.rotate_input(g1, PI);
        assert!((train.gears[g2].angle - (-PI)).abs() < 1e-5);
        assert!((train.gears[g3].angle - PI).abs() < 1e-5);
    }
}

#[cfg(test)]
mod cipher_tests {
    use super::*;

    #[test]
    fn test_cipher_simple() {
        let mut machine = CipherMachine::new();
        let encrypted = machine.encrypt('B');
        assert_ne!(encrypted, 'B'); // Should change

        // Determinism check
        // Reset machine manually? Hard to reset without new().
        let mut machine2 = CipherMachine::new();
        let encrypted2 = machine2.encrypt('B');
        assert_eq!(encrypted, encrypted2);
    }

    #[test]
    fn test_cipher_sequence() {
        let mut machine = CipherMachine::new();
        let c1 = machine.encrypt('A');
        let c2 = machine.encrypt('A');
        // Because Drive Shaft advances, encrypting 'A' twice should yield different results
        assert_ne!(c1, c2, "Cipher should be polyalphabetic (time-variant)");
    }
}
