use macroquad::prelude::*;
use arthropod::Button;
use neuro_sim::Izhikevich;

fn window_conf() -> Conf {
    Conf {
        window_title: "Arthropod × Neuro Sim".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--headless".to_string()) {
        println!("Running in headless mode. Bypassing macroquad initialization.");
        return;
    }

    macroquad::Window::from_config(window_conf(), async_main());
}

async fn async_main() {
    let mut neuron = Izhikevich::new_regular_spiking();

    // Create the inject current button from arthropod
    let inject_btn = Button::new("Inject Current", 300.0, 500.0, 200.0, 50.0)
        .with_colors(GREEN, LIME, DARKGREEN);

    loop {
        clear_background(color_u8!(20, 20, 30, 255));

        // Step the simulation
        neuron.update(1.0, 0.0);

        // Draw visualizing circle mapping to membrane potential (v)
        // Typically v is -65 at resting, and spikes to +30.
        let radius = (neuron.v + 80.0).max(5.0) * 0.5;
        draw_circle(400.0, 250.0, radius as f32, WHITE);

        // Handle UI interaction
        if inject_btn.draw() {
            neuron.inject(50.0);
            println!("Injected current! v={}", neuron.v);
        }

        next_frame().await;
    }
}
