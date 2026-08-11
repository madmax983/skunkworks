# Arthropod 🐜

A simple library to create UI components.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
arthropod = { path = "../crates/arthropod" }
macroquad = "0.4"
```

## Quick Start

```rust
use arthropod::Button;
use macroquad::prelude::*;

// NOTE: To run this in CI or a headless server without X11, you must bypass macroquad.
// For tests, you can skip execution entirely if `--headless` is passed.


#[macroquad::main("UI Example")]
async fn main() {
    if std::env::args().any(|arg| arg == "--headless") {
        println!("Running in headless mode, exiting immediately to avoid XOpenDisplay panic.");
        return;
    }

    let start_btn = Button::new("Start", 100.0, 100.0, 200.0, 50.0)
        .with_colors(GREEN, LIME, DARKGREEN);

    loop {
        clear_background(BLACK);

        if start_btn.draw() {
            println!("Start button was clicked!");
        }

        next_frame().await;
    }
}
```

## Details

### Handling Extreme Geometry

To maintain stability and prevent crashes from geometry explosion (such as the `XOpenDisplay` crash reported by the Havoc/Mosaic personas), the `Button::draw` method implements early returns for extreme coordinate values. Any bounds coordinates where `x > 100_000.0` or `y > 100_000.0` will immediately return `false` without evaluating mouse intersections.
