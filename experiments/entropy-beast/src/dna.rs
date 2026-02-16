use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub struct Creature {
    pub head: Head,
    pub limbs: Vec<Limb>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Head {
    pub size: f32,
    pub eye_count: u8,
    pub color: [u8; 3],
}

#[derive(Debug, Clone, PartialEq)]
pub struct Limb {
    pub length: f32,
    pub thickness: f32,
    pub joints: u8,
    pub children: Vec<Limb>,
}

// Markers
pub const MAGIC: &[u8] = b"DNA\x01";
pub const HEAD_MARKER: u8 = 0xAA;
pub const LIMB_MARKER: u8 = 0xBB;

impl Creature {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let head = Head {
            size: rng.gen_range(10.0..30.0),
            eye_count: rng.gen_range(1..6),
            color: [rng.gen(), rng.gen(), rng.gen()],
        };

        let num_limbs = rng.gen_range(2..6);
        let mut limbs = Vec::new();
        for _ in 0..num_limbs {
            limbs.push(Limb::random(&mut rng, 0));
        }

        Creature { head, limbs }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(MAGIC);

        // Head
        data.push(HEAD_MARKER);
        data.extend_from_slice(&self.head.size.to_le_bytes());
        data.push(self.head.eye_count);
        data.extend_from_slice(&self.head.color);
        // Simple checksum: XOR of data bytes (excluding marker)
        let mut checksum = 0u8;
        for b in &data[data.len()-8..] { // size(4) + eye(1) + color(3) = 8
            checksum ^= b;
        }
        data.push(checksum);

        // Limbs
        for limb in &self.limbs {
            limb.serialize_into(&mut data);
        }

        data
    }
}

impl Limb {
    pub fn random<R: Rng>(rng: &mut R, depth: u8) -> Self {
        let length = rng.gen_range(20.0..60.0) / (depth as f32 + 1.0);
        let thickness = rng.gen_range(2.0..8.0) / (depth as f32 + 1.0);
        let joints = rng.gen_range(0..3);

        let mut children = Vec::new();
        if depth < 2 && rng.gen_bool(0.6) {
             let num_children = rng.gen_range(1..3);
             for _ in 0..num_children {
                 children.push(Limb::random(rng, depth + 1));
             }
        }

        Limb { length, thickness, joints, children }
    }

    pub fn serialize_into(&self, data: &mut Vec<u8>) {
        data.push(LIMB_MARKER);
        let start_idx = data.len();

        data.extend_from_slice(&self.length.to_le_bytes());
        data.extend_from_slice(&self.thickness.to_le_bytes());
        data.push(self.joints);
        data.push(self.children.len() as u8);

        // Checksum for this limb header
        let mut checksum = 0u8;
        for b in &data[start_idx..] {
            checksum ^= b;
        }
        data.push(checksum);

        for child in &self.children {
            child.serialize_into(data);
        }
    }
}
