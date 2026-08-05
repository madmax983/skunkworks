use hyper_system::{SystemMonitor, Vec4 as HyperVec4};
use miller_lattice::Crystal;
use std::env;
use std::path::Path;

fn run_headless() {
    println!("Running in headless mode. Bypassing macroquad window.");
    let crystal = Crystal::build_from_path(Path::new(".")).unwrap();
    println!("Crystal built with {} atoms", crystal.atoms.len());
    let mut monitor = SystemMonitor::new();
    monitor.update_with_time(0.016, 0.0);
    let point = HyperVec4::new(1.0, 1.0, 1.0, 0.0);
    let rotated = point.rotate_xw(monitor.cpu_usage);
    println!("Test point rotated: {:?}", rotated);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--headless".to_string()) {
        run_headless();
        return;
    }

    #[cfg(feature = "macroquad_run")]
    macroquad_main::main();
}

#[cfg(feature = "macroquad_run")]
mod macroquad_main {
    use super::*;
    use macroquad::prelude::*;

    fn window_conf() -> Conf {
        Conf {
            window_title: "Hyper Lattice".to_owned(),
            ..Default::default()
        }
    }

    #[macroquad::main(window_conf)]
    pub async fn main() {
        let crystal = Crystal::build_from_path(Path::new(".")).unwrap();
        let mut monitor = SystemMonitor::new();
        let mut time = 0.0;

        loop {
            monitor.update_with_time(get_frame_time(), get_time());
            time += 0.01 + monitor.cpu_usage * 0.1;

            clear_background(BLACK);

            set_camera(&Camera3D {
                position: vec3(0.0, 0.0, 20.0),
                up: vec3(0.0, 1.0, 0.0),
                target: vec3(0.0, 0.0, 0.0),
                ..Default::default()
            });

            for bond in &crystal.bonds {
                let atom1 = &crystal.atoms[bond.0];
                let atom2 = &crystal.atoms[bond.1];

                let p1_4d = HyperVec4::new(
                    atom1.position.x as f32,
                    atom1.position.y as f32,
                    atom1.position.z as f32,
                    0.0,
                )
                .rotate_xw(time);
                let p2_4d = HyperVec4::new(
                    atom2.position.x as f32,
                    atom2.position.y as f32,
                    atom2.position.z as f32,
                    0.0,
                )
                .rotate_xw(time);

                let p1_3d = p1_4d.project_to_3d(10.0);
                let p2_3d = p2_4d.project_to_3d(10.0);

                draw_line_3d(
                    vec3(p1_3d.x, p1_3d.y, p1_3d.z),
                    vec3(p2_3d.x, p2_3d.y, p2_3d.z),
                    GREEN,
                );
            }

            for atom in &crystal.atoms {
                let p_4d = HyperVec4::new(
                    atom.position.x as f32,
                    atom.position.y as f32,
                    atom.position.z as f32,
                    0.0,
                )
                .rotate_xw(time);
                let p_3d = p_4d.project_to_3d(10.0);

                draw_sphere(vec3(p_3d.x, p_3d.y, p_3d.z), 0.2, None, RED);
            }

            set_default_camera();
            draw_text(
                format!(
                    "CPU Stress (Rotation Speed): {:.2}%",
                    monitor.cpu_usage * 100.0
                ),
                10.0,
                20.0,
                20.0,
                WHITE,
            );

            next_frame().await;
        }
    }
}
