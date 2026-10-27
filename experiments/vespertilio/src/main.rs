//! # Vespertilio 🦇
//!
//! **Parent:** `resonance-audio` (`PhysicsGrid`, a 2D FDTD wave solver)
//!
//! ## Concept
//! A bat hunts moths in a dark cave by *actual* echolocation. The bat's chirp
//! is a `pluck()` injected into the same wave field the moths live in. The
//! wave propagates through the FDTD grid at ~0.63 cells/step, reflects off
//! the cave rock (`Material::Wall`), and washes over the moths. A moth hit by
//! a strong wavefront does two honest things: it panics, and it rings — a
//! faint secondary `pluck()`, the echo. The bat hears whatever wave energy
//! physically returns to its own cell and steers up the local amplitude
//! gradient. No moth ever whispers its coordinates to the bat; the bat knows
//! only what the wave field tells it, clutter and all.
//!
//! ## Phenotype
//! Rings of sound expanding through stone, moths flashing awake as the
//! wavefront touches them, and a small `V` turning — sometimes wrongly —
//! toward the loudest return. Cave clutter lies. Moths exploit it. Such is
//! the midnight carnival.
//!
//! ## Headless
//! `cargo run -p vespertilio -- --headless [ticks]` runs the simulation with
//! no terminal and prints the hunt ledger (chirps, echoes triggered, echoes
//! heard, moths caught).

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use resonance_audio::{Material, PhysicsGrid};
use std::time::{Duration, Instant};
use tui_shared::{
    crossterm::event::{self, Event, KeyCode},
    ratatui::{
        style::{Color, Modifier, Style},
        text::{Line, Span},
        widgets::{Block, Borders, Paragraph},
    },
    Tui,
};

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
const CHIRP_INTERVAL: u64 = 18;
const CHIRP_STRENGTH: f32 = 2.5;
const ECHO_STRENGTH: f32 = 1.5;
/// Wave amplitude at a moth that counts as "the chirp found it".
const MOTH_HIT_THRESHOLD: f32 = 0.30;
/// Wave amplitude at the bat that counts as a heard return.
const HEAR_THRESHOLD: f32 = 0.02;
/// Ticks after chirping during which the bat is deaf to its own voice.
const DEAF_TICKS: u64 = 3;
const BAT_SPEED: f64 = 0.55;
const CATCH_RADIUS: f64 = 2.0;

#[derive(Clone, Copy, Debug)]
struct Moth {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    panic: u32,
    echo_cooldown: u32,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Ledger {
    pub chirps: u64,
    pub echoes_triggered: u64,
    pub echoes_heard: u64,
    pub moths_caught: u64,
}

pub struct Hunt {
    grid: PhysicsGrid,
    walls: Vec<bool>,
    bat_x: f64,
    bat_y: f64,
    heading: f64,
    /// Direction of the strongest recently heard return; memory decays.
    heard_dir: Option<(f64, f64)>,
    heard_memory: u32,
    moths: Vec<Moth>,
    tick: u64,
    last_chirp: u64,
    pub ledger: Ledger,
    rng: StdRng,
}

impl Hunt {
    pub fn new(seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let mut grid = PhysicsGrid::new(WIDTH, HEIGHT);
        let mut walls = vec![false; WIDTH * HEIGHT];
        let set_wall = |grid: &mut PhysicsGrid, walls: &mut Vec<bool>, x: usize, y: usize| {
            grid.set_material(x, y, Material::Wall);
            walls[y * WIDTH + x] = true;
        };

        // Cave rim.
        for x in 0..WIDTH {
            set_wall(&mut grid, &mut walls, x, 0);
            set_wall(&mut grid, &mut walls, x, HEIGHT - 1);
        }
        for y in 0..HEIGHT {
            set_wall(&mut grid, &mut walls, 0, y);
            set_wall(&mut grid, &mut walls, WIDTH - 1, y);
        }
        // Rock blobs (stalactite shadows) — clutter that lies to echolocation.
        for _ in 0..7 {
            let cx = rng.gen_range(6..WIDTH - 6) as f64;
            let cy = rng.gen_range(5..HEIGHT - 5) as f64;
            let r = rng.gen_range(1.5..3.2);
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let d = ((x as f64 - cx).powi(2) + (y as f64 - cy).powi(2)).sqrt();
                    if d < r {
                        set_wall(&mut grid, &mut walls, x, y);
                    }
                }
            }
        }

