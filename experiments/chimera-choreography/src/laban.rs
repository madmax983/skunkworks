#[derive(Debug, Clone, Copy, Default)]
pub struct LabanEffort {
    pub weight: f32, // 0.0 (Strong) -> 1.0 (Light)
    pub time: f32,   // 0.0 (Sudden) -> 1.0 (Sustained)
    pub space: f32,  // 0.0 (Direct) -> 1.0 (Indirect)
    pub flow: f32,   // 0.0 (Bound)  -> 1.0 (Free)
}

impl LabanEffort {
    pub fn new() -> Self {
        Self {
            weight: 0.5,
            time: 0.5,
            space: 0.5,
            flow: 0.5,
        }
    }

    pub fn is_strong(&self) -> bool { self.weight < 0.5 }
    pub fn is_light(&self) -> bool { self.weight >= 0.5 }

    pub fn is_sudden(&self) -> bool { self.time < 0.5 }
    pub fn is_sustained(&self) -> bool { self.time >= 0.5 }

    pub fn is_direct(&self) -> bool { self.space < 0.5 }
    pub fn is_indirect(&self) -> bool { self.space >= 0.5 }

    pub fn is_bound(&self) -> bool { self.flow < 0.5 }
    pub fn is_free(&self) -> bool { self.flow >= 0.5 }
}
