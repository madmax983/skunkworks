use crate::mechanism::{LogicLever, Pin};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct CpuPlugin;

impl Plugin for CpuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CpuState::default())
            .add_event::<InstructionTriggered>()
            .add_systems(Startup, setup_ui)
            .add_systems(Update, (instruction_trigger_system, update_ui));
    }
}

#[derive(Resource, Default)]
pub struct CpuState {
    pub pc: usize,
    pub accumulator: i32,
    pub instruction_history: Vec<String>,
}

#[derive(Event)]
pub struct InstructionTriggered {
    pub note_index: usize,
    pub instruction: String,
}

#[derive(Component)]
struct CpuDisplay;

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "CPU State: INIT",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        CpuDisplay,
    ));
}

fn instruction_trigger_system(
    mut collision_events: EventReader<CollisionEvent>,
    pin_query: Query<&Pin>,
    lever_query: Query<&LogicLever>,
    mut cpu_state: ResMut<CpuState>,
    mut trigger_events: EventWriter<InstructionTriggered>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            // Check if one is Pin and one is Lever
            let pin_ent = if pin_query.contains(*e1) {
                *e1
            } else if pin_query.contains(*e2) {
                *e2
            } else {
                continue;
            };
            let lever_ent = if lever_query.contains(*e1) {
                *e1
            } else if lever_query.contains(*e2) {
                *e2
            } else {
                continue;
            };

            // Only handle Pin -> Lever collision
            if let Ok(pin) = pin_query.get(pin_ent) {
                if let Ok(_lever) = lever_query.get(lever_ent) {
                    let instr_str = match pin.note_index {
                        0 => {
                            cpu_state.accumulator += 1;
                            "INC"
                        }
                        1 => {
                            cpu_state.accumulator -= 1;
                            "DEC"
                        }
                        2 => {
                            cpu_state.accumulator += 5;
                            "ADD 5"
                        }
                        3 => {
                            cpu_state.accumulator -= 2;
                            "SUB 2"
                        }
                        4 => {
                            cpu_state.accumulator = 0;
                            "CLR"
                        }
                        _ => "NOP",
                    };

                    cpu_state.pc += 1;
                    let pc = cpu_state.pc;
                    cpu_state
                        .instruction_history
                        .push(format!("{}: {}", pc, instr_str));
                    if cpu_state.instruction_history.len() > 5 {
                        cpu_state.instruction_history.remove(0);
                    }

                    trigger_events.send(InstructionTriggered {
                        note_index: pin.note_index,
                        instruction: instr_str.to_string(),
                    });

                    info!("EXEC: {}", instr_str);
                }
            }
        }
    }
}

fn update_ui(cpu_state: Res<CpuState>, mut query: Query<&mut Text, With<CpuDisplay>>) {
    if cpu_state.is_changed() {
        if let Ok(mut text) = query.get_single_mut() {
            let history = cpu_state.instruction_history.join("\n");
            text.sections[0].value = format!(
                "PC: {}\nACC: {}\n\nLast Instructions:\n{}",
                cpu_state.pc, cpu_state.accumulator, history
            );
        }
    }
}
