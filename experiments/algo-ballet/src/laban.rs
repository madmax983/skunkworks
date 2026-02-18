use crate::dancer::PoseTransition;

#[derive(Clone, Copy, Debug)]
pub enum Weight {
    Strong,
    Light,
}

#[derive(Clone, Copy, Debug)]
pub enum EffortTime {
    Sudden,
    Sustained,
}

#[derive(Clone, Copy, Debug)]
pub struct LabanEffort {
    pub weight: Weight,
    pub time: EffortTime,
}

impl LabanEffort {
    pub fn new(weight: Weight, time: EffortTime) -> Self {
        Self { weight, time }
    }

    pub fn apply(&self, transition: &mut PoseTransition) {
        // Base speed
        let mut base_speed = 5.0;

        match self.weight {
            Weight::Strong => {
                base_speed *= 0.8;
            },
            Weight::Light => {
                base_speed *= 1.2;
            }
        }

        match self.time {
            EffortTime::Sudden => {
                base_speed *= 2.0;
            },
            EffortTime::Sustained => {
                base_speed *= 0.5;
            }
        }

        transition.speed = base_speed;
    }
}
