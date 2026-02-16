use macroquad::prelude::*;
use ::rand::Rng;

mod mechanism;
use mechanism::{Differential, Integrator};

const GEAR_COLOR: Color = GOLD;
const SHAFT_COLOR: Color = LIGHTGRAY;
const BID_COLOR: Color = GREEN;
const ASK_COLOR: Color = RED;

struct Order {
    pos: Vec2,
    vel: Vec2,
    order_type: OrderType, // Bid or Ask
    value: f32, // Size of the order (force)
    active: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum OrderType {
    Bid,
    Ask,
}

struct Machine {
    // Price Discovery Mechanism
    diff: Differential, // Sums Bid Torque + Ask Torque
    price: f32, // Current Price (Accumulated diff output)

    // Trend Analysis Mechanism
    sma_integrator: Integrator, // Integrates Price over time

    // Order Flow
    orders: Vec<Order>,

    // Physics State
    bid_torque: f32,
    ask_torque: f32,

    // History for plotting
    history: Vec<(f32, f32)>, // (time, price)
    t: f32,
}

impl Machine {
    fn new() -> Self {
        let mut m = Self {
            diff: Differential::new(),
            price: 100.0, // Starting Price
            sma_integrator: Integrator::new(),
            orders: Vec::new(),
            bid_torque: 0.0,
            ask_torque: 0.0,
            history: Vec::new(),
            t: 0.0,
        };
        // Setup initial integrator state
        m.sma_integrator.carriage_pos = m.price;
        m
    }

    fn update(&mut self, dt: f32) {
        let mut rng = ::rand::thread_rng();

        // 1. Spawn Orders
        if rng.gen_bool(0.05) {
            self.spawn_order(OrderType::Bid);
        }
        if rng.gen_bool(0.05) {
            self.spawn_order(OrderType::Ask);
        }

        // 2. Update Orders (Physics)
        self.bid_torque = 0.0;
        self.ask_torque = 0.0;

        let paddle_y = 300.0;
        let cx = screen_width() / 2.0;

        for order in &mut self.orders {
            if !order.active { continue; }

            order.pos += order.vel;
            order.vel.y += 0.2; // Gravity

            // Collision with Paddles
            if order.pos.y > paddle_y && order.pos.y < paddle_y + 20.0 {
                // Bid Paddle (Left)
                if order.order_type == OrderType::Bid && (order.pos.x - (cx - 100.0)).abs() < 30.0 {
                    self.bid_torque += order.value; // Positive Torque pushes Price UP
                    order.active = false;
                }
                // Ask Paddle (Right)
                else if order.order_type == OrderType::Ask && (order.pos.x - (cx + 100.0)).abs() < 30.0 {
                    self.ask_torque -= order.value; // Negative Torque pushes Price DOWN
                    order.active = false;
                }
            }

            if order.pos.y > screen_height() {
                order.active = false;
            }
        }

        self.orders.retain(|o| o.active);

        // 3. Update Mechanisms

        // Differential: Input A = Bid Torque, Input B = Ask Torque
        let sensitivity = 1.0;

        // If no orders, maybe some friction brings it to valid state?
        // Or just holds position.
        // Let's add some "market noise" or random walk if torque is 0?
        // No, let's keep it pure physics.

        let delta_price = self.diff.update(self.bid_torque * sensitivity, self.ask_torque * sensitivity);
        self.price += delta_price;

        // Clamp Price
        if self.price < 0.0 { self.price = 0.0; }

        // Integrator: Input = Time (constant rotation), Carriage = Price
        // Carriage position is relative to some center.
        // Let's say center is 100.0.
        // If price > 100, integrator accumulates positive.
        // If price < 100, integrator accumulates negative.
        // This makes the integrator an "Accumulated Deviation" meter.
        self.sma_integrator.carriage_pos = (self.price - 100.0) / 10.0;
        let _ = self.sma_integrator.update(dt);

        // 4. Update History
        self.t += dt;
        if self.history.len() > 1000 {
            self.history.remove(0);
        }
        self.history.push((self.t, self.price));
    }

    fn spawn_order(&mut self, o_type: OrderType) {
        let cx = screen_width() / 2.0;
        let x = match o_type {
            OrderType::Bid => cx - 100.0 + ::rand::thread_rng().gen_range(-10.0..10.0),
            OrderType::Ask => cx + 100.0 + ::rand::thread_rng().gen_range(-10.0..10.0),
        };

        self.orders.push(Order {
            pos: vec2(x, 0.0), // Start from top
            vel: vec2(0.0, 0.0),
            order_type: o_type,
            value: ::rand::thread_rng().gen_range(0.5..2.5),
            active: true,
        });
    }

