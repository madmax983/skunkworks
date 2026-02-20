mod audio;
mod dna;
mod model;
mod musician;
mod tui;

use anyhow::Result;
use chimera_lang::vm::ChimeraVM;
use crossbeam_channel::unbounded;
use std::sync::{Arc, Mutex, atomic::AtomicBool};
use std::thread;
use crate::audio::start_audio_thread;
use crate::dna::{generate_rhythm_dna, generate_jazz_dna};
use crate::model::Instrument;
use crate::musician::Musician;
use crate::tui::run_tui;

fn main() -> Result<()> {
    // 1. Audio
    let (audio_sender, audio_receiver) = unbounded();
    let audio_handle = start_audio_thread(audio_receiver);

    // 2. State Channel
    let (state_sender, state_receiver) = unbounded();

    // 3. Instruments
    let kick: Instrument = Arc::new(Mutex::new(()));
    let snare: Instrument = Arc::new(Mutex::new(()));
    let hat: Instrument = Arc::new(Mutex::new(()));

    // We need a vector of instruments to pass to musicians
    let instruments = vec![kick.clone(), snare.clone(), hat.clone()];

    let running = Arc::new(AtomicBool::new(true));
    let mut handles = vec![];

    // Thread 0: Kick (4/4 - 500ms)
    // generate_rhythm_dna(inst, sustain, rest)
    // Inst 0 = Kick
    let kick_dna = generate_rhythm_dna(0, 50, 450);
    let kick_vm = ChimeraVM::new(kick_dna);
    let kick_musician = Musician::new(
        0,
        kick_vm,
        instruments.clone(),
        audio_sender.clone(),
        state_sender.clone(),
        running.clone()
    );
    handles.push(thread::spawn(move || kick_musician.run()));

    // Thread 1: Snare (Polyrhythm - 333ms)
    let snare_dna = generate_rhythm_dna(1, 50, 283);
    let snare_vm = ChimeraVM::new(snare_dna);
    let snare_musician = Musician::new(
        1,
        snare_vm,
        instruments.clone(),
        audio_sender.clone(),
        state_sender.clone(),
        running.clone()
    );
    handles.push(thread::spawn(move || snare_musician.run()));

    // Thread 2: Hat (Fast - 200ms)
    let hat_dna = generate_rhythm_dna(2, 20, 180);
    let hat_vm = ChimeraVM::new(hat_dna);
    let hat_musician = Musician::new(
        2,
        hat_vm,
        instruments.clone(),
        audio_sender.clone(),
        state_sender.clone(),
        running.clone()
    );
    handles.push(thread::spawn(move || hat_musician.run()));

    // Thread 3: Jazz (Random)
    let jazz_dna = generate_jazz_dna(3);
    let jazz_vm = ChimeraVM::new(jazz_dna);
    let jazz_musician = Musician::new(
        3,
        jazz_vm,
        instruments.clone(),
        audio_sender.clone(),
        state_sender.clone(),
        running.clone()
    );
    handles.push(thread::spawn(move || jazz_musician.run()));

    // 5. Run TUI
    run_tui(state_receiver, 4)?;

    // 6. Cleanup
    running.store(false, std::sync::atomic::Ordering::Relaxed);
    let _ = audio_sender.send(crate::model::AudioCommand::Stop);

    for handle in handles {
        let _ = handle.join();
    }
    let _ = audio_handle.join();

    Ok(())
}