        // Bat starts mid-cave in open air; clear a pocket around the spawn.
        let (bat_x, bat_y) = (WIDTH as f64 / 2.0, HEIGHT as f64 / 2.0);
        clear_pocket(&mut grid, &mut walls, bat_x as usize, bat_y as usize, 3);

        let mut moths = Vec::new();
        for _ in 0..6 {
            moths.push(Self::spawn_moth(&mut rng, &walls, bat_x, bat_y));
        }

        Self {
            grid,
            walls,
            bat_x,
            bat_y,
            heading: 0.0,
            heard_dir: None,
            heard_memory: 0,
            moths,
            tick: 0,
            last_chirp: 0,
            ledger: Ledger::default(),
            rng,
        }
    }

    fn is_wall(&self, x: usize, y: usize) -> bool {
        if x >= WIDTH || y >= HEIGHT {
            return true;
        }
        self.walls[y * WIDTH + x]
    }

    fn spawn_moth(rng: &mut StdRng, walls: &[bool], bat_x: f64, bat_y: f64) -> Moth {
        loop {
            let x = rng.gen_range(2..WIDTH - 2) as f64 + 0.5;
            let y = rng.gen_range(2..HEIGHT - 2) as f64 + 0.5;
            if walls[y as usize * WIDTH + x as usize] {
                continue;
            }
            if (x - bat_x).hypot(y - bat_y) < 10.0 {
                continue; // no free lunches; moths start beyond easy reach
            }
            let a = rng.gen_range(0.0..std::f64::consts::TAU);
            return Moth {
                x,
                y,
                vx: a.cos() * 0.15,
                vy: a.sin() * 0.15,
                panic: 0,
                echo_cooldown: 0,
            };
        }
    }

    pub fn chirp_now(&mut self) {
        let bx = self.bat_x as usize;
        let by = self.bat_y as usize;
        self.grid.pluck(bx, by, CHIRP_STRENGTH);
        self.ledger.chirps += 1;
        self.last_chirp = self.tick;
    }

    pub fn step(&mut self) {
        self.tick += 1;

        if self.tick.saturating_sub(self.last_chirp) >= CHIRP_INTERVAL {
            self.chirp_now();
        }

        // Two FDTD substeps per tick: waves visibly outrun the bat.
        self.grid.step();
        self.grid.step();

        // --- Moths: flutter, get found, ring, flee. ---
        let bat_pos = (self.bat_x, self.bat_y);
        let mut echo_plucks: Vec<(usize, usize)> = Vec::new();
        for m in &mut self.moths {
            let amp = self.grid.get(m.x as usize, m.y as usize).abs();
            if m.echo_cooldown > 0 {
                m.echo_cooldown -= 1;
            }
            if amp > MOTH_HIT_THRESHOLD && m.echo_cooldown == 0 {
                echo_plucks.push((m.x as usize, m.y as usize));
                m.echo_cooldown = 25;
                m.panic = 14;
                self.ledger.echoes_triggered += 1;
            }

            let speed = if m.panic > 0 { 0.42 } else { 0.16 };
            if m.panic > 0 {
                m.panic -= 1;
                // Flee directly away from the bat.
                let dx = m.x - bat_pos.0;
                let dy = m.y - bat_pos.1;
                let d = dx.hypot(dy).max(0.001);
                m.vx = dx / d * speed;
                m.vy = dy / d * speed;
            } else if self.rng.gen_bool(0.06) {
                let a = self.rng.gen_range(0.0..std::f64::consts::TAU);
                m.vx = a.cos() * speed;
                m.vy = a.sin() * speed;
            }
            let nx = m.x + m.vx;
            let ny = m.y + m.vy;
            if wall_at(&self.walls, nx as usize, m.y as usize) {
                m.vx = -m.vx;
            } else {
                m.x = nx;
            }
            if wall_at(&self.walls, m.x as usize, ny as usize) {
                m.vy = -m.vy;
            } else {
                m.y = ny;
            }
        }
        for (x, y) in echo_plucks {
            self.grid.pluck(x, y, ECHO_STRENGTH);
        }

        // --- Bat: listen, then steer by what it heard. ---
        if self.tick.saturating_sub(self.last_chirp) > DEAF_TICKS {
            let bx = self.bat_x as usize;
            let by = self.bat_y as usize;
            let amp_here = self.grid.get(bx, by).abs();
            if amp_here > HEAR_THRESHOLD {
                let gx = self.grid.get(bx + 1, by) - self.grid.get(bx.saturating_sub(1), by);
                let gy = self.grid.get(bx, by + 1) - self.grid.get(bx, by.saturating_sub(1));
                if gx.abs() + gy.abs() > 0.002 {
                    self.heard_dir = Some((gx as f64, gy as f64));
                    self.heard_memory = 60;
                    self.ledger.echoes_heard += 1;
                }
            }
        }

        let mut desired = self.heading;
        if self.heard_memory > 0 {
            self.heard_memory -= 1;
            if let Some((gx, gy)) = self.heard_dir {
                desired = gy.atan2(gx);
            }
        } else {
            // Wander: a slow sinusoidal sweep, the bat quartering the cave.
            desired = self.heading + 0.08 * (self.tick as f64 * 0.05).sin();
        }

        // Wall avoidance: probe ahead; rock in the way bends the turn hard.
        let probe_x = self.bat_x + desired.cos() * 3.0;
        let probe_y = self.bat_y + desired.sin() * 3.0;
        if self.is_wall(probe_x as usize, probe_y as usize) {
            desired += 1.1;
        }

        // Turn toward desired with a limited rate.
        let mut diff = desired - self.heading;
        while diff > std::f64::consts::PI {
            diff -= std::f64::consts::TAU;
        }
        while diff < -std::f64::consts::PI {
            diff += std::f64::consts::TAU;
        }
        self.heading += diff.clamp(-0.35, 0.35);

        let nx = self.bat_x + self.heading.cos() * BAT_SPEED;
        let ny = self.bat_y + self.heading.sin() * BAT_SPEED;
        if !self.is_wall(nx as usize, self.bat_y as usize) {
            self.bat_x = nx;
        } else {
            self.heading += 0.9;
        }
        if !self.is_wall(self.bat_x as usize, ny as usize) {
            self.bat_y = ny;
        } else {
            self.heading += 0.9;
        }

        // --- The catch. ---
        let mut caught = Vec::new();
        for (i, m) in self.moths.iter().enumerate() {
            if (m.x - self.bat_x).hypot(m.y - self.bat_y) < CATCH_RADIUS {
                caught.push(i);
            }
        }
        for i in caught {
            self.ledger.moths_caught += 1;
            self.moths[i] = Self::spawn_moth(&mut self.rng, &self.walls, self.bat_x, self.bat_y);
        }
    }

    fn render_lines(&self) -> Vec<Line<'static>> {
        let shades = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
        let mut lines = Vec::with_capacity(HEIGHT + 1);
        for y in 0..HEIGHT {
            let mut spans = Vec::with_capacity(WIDTH);
            for x in 0..WIDTH {
                if self.is_wall(x, y) {
                    spans.push(Span::styled(
                        "█",
                        Style::default().fg(Color::DarkGray),
                    ));
                    continue;
                }
                if x == self.bat_x as usize && y == self.bat_y as usize {
                    spans.push(Span::styled(
                        "V",
                        Style::default()
                            .fg(Color::Magenta)
                            .add_modifier(Modifier::BOLD),
                    ));
                    continue;
                }
                if self
                    .moths
                    .iter()
                    .any(|m| m.x as usize == x && m.y as usize == y)
                {
                    spans.push(Span::styled(
                        "*",
                        Style::default().fg(Color::Yellow),
                    ));
                    continue;
                }
                let amp = self.grid.get(x, y).abs();
                let idx = ((amp * 6.0) as usize).min(shades.len() - 1);
                let ch = shades[idx];
                spans.push(Span::styled(
                    ch.to_string(),
                    Style::default().fg(Color::Cyan),
                ));
            }
            lines.push(Line::from(spans));
        }
        lines
    }
}

