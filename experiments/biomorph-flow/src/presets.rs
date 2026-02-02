
#[derive(Debug, Clone)]
pub struct Preset {
    pub name: &'static str,
    pub f: f64,
    pub k: f64,
}

pub const PRESETS: &[Preset] = &[
    Preset { name: "Coral", f: 0.0545, k: 0.062 },
    Preset { name: "Mitosis", f: 0.0367, k: 0.06448 },
    Preset { name: "U-Skate", f: 0.062, k: 0.0609 },
    Preset { name: "Loops", f: 0.082, k: 0.06 },
    Preset { name: "Solitons", f: 0.03, k: 0.062 },
    Preset { name: "Worms", f: 0.078, k: 0.061 },
    Preset { name: "Chaos", f: 0.026, k: 0.051 },
];
