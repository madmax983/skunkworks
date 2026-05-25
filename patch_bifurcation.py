import re

with open('experiments/bifurcation-crawler/src/main.rs', 'r') as f:
    content = f.read()

# remove duplicate main
content = content.replace('''fn main() {
    if std::env::args().any(|arg| arg == "--headless") {
        println!("Running in headless mode, exiting immediately to avoid XOpenDisplay panic.");
        return;
    }
    macroquad::Window::from_config(window_conf(), async_main());
}

fn main() {
    if std::env::args().any(|arg| arg == "--headless") {
        println!("Running in headless mode, exiting immediately to avoid XOpenDisplay panic.");
        return;
    }
    macroquad::Window::from_config(window_conf(), async_main());
}''', '''fn main() {
    if std::env::args().any(|arg| arg == "--headless") {
        println!("Running in headless mode, exiting immediately to avoid XOpenDisplay panic.");
        return;
    }
    macroquad::Window::from_config(window_conf(), async_main());
}''')

with open('experiments/bifurcation-crawler/src/main.rs', 'w') as f:
    f.write(content)
