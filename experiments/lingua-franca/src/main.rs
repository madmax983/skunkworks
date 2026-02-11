use crate::agent::{Agent, Concept};
use crate::phonology::{PhonemeType, Phonotactics};
use macroquad::prelude::*;
use std::collections::HashMap;

mod agent;
mod phonology;

struct InteractionEvent {
    p1: Vec2,
    p2: Vec2,
    word: String,
    success: bool,
    timer: f32,
}

fn analyze_lexicon(agents: &[Agent]) -> Vec<(Concept, String, usize)> {
    let mut counts: HashMap<Concept, HashMap<String, usize>> = HashMap::new();

    for agent in agents {
        for (concept, word) in &agent.lexicon.map {
            let word_str = word.to_string();
            let entry = counts.entry(*concept).or_default();
            *entry.entry(word_str).or_insert(0) += 1;
        }
    }

    let mut results = Vec::new();
    for concept in Concept::all() {
        if let Some(word_counts) = counts.get(&concept) {
            if let Some((best_word, count)) = word_counts.iter().max_by_key(|(_, c)| *c) {
                results.push((concept, best_word.clone(), *count));
            }
        }
    }
    results
}

#[macroquad::main("Lingua Franca")]
async fn main() {
    // Setup Nords
    let nord_phono = Phonotactics::new(
        vec!['a', 'i', 'u'],
        vec!['p', 't', 'k', 'r', 's', 'l'],
        vec![PhonemeType::C, PhonemeType::V, PhonemeType::C],
    );

    // Setup Sudrons
    let sudron_phono = Phonotactics::new(
        vec!['e', 'o', 'a'],
        vec!['m', 'n', 'w', 'y', 'h', 'z'],
        vec![PhonemeType::V, PhonemeType::C, PhonemeType::V],
    );

    let mut agents = Vec::new();

    // Create Nords
    for _ in 0..15 {
        let mut a = Agent::new(
            nord_phono.clone(),
            (rand::gen_range(50., 300.), rand::gen_range(50., 300.)),
            (100, 100, 255),
        );
        a.generate_native_lexicon();
        agents.push(a);
    }

    // Create Sudrons
    for _ in 0..15 {
        let mut a = Agent::new(
            sudron_phono.clone(),
            (rand::gen_range(500., 750.), rand::gen_range(300., 550.)),
            (255, 100, 100),
        );
        a.generate_native_lexicon();
        agents.push(a);
    }

    let mut interactions: Vec<InteractionEvent> = Vec::new();
    let mut frame_count = 0;
    let mut dominant_words: Vec<(Concept, String, usize)> = Vec::new();

    loop {
        clear_background(BLACK);

        frame_count += 1;
        if frame_count % 30 == 0 {
            dominant_words = analyze_lexicon(&agents);
        }

        let dt = get_frame_time();
        let screen_center = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);

        // Update Agents
        for agent in &mut agents {
            // Brownian
            agent.position.0 += rand::gen_range(-1.0, 1.0);
            agent.position.1 += rand::gen_range(-1.0, 1.0);

            // Attract to center (Marketplace)
            let pos = Vec2::new(agent.position.0, agent.position.1);
            let dir = (screen_center - pos).normalize_or_zero() * 0.5;
            agent.position.0 += dir.x;
            agent.position.1 += dir.y;

            // Bounds
            agent.position.0 = agent.position.0.clamp(10., screen_width() - 10.);
            agent.position.1 = agent.position.1.clamp(10., screen_height() - 10.);
        }

        // Interaction
        if rand::gen_range(0, 5) == 0 {
            let idx1 = rand::gen_range(0, agents.len());
            let idx2 = rand::gen_range(0, agents.len());

            if idx1 != idx2 {
                let pos1 = Vec2::new(agents[idx1].position.0, agents[idx1].position.1);
                let pos2 = Vec2::new(agents[idx2].position.0, agents[idx2].position.1);

                if pos1.distance(pos2) < 50.0 {
                    // Interact
                    let concept = Concept::random();

                    let (a1, a2) = if idx1 < idx2 {
                        let (left, right) = agents.split_at_mut(idx2);
                        (&mut left[idx1], &mut right[0])
                    } else {
                        let (left, right) = agents.split_at_mut(idx1);
                        (&mut right[0], &mut left[idx2])
                    };

                    // a1 speaks
                    let word = a1.speak(concept);
                    let success = a2.listen(concept, &word);

                    interactions.push(InteractionEvent {
                        p1: pos1,
                        p2: pos2,
                        word: format!("{} ({:?})", word, concept),
                        success,
                        timer: 2.0,
                    });

                    // Push them apart slightly
                    let push = (pos1 - pos2).normalize_or_zero() * 10.0;
                    a1.position.0 += push.x;
                    a1.position.1 += push.y;
                    a2.position.0 -= push.x;
                    a2.position.1 -= push.y;
                }
            }
        }

        // Cleanup interactions
        interactions.retain_mut(|i| {
            i.timer -= dt;
            i.timer > 0.0
        });

        // Draw Agents
        for agent in &agents {
            draw_circle(
                agent.position.0,
                agent.position.1,
                8.0,
                Color::from_rgba(agent.color.0, agent.color.1, agent.color.2, 255),
            );
        }

        // Draw Interactions
        for i in &interactions {
            let alpha = (i.timer / 2.0).clamp(0.0, 1.0);
            let color = if i.success {
                Color::new(0.0, 1.0, 0.0, alpha)
            } else {
                Color::new(1.0, 0.0, 0.0, alpha)
            };
            draw_line(i.p1.x, i.p1.y, i.p2.x, i.p2.y, 2.0, color);
            let mid = (i.p1 + i.p2) * 0.5;

            // Draw text background for readability
            let text_dims = measure_text(&i.word, None, 16, 1.0);
            draw_rectangle(
                mid.x,
                mid.y - 10.0 - text_dims.offset_y,
                text_dims.width + 4.,
                text_dims.height + 4.,
                Color::new(0., 0., 0., alpha * 0.8),
            );

            draw_text(
                &i.word,
                mid.x,
                mid.y - 10.0,
                16.0,
                Color::new(1.0, 1.0, 1.0, alpha),
            );
        }

        draw_text(
            "Genesis: Lingua Franca - Pidgin Evolution",
            20.0,
            30.0,
            30.0,
            WHITE,
        );
        draw_text(
            "Blue: Nords (CVC) | Red: Sudrons (VCV)",
            20.0,
            60.0,
            20.0,
            GRAY,
        );

        // Draw Pidgin Stats
        let mut y = 100.0;
        draw_text("Dominant Pidgin:", 20.0, y, 20.0, WHITE);
        y += 20.0;
        for (concept, word, count) in &dominant_words {
            draw_text(
                &format!("{:?}: {} ({} agents)", concept, word, count),
                20.0,
                y,
                16.0,
                LIGHTGRAY,
            );
            y += 20.0;
        }

        next_frame().await
    }
}
