#[derive(Debug, Clone, Copy)]
pub struct TerrainPoint {
    pub height: f32,
    pub hardness: f32,
}

#[derive(Debug, Clone)]
pub struct Phoneme {
    pub char: char,
    pub target_height: f32,
    pub target_hardness: f32,
}

impl Phoneme {
    pub const fn new(char: char, height: f32, hardness: f32) -> Self {
        Self { char, target_height: height, target_hardness: hardness }
    }
}

// Minimal phoneme inventory
pub const PHONEME_TABLE: &[Phoneme] = &[
    Phoneme::new('a', 0.1, 0.1),
    Phoneme::new('e', 0.15, 0.1),
    Phoneme::new('i', 0.2, 0.1),
    Phoneme::new('o', 0.12, 0.1),
    Phoneme::new('u', 0.18, 0.1),

    Phoneme::new('p', 0.9, 0.9),
    Phoneme::new('t', 0.92, 0.95),
    Phoneme::new('k', 0.88, 0.9),
    Phoneme::new('b', 0.85, 0.8),
    Phoneme::new('d', 0.87, 0.85),
    Phoneme::new('g', 0.82, 0.8),

    Phoneme::new('f', 0.5, 0.4),
    Phoneme::new('s', 0.55, 0.5),
    Phoneme::new('v', 0.48, 0.4),
    Phoneme::new('z', 0.52, 0.5),
    Phoneme::new('h', 0.4, 0.2),

    Phoneme::new('m', 0.3, 0.3),
    Phoneme::new('n', 0.35, 0.35),
    Phoneme::new('r', 0.32, 0.4),
    Phoneme::new('l', 0.28, 0.3),

    Phoneme::new(' ', 0.0, 0.0), // Silence
];

pub fn char_to_terrain(c: char) -> TerrainPoint {
    let lower_c = c.to_ascii_lowercase();

    if let Some(ph) = PHONEME_TABLE.iter().find(|p| p.char == lower_c) {
        TerrainPoint { height: ph.target_height, hardness: ph.target_hardness }
    } else {
        // Unknown chars treated as generic noise/hills
        TerrainPoint { height: 0.2, hardness: 0.2 }
    }
}

pub fn terrain_to_char(point: &TerrainPoint) -> char {
    let mut best_char = '?';
    let mut min_dist = f32::MAX;

    for ph in PHONEME_TABLE {
        let dh = point.height - ph.target_height;
        let dhard = point.hardness - ph.target_hardness;
        let dist = dh * dh + dhard * dhard;

        if dist < min_dist {
            min_dist = dist;
            best_char = ph.char;
        }
    }

    best_char
}
