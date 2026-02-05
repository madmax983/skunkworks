use bevy::prelude::*;
use chimera_lang::vm::ChimeraVM;

// Event emitted by the mechanism when a tick occurs
#[derive(Event)]
pub struct TickEvent;

// Component holding the VM
#[derive(Component)]
pub struct ChimeraComponent {
    pub vm: ChimeraVM,
}

#[derive(Component)]
pub struct VmStatusText;

pub struct VmBridgePlugin;

impl Plugin for VmBridgePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TickEvent>()
            .add_systems(Startup, setup_ui)
            .add_systems(Update, (step_vm_system, sync_ui_system));
    }
}

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "Initializing Chimera...",
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
        VmStatusText,
    ));
}

fn step_vm_system(mut events: EventReader<TickEvent>, mut query: Query<&mut ChimeraComponent>) {
    for _ in events.read() {
        for mut chimera in &mut query {
            if !chimera.vm.halted {
                chimera.vm.step();
                info!(
                    "VM Stepped! IP: {:?}, Stack: {:?}",
                    chimera.vm.ip, chimera.vm.stack
                );
            }
        }
    }
}

fn sync_ui_system(
    chimera_query: Query<&ChimeraComponent, Changed<ChimeraComponent>>,
    mut text_query: Query<&mut Text, With<VmStatusText>>,
) {
    if let Ok(chimera) = chimera_query.get_single() {
        if let Ok(mut text) = text_query.get_single_mut() {
            let top = if let Some(val) = chimera.vm.stack.last() {
                format!("{}", val)
            } else {
                "Empty".to_string()
            };

            text.sections[0].value = format!(
                "Status: {}\nIP: {:?}\nEnergy: {}\nStack Top: {}",
                if chimera.vm.halted {
                    "HALTED"
                } else {
                    "RUNNING"
                },
                chimera.vm.ip,
                chimera.vm.energy,
                top
            );
        }
    }
}
