use crate::components::*;
use bevy::prelude::*;
use std::f32::consts::PI;

pub struct CpuPlugin;

impl Plugin for CpuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cpu_display);
        app.add_systems(Update, update_cpu_state);
    }
}

fn spawn_cpu_display(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::FlexStart,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "CPU STATE: HALTED",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::GOLD,
                        ..default()
                    },
                ),
                CpuStateDisplay,
            ));
        });
}

fn update_cpu_state(
    mut query: Query<&mut Text, With<CpuStateDisplay>>,
    wheel_query: Query<&Transform, With<EscapeWheel>>,
) {
    let Ok(wheel_transform) = wheel_query.get_single() else {
        return;
    };

    // Get rotation in radians (Z-axis)
    let rotation = wheel_transform.rotation.to_euler(EulerRot::XYZ).2;
    // Normalize to 0..2PI
    let mut angle = rotation % (2.0 * PI);
    if angle < 0.0 {
        angle += 2.0 * PI;
    }

    let state = if angle < PI / 2.0 {
        "FETCH"
    } else if angle < PI {
        "DECODE"
    } else if angle < 3.0 * PI / 2.0 {
        "EXECUTE"
    } else {
        "WRITEBACK"
    };

    let color = match state {
        "FETCH" => Color::CYAN,
        "DECODE" => Color::FUCHSIA,
        "EXECUTE" => Color::RED,
        "WRITEBACK" => Color::GREEN,
        _ => Color::WHITE,
    };

    for mut text in query.iter_mut() {
        text.sections[0].value = format!("CPU STATE: {} ({:.1} deg)", state, angle.to_degrees());
        text.sections[0].style.color = color;
    }
}
