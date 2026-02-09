mod audio;
mod neuron;

use audio::init_audio;
use macroquad::prelude::*;
use neuron::{Command, HodgkinHuxley};
use std::collections::VecDeque;

const HISTORY_LEN: usize = 1000;

#[macroquad::main("Biophysical Synth")]
async fn main() -> anyhow::Result<()> {
    // Start Audio System
    let mut audio_sys = init_audio()?;

    // Visualization state
    let mut wave_history: VecDeque<f32> = VecDeque::with_capacity(HISTORY_LEN);
    for _ in 0..HISTORY_LEN {
        wave_history.push_back(-65.0);
    }

    let mut last_state = HodgkinHuxley::new();

    // UI State
    let mut current_inj: f32 = 0.0;

    loop {
        // 1. Read Inputs

        // Piano Keys (Current Injection)
        let target_current = if is_key_down(KeyCode::A) {
            2.0
        } else if is_key_down(KeyCode::S) {
            5.0
        } else if is_key_down(KeyCode::D) {
            10.0
        } else if is_key_down(KeyCode::F) {
            20.0
        } else if is_key_down(KeyCode::G) {
            35.0
        }
        // Depolarization block territory?
        else if is_key_down(KeyCode::H) {
            50.0
        } else if is_key_down(KeyCode::J) {
            100.0
        } else {
            0.0
        };

        if (target_current - current_inj).abs() > 0.1 {
            current_inj = target_current;
            audio_sys
                .command_tx
                .push(Command::SetCurrent(current_inj))
                .ok();
        }

        if is_key_pressed(KeyCode::Space) {
            audio_sys.command_tx.push(Command::Pluck).ok();
        }

        // Parameter Tuning
        if is_key_down(KeyCode::Up) {
            last_state.g_na += 1.0;
            audio_sys
                .command_tx
                .push(Command::SetGNa(last_state.g_na))
                .ok();
        }
        if is_key_down(KeyCode::Down) {
            last_state.g_na = (last_state.g_na - 1.0).max(0.0);
            audio_sys
                .command_tx
                .push(Command::SetGNa(last_state.g_na))
                .ok();
        }
        if is_key_down(KeyCode::Right) {
            last_state.g_k += 1.0;
            audio_sys
                .command_tx
                .push(Command::SetGK(last_state.g_k))
                .ok();
        }
        if is_key_down(KeyCode::Left) {
            last_state.g_k = (last_state.g_k - 1.0).max(0.0);
            audio_sys
                .command_tx
                .push(Command::SetGK(last_state.g_k))
                .ok();
        }

        if is_key_pressed(KeyCode::R) {
            let def = HodgkinHuxley::new();
            audio_sys.command_tx.push(Command::SetGNa(def.g_na)).ok();
            audio_sys.command_tx.push(Command::SetGK(def.g_k)).ok();
            last_state.g_na = def.g_na;
            last_state.g_k = def.g_k;
        }

        // 2. Consume Data
        // Waveform
        while let Some(v) = audio_sys.waveform_rx.pop() {
            wave_history.push_back(v);
            if wave_history.len() > HISTORY_LEN {
                wave_history.pop_front();
            }
        }

        // State
        while let Some(state) = audio_sys.state_rx.pop() {
            // Update local tracking of parameters if they changed externally (unlikely, but good sync)
            // Actually we mainly want m, h, n.
            // We shouldn't overwrite our local g_na/g_k control variables if we are currently editing them,
            // but for visualization we want the *actual* values.
            // Let's just update `last_state` completely.
            // But if we do that, our `is_key_down` logic above works on `last_state`, so it should be fine.
            last_state = state;
        }

        // 3. Render
        clear_background(BLACK);

        // Draw Oscilloscope
        draw_text("MEMBRANE POTENTIAL (mV)", 10.0, 30.0, 20.0, GREEN);

        let center_y = screen_height() / 3.0;
        let scale_y = 2.0; // mV to pixels
        let step_x = screen_width() / HISTORY_LEN as f32;

        for i in 0..wave_history.len() - 1 {
            let v1 = wave_history[i];
            let v2 = wave_history[i + 1];

            // Map -100..50 to screen
            let y1 = center_y - (v1 + 60.0) * scale_y;
            let y2 = center_y - (v2 + 60.0) * scale_y;

            let x1 = i as f32 * step_x;
            let x2 = (i + 1) as f32 * step_x;

            draw_line(x1, y1, x2, y2, 2.0, GREEN);
        }

        // Draw Parameters (Bars)
        let bar_base = screen_height() * 0.7;
        let spacing = 60.0;
        let start_x = 50.0;

        // Gating Variables (0.0 - 1.0)
        draw_bar(start_x, bar_base, last_state.m, "m (Na)", RED);
        draw_bar(start_x + spacing, bar_base, last_state.h, "h (Na)", ORANGE);
        draw_bar(
            start_x + spacing * 2.0,
            bar_base,
            last_state.n,
            "n (K)",
            BLUE,
        );

        // Conductances (Normalized roughly for display)
        draw_bar(
            start_x + spacing * 4.0,
            bar_base,
            last_state.g_na / 200.0,
            "G_Na",
            RED,
        );
        draw_bar(
            start_x + spacing * 5.0,
            bar_base,
            last_state.g_k / 100.0,
            "G_K",
            BLUE,
        );

        // Current
        draw_bar(
            start_x + spacing * 6.0,
            bar_base,
            current_inj / 100.0,
            "I_inj",
            WHITE,
        );

        // Instructions
        draw_text("CONTROLS:", 10.0, screen_height() - 60.0, 20.0, LIGHTGRAY);
        draw_text(
            "Hold A/S/D/F/G/H/J to inject Current (Notes)",
            10.0,
            screen_height() - 40.0,
            20.0,
            LIGHTGRAY,
        );
        draw_text(
            "Arrows: Tune G_Na / G_K (Timbre) | Space: Pluck | R: Reset",
            10.0,
            screen_height() - 20.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await
    }
}

fn draw_bar(x: f32, y: f32, value: f32, label: &str, color: Color) {
    let max_h = 200.0;
    let h = (value.clamp(0.0, 1.0) * max_h).max(1.0);

    draw_rectangle(x, y - h, 40.0, h, color);
    draw_rectangle_lines(x, y - max_h, 40.0, max_h, 1.0, DARKGRAY);
    draw_text(label, x, y + 20.0, 15.0, color);
    draw_text(&format!("{:.2}", value), x, y - h - 5.0, 15.0, WHITE);
}
