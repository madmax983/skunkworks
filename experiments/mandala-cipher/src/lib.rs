use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
    Circle,
    Square,
    Triangle,
    Diamond,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Purple,
    Cyan,
    White,
    Black,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Jewel {
    pub shape: Shape,
    pub color: Color,
}

impl Jewel {
    pub fn new(shape: Shape, color: Color) -> Self {
        Self { shape, color }
    }

    pub fn from_nibble(n: u8) -> Self {
        let n = n & 0xF;
        let shape = if n % 2 == 0 {
            Shape::Circle
        } else {
            Shape::Square
        };
        let color_idx = n / 2;
        let color = match color_idx {
            0 => Color::Red,
            1 => Color::Green,
            2 => Color::Blue,
            3 => Color::Yellow,
            4 => Color::Purple,
            5 => Color::Cyan,
            6 => Color::White,
            _ => Color::Black,
        };
        Jewel { shape, color }
    }

    pub fn to_nibble(&self) -> Option<u8> {
        let s = match self.shape {
            Shape::Circle => 0,
            Shape::Square => 1,
            _ => return None,
        };
        let c = match self.color {
            Color::Red => 0,
            Color::Green => 1,
            Color::Blue => 2,
            Color::Yellow => 3,
            Color::Purple => 4,
            Color::Cyan => 5,
            Color::White => 6,
            Color::Black => 7,
        };
        Some(c * 2 + s)
    }

    pub fn random_chaff() -> Self {
        let mut rng = rand::thread_rng();
        let shape = if rng.gen_bool(0.5) {
            Shape::Triangle
        } else {
            Shape::Diamond
        };
        let color = match rng.gen_range(0..8) {
            0 => Color::Red,
            1 => Color::Green,
            2 => Color::Blue,
            3 => Color::Yellow,
            4 => Color::Purple,
            5 => Color::Cyan,
            6 => Color::White,
            _ => Color::Black,
        };
        Jewel { shape, color }
    }
}

#[derive(Debug, Clone)]
pub struct MandalaConfig {
    pub rings: usize,
    pub segments_per_ring: usize,
    pub symmetry_order: usize,
}

impl Default for MandalaConfig {
    fn default() -> Self {
        Self {
            rings: 16,
            segments_per_ring: 8,
            symmetry_order: 12,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Mandala {
    pub jewels: Vec<Option<Jewel>>,
    pub config: MandalaConfig,
}

impl Mandala {
    pub fn new(config: MandalaConfig) -> Self {
        let capacity = config.rings * config.segments_per_ring;
        Self {
            jewels: vec![None; capacity],
            config,
        }
    }
}

pub fn encode(data: &[u8], config: &MandalaConfig) -> Mandala {
    let mut mandala = Mandala::new(config.clone());
    let capacity = mandala.jewels.len();

    let mut jewel_idx = 0;
    for &byte in data {
        if jewel_idx + 1 >= capacity {
            break;
        }
        let low = byte & 0xF;
        let high = (byte >> 4) & 0xF;

        mandala.jewels[jewel_idx] = Some(Jewel::from_nibble(low));
        jewel_idx += 1;
        mandala.jewels[jewel_idx] = Some(Jewel::from_nibble(high));
        jewel_idx += 1;
    }

    for i in jewel_idx..capacity {
        mandala.jewels[i] = Some(Jewel::random_chaff());
    }

    mandala
}

pub fn decode(mandala: &Mandala) -> Vec<u8> {
    let mut data = Vec::new();
    let mut current_byte: u8 = 0;
    let mut is_high_nibble = false;

    for jewel_opt in &mandala.jewels {
        if let Some(jewel) = jewel_opt {
            if let Some(nibble) = jewel.to_nibble() {
                if !is_high_nibble {
                    current_byte = nibble;
                    is_high_nibble = true;
                } else {
                    current_byte |= nibble << 4;
                    data.push(current_byte);
                    current_byte = 0;
                    is_high_nibble = false;
                }
            }
        }
    }

    data
}