fn wall_at(walls: &[bool], x: usize, y: usize) -> bool {
    if x >= WIDTH || y >= HEIGHT {
        return true;
    }
    walls[y * WIDTH + x]
}

fn clear_pocket(grid: &mut PhysicsGrid, walls: &mut Vec<bool>, cx: usize, cy: usize, r: usize) {
    for y in cy.saturating_sub(r)..=(cy + r).min(HEIGHT - 1) {
        for x in cx.saturating_sub(r)..=(cx + r).min(WIDTH - 1) {
            if x == 0 || y == 0 || x == WIDTH - 1 || y == HEIGHT - 1 {
                continue; // rim stays rock
            }
            grid.set_material(x, y, Material::Air);
            walls[y * WIDTH + x] = false;
        }
    }
}

fn parse_headless(args: &[String]) -> Option<u64> {
    if !args.iter().any(|a| a == "--headless") {
        return None;
    }
    let pos = args.iter().position(|a| a == "--headless")?;
    let ticks = args
        .get(pos + 1)
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(1200);
    Some(ticks)
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    // Headless FIRST, before any terminal is touched — the Reaper reads this file.
    if let Some(ticks) = parse_headless(&args) {
        let mut hunt = Hunt::new(20261027);
        for _ in 0..ticks {
            hunt.step();
        }
        println!("vespertilio headless hunt: {ticks} ticks");
        println!("  chirps emitted:   {}", hunt.ledger.chirps);
        println!("  echoes triggered: {}", hunt.ledger.echoes_triggered);
        println!("  echoes heard:     {}", hunt.ledger.echoes_heard);
        println!("  moths caught:     {}", hunt.ledger.moths_caught);
        return Ok(());
    }

    let mut tui = Tui::init()?;
    let mut hunt = Hunt::new(20261027);
    let mut paused = false;
    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| {
            let area = f.area();
            let mut lines = vec![Line::from(Span::styled(
                format!(
                    " 🦇 vespertilio — tick {} · chirps {} · echoes heard {} · caught {} ",
                    hunt.tick, hunt.ledger.chirps, hunt.ledger.echoes_heard, hunt.ledger.moths_caught
                ),
                Style::default().fg(Color::Magenta),
            ))];
            lines.extend(hunt.render_lines());
            lines.push(Line::from(
                " [space] pause · [c] chirp now · [r] new cave · [q] quit ",
            ));
            let p = Paragraph::new(lines).block(
                Block::default()
                    .title("The Midnight Cave")
                    .borders(Borders::ALL),
            );
            f.render_widget(p, area);
        })?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char(' ') => paused = !paused,
                    KeyCode::Char('c') => hunt.chirp_now(),
                    KeyCode::Char('r') => {
                        hunt = Hunt::new((hunt.tick << 7) ^ 0xE57);
                    }
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            if !paused {
                hunt.step();
            }
            last_tick = Instant::now();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wave_leaves_the_pluck_site() {
        let mut g = PhysicsGrid::new(16, 16);
        g.pluck(8, 8, 1.0);
        for _ in 0..10 {
            g.step();
        }
        // Energy must have spread to a neighbour — the FDTD grid is alive.
        assert!(g.get(10, 8).abs() > 0.001 || g.get(8, 10).abs() > 0.001);
    }

    #[test]
    fn a_hunt_actually_hunts() {
        let mut hunt = Hunt::new(20261027);
        for _ in 0..1500 {
            hunt.step();
        }
        assert!(hunt.ledger.chirps > 50, "bat should chirp: {:?}", hunt.ledger);
        assert!(
            hunt.ledger.echoes_triggered > 0,
            "some moth should have been found by a wavefront: {:?}",
            hunt.ledger
        );
        assert!(
            hunt.ledger.echoes_heard > 0,
            "some return should have reached the bat: {:?}",
            hunt.ledger
        );
    }
}