    fn draw(&self) {
        let cx = screen_width() / 2.0;
        let cy = 300.0; // Paddle Height

        draw_text("MECHANICAL MARKET", 20.0, 30.0, 30.0, WHITE);
        draw_text("Analog High Frequency Trading", 20.0, 50.0, 20.0, LIGHTGRAY);
        draw_text("Left: BID (Buy) | Right: ASK (Sell)", 20.0, 70.0, 20.0, GRAY);

        // Paddles / Buckets
        draw_rectangle(cx - 120.0, cy, 40.0, 10.0, GREEN); // Bid Bucket
        draw_rectangle(cx + 80.0, cy, 40.0, 10.0, RED);   // Ask Bucket

        // Funnels
        draw_line(cx - 130.0, 0.0, cx - 120.0, cy, 2.0, DARKGRAY);
        draw_line(cx - 70.0, 0.0, cx - 80.0, cy, 2.0, DARKGRAY);

        draw_line(cx + 70.0, 0.0, cx + 80.0, cy, 2.0, DARKGRAY);
        draw_line(cx + 130.0, 0.0, cx + 120.0, cy, 2.0, DARKGRAY);

        // Shafts to Differential
        // Left Shaft (Bids)
        draw_line(cx - 100.0, cy + 10.0, cx - 100.0, cy + 80.0, 4.0, SHAFT_COLOR);
        draw_line(cx - 100.0, cy + 80.0, cx - 30.0, cy + 80.0, 4.0, SHAFT_COLOR);

        // Right Shaft (Asks)
        draw_line(cx + 100.0, cy + 10.0, cx + 100.0, cy + 80.0, 4.0, SHAFT_COLOR);
        draw_line(cx + 100.0, cy + 80.0, cx + 30.0, cy + 80.0, 4.0, SHAFT_COLOR);

        // Differential Box
        let diff_pos = vec2(cx, cy + 100.0);
        draw_rectangle(cx - 30.0, cy + 80.0, 60.0, 40.0, GEAR_COLOR);
        draw_rectangle_lines(cx - 30.0, cy + 80.0, 60.0, 40.0, 2.0, WHITE);
        draw_text("DIFF", cx - 20.0, cy + 105.0, 20.0, BLACK);

        // Output Shaft (Price) driving Integrator
        // Price determines height of Integrator Carriage?
        // Let's visualize Price as a vertical gauge
        let gauge_x = cx;
        let gauge_top = cy + 150.0;
        let gauge_h = 200.0;

        draw_line(gauge_x, gauge_top, gauge_x, gauge_top + gauge_h, 4.0, SHAFT_COLOR);

        // Price Marker
        // Map Price 0..200 to gauge height
        let price_norm = self.price / 200.0; // 0.5 is 100
        let marker_y = gauge_top + gauge_h - (price_norm * gauge_h);

        draw_circle(gauge_x, marker_y, 8.0, YELLOW);
        draw_text(&format!("{:.2}", self.price), gauge_x + 15.0, marker_y + 5.0, 20.0, YELLOW);

        // Integrator
        let int_pos = vec2(cx + 200.0, cy + 200.0);
        draw_line(gauge_x, marker_y, int_pos.x - 40.0, marker_y, 1.0, GRAY); // Connection
        self.draw_integrator(int_pos);

        // Orders
        for order in &self.orders {
            let color = match order.order_type {
                OrderType::Bid => BID_COLOR,
                OrderType::Ask => ASK_COLOR,
            };
            draw_circle(order.pos.x, order.pos.y, 4.0, color);
        }

        // Plot
        self.draw_plot(vec2(cx - 250.0, gauge_top + 220.0), vec2(500.0, 100.0));
    }

    fn draw_integrator(&self, pos: Vec2) {
        let r = 40.0;
        draw_circle(pos.x, pos.y, r, DARKGRAY);
        draw_circle_lines(pos.x, pos.y, r, 2.0, LIGHTGRAY);

        // Carriage Position
        let scale = 4.0;
        let c_offset = (self.sma_integrator.carriage_pos * scale).clamp(-r, r);
        draw_circle(pos.x + c_offset, pos.y, 5.0, RED);

        // Rotating indicator
        let angle = self.sma_integrator.output_angle;
        let dx = angle.cos() * r;
        let dy = angle.sin() * r;
        draw_line(pos.x, pos.y, pos.x + dx, pos.y + dy, 2.0, WHITE);

        draw_text("Deviation", pos.x - 30.0, pos.y + r + 20.0, 15.0, WHITE);
        draw_text("Integrator", pos.x - 30.0, pos.y + r + 35.0, 15.0, WHITE);
    }

    fn draw_plot(&self, pos: Vec2, size: Vec2) {
        draw_rectangle(pos.x, pos.y, size.x, size.y, Color::new(0.0, 0.0, 0.0, 0.5));
        draw_rectangle_lines(pos.x, pos.y, size.x, size.y, 1.0, GRAY);

        if self.history.len() < 2 { return; }

        let t_start = self.history.first().unwrap().0;
        let t_end = self.history.last().unwrap().0;
        let t_span = t_end - t_start;

        if t_span < 0.001 { return; }

        let min_price = 0.0;
        let max_price = 200.0;
        let price_span = max_price - min_price;

        let map_x = |t: f32| pos.x + ((t - t_start) / t_span) * size.x;
        let map_y = |p: f32| pos.y + size.y - ((p - min_price) / price_span) * size.y;

        let mut prev = self.history.first().unwrap();

        for curr in self.history.iter().skip(1) {
            let x1 = map_x(prev.0);
            let y1 = map_y(prev.1);
            let x2 = map_x(curr.0);
            let y2 = map_y(curr.1);

            draw_line(x1, y1, x2, y2, 2.0, BLUE);
            prev = curr;
        }
    }
}

#[macroquad::main("Mechanical Market")]
async fn main() {
    let mut machine = Machine::new();

    loop {
        clear_background(Color::new(0.1, 0.1, 0.15, 1.0));

        let dt = get_frame_time(); // Realtime
        machine.update(dt);
        machine.draw();

        next_frame().await
    }
}
