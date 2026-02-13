// Simple pulsing circle animation
let t = time();
let w = screen_w();
let h = screen_h();

let r = 50.0 + (sin(t * 2.0) * 20.0);
let x = w / 2.0 + (cos(t) * 100.0);
let y = h / 2.0 + (sin(t) * 100.0);

// Use mouse position for color
let mx = mouse_x();
let my = mouse_y();

let red = mx / w;
let green = my / h;
let blue = 0.5 + (sin(t) * 0.5);

circle(x, y, r, red, green, blue);

// Draw something at mouse
circle(mx, my, 10.0, 1.0, 1.0, 1.0);
