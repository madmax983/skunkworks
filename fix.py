with open("experiments/poincare-resonance/src/main.rs", "r") as f:
    code = f.read()

code = code.replace('#[macroquad::main("Poincaré Resonance")]\nasync fn main() {', '''fn window_conf() -> macroquad::window::Conf {
    macroquad::window::Conf {
        window_title: "Poincaré Resonance".to_owned(),
        ..Default::default()
    }
}

fn main() {
    if std::env::args().any(|arg| arg == "--headless") {
        return;
    }
    macroquad::Window::from_config(window_conf(), async_main());
}

async fn async_main() {''')

with open("experiments/poincare-resonance/src/main.rs", "w") as f:
    f.write(code)
