use macroquad::prelude::*;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::thread;

mod garden;
mod seeds;
mod tracer;

use garden::Garden;
use tracer::{Event, Tracer};

#[macroquad::main("Recursion Garden")]
async fn main() {
    let mut garden = Garden::new();
    let (mut tx_token, mut rx_event) = start_sim("fib");

    let mut playing = true;
    let mut speed = 1; // tokens per frame
    let mut sim_type = "fib";

    loop {
        if is_key_pressed(KeyCode::Space) {
            playing = !playing;
        }
        if is_key_pressed(KeyCode::R) {
            garden = Garden::new();
            let (tx, rx) = start_sim(sim_type);
            tx_token = tx;
            rx_event = rx;
            playing = true;
        }
        if is_key_pressed(KeyCode::F) {
            sim_type = "fib";
            garden = Garden::new();
            let (tx, rx) = start_sim(sim_type);
            tx_token = tx;
            rx_event = rx;
            playing = true;
        }
        if is_key_pressed(KeyCode::M) {
            sim_type = "merge";
            garden = Garden::new();
            let (tx, rx) = start_sim(sim_type);
            tx_token = tx;
            rx_event = rx;
            playing = true;
        }
        if is_key_pressed(KeyCode::C) {
            sim_type = "collatz";
            garden = Garden::new();
            let (tx, rx) = start_sim(sim_type);
            tx_token = tx;
            rx_event = rx;
            playing = true;
        }

        // Speed control
        if is_key_pressed(KeyCode::Right) {
            speed += 1;
        }
        if is_key_pressed(KeyCode::Left) && speed > 1 {
            speed -= 1;
        }

        // Logic
        if playing {
            for _ in 0..speed {
                // Check if we have pending events first
                match rx_event.try_recv() {
                    Ok(event) => {
                        garden.process_event(event);
                        // If we processed an event, we can send a new token to request the next step
                        let _ = tx_token.send(());
                    }
                    Err(TryRecvError::Empty) => {
                        // No event ready.
                        // If we haven't sent enough tokens, we should send one?
                        // Wait, strictly 1 token = 1 event?
                        // Yes, Tracer::enter consumes 1 token, produces 1 event.
                        // Tracer::exit consumes 1 token, produces 1 event.
                        // So 1-to-1 mapping.

                        // If the channel is empty, it means the thread is working or waiting for a token.
                        // To bootstrap, we need to send tokens.
                        let _ = tx_token.send(());
                        break;
                    }
                    Err(TryRecvError::Disconnected) => {
                        playing = false;
                        break;
                    }
                }
            }
        }

        // Draw
        clear_background(BLACK);
        garden.draw();

        // UI
        draw_text(
            &format!("Sim: {} (F/M/C)", sim_type),
            20.0,
            30.0,
            30.0,
            WHITE,
        );
        draw_text(
            &format!("Speed: {} (Left/Right)", speed),
            20.0,
            60.0,
            30.0,
            WHITE,
        );
        draw_text("Space: Pause/Play, R: Reset", 20.0, 90.0, 30.0, WHITE);

        if !playing {
            draw_text("PAUSED / DONE", screen_width() - 200.0, 30.0, 30.0, RED);
        }

        next_frame().await;
    }
}

fn start_sim(sim_type: &str) -> (Sender<()>, Receiver<Event>) {
    let (tx_token, rx_token) = channel();
    let (tx_event, rx_event) = channel();

    let sim_type = sim_type.to_string();

    thread::spawn(move || {
        let mut tracer = Tracer::new(tx_event, rx_token);
        match sim_type.as_str() {
            "fib" => {
                seeds::fibonacci(10, &mut tracer);
            } // Reduced depth for visibility
            "merge" => {
                use ::rand::Rng;
                let mut rng = ::rand::thread_rng();
                let arr: Vec<i32> = (0..16).map(|_| rng.gen_range(0..100)).collect();
                seeds::merge_sort(arr, &mut tracer);
            }
            "collatz" => {
                seeds::collatz(27, &mut tracer);
            }
            _ => {}
        }
    });

    (tx_token, rx_event)
}
