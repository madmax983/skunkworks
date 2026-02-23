use crate::sediment::SedimentParticle;
use ::rand::Rng;
use chimera_lang::prelude::*;
use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub enum DetritivoreState {
    Foraging,
    Moving(Vec2),
}

pub struct Detritivore {
    pub vm: ChimeraVM,
    pub position: Vec2,
    pub state: DetritivoreState,
    pub color: Color,
    pub size: f32,
    pub energy: i64,
}

impl Detritivore {
    pub fn new(pos: Vec2) -> Self {
        // Simple DNA
        let genes = vec![
            Gene::new(OpCode::Incubate, vec![Nucleotide::from(1)]),
            Gene::new(OpCode::Photosynthesize, vec![]),
        ];

        let strand = Strand { genes };
        let helix = Helix {
            strands: vec![strand],
        };
        let dna = Dna {
            evolution_config: None,
            helix,
        };

        let mut vm = ChimeraVM::new(dna);
        vm.energy = 50;

        let mut rng = ::rand::thread_rng();
        Self {
            vm,
            position: pos,
            state: DetritivoreState::Foraging,
            color: Color::from_rgba(rng.gen(), rng.gen(), rng.gen(), 255),
            size: rng.gen_range(3.0..6.0),
            energy: 50,
        }
    }

    pub fn update(
        &mut self,
        dt: f32,
        sediment: &mut Vec<SedimentParticle>,
        ground_level: f32,
    ) -> Option<Detritivore> {
        let mut child = None;

        // Metabolism
        if rand::gen_range(0, 100) < 5 {
            self.energy -= 1;
        }

        // Find nearest food
        let mut target_idx = None;
        let mut min_dist = 200.0; // Vision range

        for (i, p) in sediment.iter().enumerate() {
            let dist = self.position.distance(p.pos);
            if dist < min_dist {
                min_dist = dist;
                target_idx = Some(i);
            }
        }

        if let Some(idx) = target_idx {
            // Move towards it
            let target_pos = sediment[idx].pos;
            let dir = (target_pos - self.position).normalize_or_zero();
            self.position += dir * 100.0 * dt;

            // Eat
            if self.position.distance(target_pos) < 5.0 {
                self.energy += (sediment[idx].mass * 5.0) as i64;
                sediment.remove(idx);
            }
        } else {
            // Random wander
            let mut rng = ::rand::thread_rng();
            if rng.gen_bool(0.02) {
                let target = vec2(rng.gen_range(0.0..screen_width()), ground_level - 10.0);
                self.state = DetritivoreState::Moving(target);
            }

            if let DetritivoreState::Moving(target) = self.state {
                let dir = (target - self.position).normalize_or_zero();
                self.position += dir * 30.0 * dt;
                if self.position.distance(target) < 5.0 {
                    self.state = DetritivoreState::Foraging;
                }
            }
        }

        // Clamp to ground
        if self.position.y > ground_level {
            self.position.y = ground_level;
        }

        // Mitosis
        if self.energy > 150 {
            self.energy /= 2;
            let mut c = Detritivore::new(self.position);
            c.energy = self.energy;
            child = Some(c);
        }

        if self.energy <= 0 {
            // Die?
            // For now, just reset energy to prevent instant death loop, or remove from main.
            // Handled in main.
        }

        child
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.size, self.color);
        // Antennae
        draw_line(
            self.position.x,
            self.position.y,
            self.position.x - 5.0,
            self.position.y - 10.0,
            1.0,
            self.color,
        );
        draw_line(
            self.position.x,
            self.position.y,
            self.position.x + 5.0,
            self.position.y - 10.0,
            1.0,
            self.color,
        );

        // Energy Indicator
        if self.energy > 0 {
            let ratio = self.energy as f32 / 100.0;
            draw_circle(self.position.x, self.position.y, self.size * ratio, WHITE);
        }
    }
}
