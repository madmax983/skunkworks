use crate::monitor::SystemMonitor;
use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct LabanState {
    pub weight: f32, // 0.0 (Light) to 1.0 (Heavy/Strong)
    pub time: f32,   // 0.0 (Sudden/Quick) to 1.0 (Sustained/Slow)
    pub space: f32,  // 0.0 (Direct) to 1.0 (Indirect/Flexible)
    pub flow: f32,   // 0.0 (Bound) to 1.0 (Free)
}

pub fn update_laban_from_monitor(monitor: Res<SystemMonitor>, mut laban: ResMut<LabanState>) {
    // RAM -> Weight
    // High RAM usage = Heavy movements (Strong Weight)
    laban.weight = monitor.ram_usage.clamp(0.0, 1.0);

    // CPU -> Time
    // High CPU usage = Sudden/Quick movements (Low Time value)
    // Low CPU usage = Sustained/Slow movements (High Time value)
    // Map 0..100 to 1.0..0.0
    let cpu_normalized = (monitor.cpu_usage / 100.0).clamp(0.0, 1.0);
    laban.time = 1.0 - cpu_normalized;

    // Flow:
    // High CPU = Bound Flow (Tension/Rigid/Controlled) -> 0.0
    // Low CPU = Free Flow (Relaxed/Loose) -> 1.0
    laban.flow = (1.0 - cpu_normalized).clamp(0.0, 1.0);

    // Space:
    // Map RAM to Space? High RAM = Large Space (Indirect)
    laban.space = monitor.ram_usage.clamp(0.0, 1.0);
}
