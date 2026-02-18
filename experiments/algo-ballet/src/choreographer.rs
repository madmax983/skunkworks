use bevy::prelude::*;
use std::collections::VecDeque;
use crate::dancer::{DancePose, PoseTransition, TargetPosition};
use crate::laban::{LabanEffort, Weight, EffortTime};

#[derive(Resource)]
pub struct SortDirector {
    pub array: Vec<usize>,
    pub dancers: Vec<Entity>,
    pub state: SortState,
    pub algorithm: Box<dyn SortAlgorithm + Send + Sync>,
    pub instruction_queue: VecDeque<SortInstruction>,
    pub current_wait: f32,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum SortState {
    Idle,
    Running,
    Finished,
}

pub trait SortAlgorithm {
    fn next_step(&mut self, array: &mut [usize]) -> Option<Vec<SortInstruction>>;
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum SortInstruction {
    Compare(usize, usize),
    Swap(usize, usize),
    Highlight(usize),
    Wait(f32),
}

// Implement Bubble Sort
pub struct BubbleSort {
    i: usize,
    j: usize,
    len: usize,
}

impl BubbleSort {
    pub fn new(len: usize) -> Self {
        Self { i: 0, j: 0, len }
    }
}

impl SortAlgorithm for BubbleSort {
    fn next_step(&mut self, array: &mut [usize]) -> Option<Vec<SortInstruction>> {
        if self.i >= self.len {
            return None;
        }

        if self.j >= self.len.saturating_sub(1 + self.i) {
            self.i += 1;
            self.j = 0;
            if self.i >= self.len {
                return None;
            }
        }

        let j = self.j;
        let next_j = j + 1;

        // Safety check
        if next_j >= self.len {
             self.j += 1;
             return Some(vec![]);
        }

        let mut instructions = vec![];
        instructions.push(SortInstruction::Compare(j, next_j));
        instructions.push(SortInstruction::Wait(0.5));

        if array[j] > array[next_j] {
            array.swap(j, next_j);
            instructions.push(SortInstruction::Swap(j, next_j));
            instructions.push(SortInstruction::Wait(1.0));
        }

        self.j += 1;
        Some(instructions)
    }
}

pub fn director_system(
    time: Res<Time>,
    mut director: ResMut<SortDirector>,
    mut query: Query<(&mut DancePose, &mut TargetPosition, &mut PoseTransition)>,
) {
    if director.state != SortState::Running {
        return;
    }

    if director.current_wait > 0.0 {
        director.current_wait -= time.delta_seconds();
        return;
    }

    if let Some(instruction) = director.instruction_queue.pop_front() {
        match instruction {
            SortInstruction::Wait(duration) => {
                director.current_wait = duration;
            },
            SortInstruction::Compare(a, b) => {
                info!("Comparing indices {} and {}", a, b);
                let entity_a = director.dancers[a];
                let entity_b = director.dancers[b];

                if let Ok([mut q_a, mut q_b]) = query.get_many_mut([entity_a, entity_b]) {
                    // q_a: (pose, target, transition)
                    let effort = LabanEffort::new(Weight::Light, EffortTime::Sustained);

                    effort.apply(&mut q_a.2);
                    effort.apply(&mut q_b.2);

                    // Pose: Look at each other
                    // A is left, looks right
                    q_a.0.head_tilt = -0.5;
                    q_a.0.right_arm_angle = 1.0;

                    // B is right, looks left
                    q_b.0.head_tilt = 0.5;
                    q_b.0.left_arm_angle = -1.0;
                }
            },
            SortInstruction::Swap(a, b) => {
                info!("Swapping indices {} and {}", a, b);
                let entity_a = director.dancers[a];
                let entity_b = director.dancers[b];

                if let Ok([mut q_a, mut q_b]) = query.get_many_mut([entity_a, entity_b]) {
                    // Swap positions
                    let pos_a = q_a.1.0;
                    let pos_b = q_b.1.0;

                    // Move A to B's spot, B to A's spot
                    q_a.1.0 = pos_b;
                    q_b.1.0 = pos_a;

                    let effort = LabanEffort::new(Weight::Strong, EffortTime::Sudden);
                    effort.apply(&mut q_a.2);
                    effort.apply(&mut q_b.2);

                    // Pose: Leap
                    for q in [&mut q_a, &mut q_b] {
                         q.0.left_leg_angle = 0.5;
                         q.0.right_leg_angle = -0.5;
                         q.0.left_arm_angle = 2.5;
                         q.0.right_arm_angle = -2.5;
                         q.0.head_tilt = 0.0;
                         q.0.torso_bend = 0.0;
                    }
                }

                director.dancers.swap(a, b);
            },
            SortInstruction::Highlight(_idx) => {
                // Highlight logic (maybe pulse color?)
            }
        }
    } else {
        // Get next instructions
        let director = director.into_inner();
        if let Some(new_instructions) = director.algorithm.next_step(&mut director.array) {
            director.instruction_queue.extend(new_instructions);
        } else {
            director.state = SortState::Finished;
            info!("Sorting Finished!");
            // Reset poses?
        }
    }
}
