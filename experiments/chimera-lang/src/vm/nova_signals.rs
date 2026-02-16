#![cfg(feature = "nova")]

#[cfg(feature = "biophysics")]
use super::neuron::Neuron;
#[cfg(feature = "nova")]
use super::nova_sigil;
#[cfg(feature = "oracle")]
use super::oracle;
use super::{ChimeraVM, MidiEvent, Value, GRID_SIZE};
use crate::ast::{JunctionType, Nucleotide};
use crate::opcode::OpCode;
use rand::Rng;
use std::collections::HashMap;

const GOLDEN_FREQUENCIES: [f32; 4] = [161.8, 261.6, 432.0, 528.0];

fn char_to_val(c: char) -> Option<i64> {
    match c {
        '0'..='9' => Some(c as i64 - '0' as i64),
        'a'..='z' => Some(c as i64 - 'a' as i64 + 10),
        'A'..='Z' => Some(c as i64 - 'A' as i64 + 10),
        _ => None,
    }
}

fn val_to_char(v: i64) -> char {
    let v = v.rem_euclid(36);
    if v < 10 {
        ((v as u8) + b'0') as char
    } else {
        ((v as u8 - 10) + b'a') as char
    }
}

fn peek(vm: &ChimeraVM, y: usize, x: usize, dy: i64, dx: i64) -> Option<i64> {
    if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
        match &vm.grid[ny][nx] {
            Value::Int(n) => Some(*n),
            Value::Str(s) => {
                if s.len() == 1 {
                    char_to_val(s.chars().next().unwrap())
                } else {
                    None
                }
            }
            _ => None,
        }
    } else {
        None
    }
}

fn peek_char(vm: &ChimeraVM, y: usize, x: usize, dy: i64, dx: i64) -> Option<char> {
    if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
        match &vm.grid[ny][nx] {
            Value::Str(s) => s.chars().next(),
            Value::Int(n) => Some(val_to_char(*n)),
            _ => None,
        }
    } else {
        None
    }
}

struct GridWrite {
    y: usize,
    x: usize,
    val: Value,
}

struct DnaWrite {
    strand_idx: usize,
    gene_idx: usize,
    val: Nucleotide,
}

struct DnaAppend {
    strand_idx: usize,
    gene: crate::ast::Gene,
}

struct ResonanceWrite {
    y: usize,
    x: usize,
    freq: f32,
    amp: f32,
}

struct EntropyWrite {
    y: usize,
    x: usize,
    val: i64,
}

struct MutationRequest {
    strand_idx: usize,
}

struct SpawnRequest {
    y: usize,
    x: usize,
    kind: crate::vm::nova::OrganelleType,
}

struct EtherWrite {
    channel: i64,
    val: Value,
}

struct EtherRead {
    channel: i64,
    y: usize,
    x: usize,
}

#[cfg(feature = "biophysics")]
struct NeuronStimulus {
    y: usize,
    x: usize,
    amount: f32,
}

struct HoloWrite {
    y: usize,
    x: usize,
    re: Option<f64>,
    im: Option<f64>,
}

struct VoltageWrite {
    y: usize,
    x: usize,
    amount: f32,
}

struct OperatorRegister {
    char_val: char,
    strand_idx: usize,
}

struct SignalContext {
    next_signals: Vec<Vec<u8>>,
    grid_writes: Vec<GridWrite>,
    dna_writes: Vec<DnaWrite>,
    dna_appends: Vec<DnaAppend>,
    resonance_writes: Vec<ResonanceWrite>,
    voltage_writes: Vec<VoltageWrite>,
    entropy_writes: Vec<EntropyWrite>,
    mutation_requests: Vec<MutationRequest>,
    spawn_requests: Vec<SpawnRequest>,
    phage_updates: Vec<PhageUpdate>,
    phage_clones: Vec<PhageCloneRequest>,
    ether_writes: Vec<EtherWrite>,
    ether_reads: Vec<EtherRead>,
    holo_writes: Vec<HoloWrite>,
    operator_registers: Vec<OperatorRegister>,
    #[cfg(feature = "biophysics")]
    neuron_stimuli: Vec<NeuronStimulus>,
    executions: Vec<(OpCode, Vec<Nucleotide>)>,
    midi_events: Vec<MidiEvent>,
}

struct PhageUpdate {
    organelle_idx: usize,
    new_loc: (usize, usize),
    new_dir: (i8, i8),
}

struct PhageCloneRequest {
    strand_idx: usize,
}

pub fn process_signals(vm: &mut ChimeraVM) {
    let size = GRID_SIZE;
    let mut ctx = SignalContext {
        next_signals: vec![vec![0u8; size]; size],
        grid_writes: Vec::new(),
        dna_writes: Vec::new(),
        dna_appends: Vec::new(),
        resonance_writes: Vec::new(),
        voltage_writes: Vec::new(),
        entropy_writes: Vec::new(),
        mutation_requests: Vec::new(),
        spawn_requests: Vec::new(),
        phage_updates: Vec::new(),
        phage_clones: Vec::new(),
        ether_writes: Vec::new(),
        ether_reads: Vec::new(),
        holo_writes: Vec::new(),
        operator_registers: Vec::new(),
        #[cfg(feature = "biophysics")]
        neuron_stimuli: Vec::new(),
        executions: Vec::new(),
        midi_events: Vec::new(),
    };

    // 0. Process Phages
    process_phages(vm, &mut ctx);

    // 0.5. Process Neuro-Chemicals
    #[cfg(feature = "biophysics")]
    for ((y, x), neuron) in &vm.neurons {
        for c in 0..3 {
            let level = vm.hormone_grid[*y][*x][c] as f32;
            let (sensitivity, threshold) = neuron.receptors[c];
            if level > threshold {
                let amount = level * sensitivity;
                if amount > 0.0 {
                    ctx.neuron_stimuli.push(NeuronStimulus {
                        y: *y,
                        x: *x,
                        amount,
                    });
                }
            }
        }
    }

    // 1. Scan Phase
    for y in 0..size {
        for x in 0..size {
            let signal = vm.signal_grid[y][x];

            let val = &vm.grid[y][x];
            let c = match val {
                Value::Str(s) => {
                    let mut chars = s.chars();
                    if let Some(first) = chars.next() {
                        if chars.next().is_none() {
                            first
                        } else {
                            match s.as_str() {
                                "*" | ">" | "<" | "^" | "v" | "+" => first,
                                _ => '\0',
                            }
                        }
                    } else {
                        '\0'
                    }
                }
                _ => '\0',
            };

            let is_uppercase = c.is_uppercase(); // Use Unicode uppercase
            let is_bang = c == '*';
            let is_special = matches!(c, '@' | '^' | 'Ψ' | 'ψ' | 'Φ' | 'φ' | 'Ω' | 'ω');
            let active = signal > 0 || is_uppercase || is_bang || is_special;

            if !active {
                continue;
            }

            // 1.1 Check Overrides (Custom Operators)
            if let Some(&strand_idx) = vm.custom_operators.get(&c) {
                if signal > 0 {
                    // Push context (y, x) to stack
                    ctx.executions
                        .push((OpCode::Push, vec![Nucleotide::Number(y as i64)]));
                    ctx.executions
                        .push((OpCode::Push, vec![Nucleotide::Number(x as i64)]));
                    ctx.executions
                        .push((OpCode::Call, vec![Nucleotide::Number(strand_idx as i64)]));
                }
                continue;
            }

            match c {
                #[cfg(feature = "biophysics")]
                '@' => {
                    // Neuron
                    // 1. Spiking Output
                    if let Some(neuron) = vm.neurons.get(&(y, x)) {
                        if neuron.v > 0.0 {
                            let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                            for (dy, dx) in neighbors {
                                if let Some((ny, nx)) =
                                    vm.normalize_coords(y as i64 + dy, x as i64 + dx)
                                {
                                    ctx.next_signals[ny][nx] =
                                        ctx.next_signals[ny][nx].saturating_add(1);
                                }
                            }
                        }
                    } else if signal > 0 {
                        // Neurogenesis if signaled and missing
                    }

                    // 2. Input Stimulus
                    if signal > 0 {
                        ctx.neuron_stimuli
                            .push(NeuronStimulus { y, x, amount: 50.0 });
                    }
                }
                #[cfg(feature = "biophysics")]
                '^' => {
                    // Synapse: Read South (Input), Stimulate North (Target)
                    if let Some(val) = peek(vm, y, x, 1, 0) {
                        if let Some((ny, nx)) = vm.normalize_coords(y as i64 - 1, x as i64) {
                            let weight = val as f32;
                            if weight > 0.0 {
                                ctx.neuron_stimuli.push(NeuronStimulus {
                                    y: ny,
                                    x: nx,
                                    amount: weight * 5.0,
                                });
                            }
                        }
                    }
                }
                '*' | '!' => {
                    // Bang
                    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                    for (dy, dx) in neighbors {
                        if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
                            ctx.next_signals[ny][nx] = ctx.next_signals[ny][nx].saturating_add(1);
                        }
                    }
                }
                '>' => propagate_directional(vm, y, x, 0, 1, 1, &mut ctx.next_signals),
                '<' => propagate_directional(vm, y, x, 0, -1, 1, &mut ctx.next_signals),
                #[cfg(not(feature = "biophysics"))]
                '^' => propagate_directional(vm, y, x, -1, 0, 1, &mut ctx.next_signals),
                'v' => propagate_directional(vm, y, x, 1, 0, 1, &mut ctx.next_signals),
                '+' => {
                    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                    for (dy, dx) in neighbors {
                        if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
                            ctx.next_signals[ny][nx] =
                                ctx.next_signals[ny][nx].saturating_add(signal);
                        }
                    }
                }
                'N' => read_write_directional(vm, y, x, -1, 0, 1, 0, &mut ctx),
                'S' => read_write_directional(vm, y, x, 1, 0, -1, 0, &mut ctx),
                'E' => read_write_directional(vm, y, x, 0, 1, 0, -1, &mut ctx),
                'W' => read_write_directional(vm, y, x, 0, -1, 0, 1, &mut ctx),
                'A' | 'a' => binary_op(vm, y, x, &mut ctx.grid_writes, |a, b| a.wrapping_add(b)),
                'B' | 'b' => binary_op(vm, y, x, &mut ctx.grid_writes, |a, b| a.wrapping_sub(b)),
                'D' | 'd' => binary_op(vm, y, x, &mut ctx.grid_writes, |a, b| {
                    if b != 0 {
                        a.wrapping_div(b)
                    } else {
                        0
                    }
                }),
                'M' | 'm' => exec_mutate(vm, y, x, signal, &mut ctx),
                'T' | 't' => exec_teleport(vm, y, x, &mut ctx),
                'L' | 'l' => exec_laser(vm, y, x, &mut ctx),
                'Z' | 'z' => exec_resonate(vm, y, x, signal, &mut ctx),
                'I' | 'i' => exec_increment(vm, y, x, &mut ctx),
                'R' | 'r' => exec_random(vm, y, x, &mut ctx),
                'C' | 'c' => exec_clock(vm, y, x, &mut ctx),
                'X' | 'x' => exec_write(vm, y, x, &mut ctx),
                'O' | 'o' => exec_offset(vm, y, x, &mut ctx),
                'G' | 'g' => exec_gene_read(vm, y, x, &mut ctx),
                'P' | 'p' => exec_play(vm, y, x, signal, &mut ctx),
                'K' | 'k' => exec_kill(vm, y, x, signal, &mut ctx),
                'Y' | 'y' => exec_synthesize(vm, y, x, signal, &mut ctx),
                'Q' | 'q' => exec_query(vm, y, x, &mut ctx),
                'H' | 'h' => exec_project_signal(vm, y, x, signal, &mut ctx),
                'U' | 'u' => exec_unzip(vm, y, x, signal, &mut ctx),
                'F' | 'f' => exec_flux(vm, y, x, signal, &mut ctx),
                'J' | 'j' => exec_jam(vm, y, x, signal, &mut ctx),
                ':' => exec_midi_note(vm, y, x, signal, &mut ctx),
                ';' => exec_midi_cc(vm, y, x, signal, &mut ctx),
                #[cfg(feature = "oracle")]
                '?' => exec_oracle(vm, y, x, &mut ctx),
                #[cfg(not(feature = "oracle"))]
                '?' => exec_random(vm, y, x, &mut ctx), // Fallback
                'V' => exec_voltage(vm, y, x, signal, &mut ctx),
                'E' | 'e' => exec_electrode(vm, y, x, signal, &mut ctx),
                '%' => binary_op(vm, y, x, &mut ctx.grid_writes, |a, b| {
                    if b != 0 {
                        a.rem_euclid(b)
                    } else {
                        0
                    }
                }),
                '=' => binary_op(
                    vm,
                    y,
                    x,
                    &mut ctx.grid_writes,
                    |a, b| {
                        if a == b {
                            1
                        } else {
                            0
                        }
                    },
                ),
                '&' => binary_op(vm, y, x, &mut ctx.grid_writes, |a, b| a & b),
                '|' => binary_op(vm, y, x, &mut ctx.grid_writes, |a, b| a | b),
                '[' => exec_ether_send(vm, y, x, signal, &mut ctx),
                ']' => exec_ether_recv(vm, y, x, signal, &mut ctx),
                '#' => exec_catalyze(vm, y, x, signal, &mut ctx),
                '$' => exec_stack_io(vm, y, x, signal, &mut ctx),
                '~' => exec_wave(vm, y, x, signal, &mut ctx),
                'Ψ' | 'ψ' => exec_psi(vm, y, x, signal, &mut ctx),
                'Φ' | 'φ' => exec_phi(vm, y, x, signal, &mut ctx),
                'Ω' | 'ω' => exec_omega(vm, y, x, signal, &mut ctx),
                '§' => exec_sigil(vm, y, x, signal, &mut ctx),
                'ƒ' => exec_function_op(vm, y, x, signal, &mut ctx),
                _ => {
                    if let Value::Str(s) = val {
                        if let Ok(op) = s.parse::<OpCode>() {
                            if signal > 0 {
                                ctx.executions.push((op, vec![]));
                            }
                        } else if signal > 0 {
                            // Check for Dynamic Operators in KB
                            #[cfg(feature = "oracle")]
                            if let Some(strand_idx) = check_kb_operator(vm, s.as_str()) {
                                ctx.executions
                                    .push((OpCode::Call, vec![Nucleotide::Number(strand_idx)]));
                            }

                            // Check for Named Sigils
                            #[cfg(feature = "nova")]
                            if let Some(sigil) = vm.sigil_registry.get(s) {
                                if nova_sigil::check_dynamic_pattern(vm, y, x, &sigil.pattern) {
                                    ctx.executions.push((
                                        OpCode::Call,
                                        vec![Nucleotide::Number(sigil.strand_idx as i64)],
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // 1.5 Resonance Check (Mutagenic & Harmonic)
    #[cfg(feature = "resonance")]
    {
        // Mutagenic Resonance
        for org in &vm.organelles {
            let (y, x) = org.context_loc;
            let idx = y * GRID_SIZE + x;
            if idx < vm.audio_snapshot.pressure.len() {
                if vm.audio_snapshot.pressure[idx].abs() > 0.8 {
                    ctx.mutation_requests.push(MutationRequest {
                        strand_idx: org.ip.0,
                    });
                }
            }
        }

        // Harmonic Convergence
        for y in 0..size {
            for x in 0..size {
                let (freq, amp) = vm.resonance_grid[y][x];
                if amp > 10.0 {
                    // Check for Golden Frequencies (approx)
                    // 161.8 (Phi*100), 261.6 (C4), 432.0 (Verdi A), 528.0 (Solfeggio)
                    for g in GOLDEN_FREQUENCIES {
                        if (freq - g).abs() < 5.0 {
                            ctx.spawn_requests.push(SpawnRequest {
                                y,
                                x,
                                kind: crate::vm::nova::OrganelleType::Wisp,
                            });
                            break;
                        }
                    }
                }
            }
        }
    }

    // 1.8 Apply Phage Updates
    for update in ctx.phage_updates {
        if update.organelle_idx < vm.organelles.len() {
            vm.organelles[update.organelle_idx].context_loc = update.new_loc;
            vm.organelles[update.organelle_idx].direction = update.new_dir;
        }
    }

    for clone_req in ctx.phage_clones {
        if clone_req.strand_idx < vm.dna.helix.strands.len() {
            if vm.dna.helix.strands.len() < crate::vm::MAX_STRANDS {
                let new_strand = vm.dna.helix.strands[clone_req.strand_idx].clone();
                vm.dna.helix.strands.push(new_strand);
                vm.telomeres.push(50);
                #[cfg(feature = "cortex")]
                {
                    vm.activation_levels.push(0);
                    vm.synapse_map.push(Vec::new());
                }
                let new_idx = vm.dna.helix.strands.len() - 1;
                vm.cladistics.register_strand(
                    new_idx,
                    Some(clone_req.strand_idx),
                    vm.tick_counter,
                    "PhageInfection".to_string(),
                );
                vm.output.push(format!(
                    "PHAGE: Injected strand {} as {}",
                    clone_req.strand_idx, new_idx
                ));
            }
        }
    }

    // 2. Apply Writes
    for w in ctx.grid_writes {
        vm.grid[w.y][w.x] = w.val;
    }

    // 2.2 Apply Voltage Writes
    #[cfg(feature = "elektra")]
    for w in ctx.voltage_writes {
        if w.y < vm.voltage_grid.len() && w.x < vm.voltage_grid[0].len() {
            vm.voltage_grid[w.y][w.x] += w.amount;
        }
    }

    // 2.5 Apply DNA Writes
    for w in ctx.dna_writes {
        if w.strand_idx < vm.dna.helix.strands.len() {
            let strand = &mut vm.dna.helix.strands[w.strand_idx];
            if w.gene_idx < strand.genes.len() {
                let gene = &mut strand.genes[w.gene_idx];
                if !gene.args.is_empty() {
                    gene.args[0] = w.val;
                } else {
                    gene.args.push(w.val);
                }
            }
        }
    }

    for w in ctx.dna_appends {
        if w.strand_idx < vm.dna.helix.strands.len() {
            vm.dna.helix.strands[w.strand_idx].genes.push(w.gene);
        }
    }

    // 2.75 Apply Ether Ops
    for w in ctx.ether_writes {
        vm.ether.entry(w.channel).or_default().push_back(w.val);
    }

    for r in ctx.ether_reads {
        if let Some(queue) = vm.ether.get_mut(&r.channel) {
            if let Some(val) = queue.pop_front() {
                let v: Value = val;
                vm.grid[r.y][r.x] = v;
            }
        }
    }

    // 3. Update Signal State
    vm.signal_grid = ctx.next_signals;

    // 3.5 Apply Resonance, Entropy & Mutations
    for w in ctx.resonance_writes {
        vm.resonance_grid[w.y][w.x] = (w.freq, w.amp);
    }

    for w in ctx.holo_writes {
        if let Some(re) = w.re {
            vm.hologram_grid[w.y][w.x].0 = re;
        }
        if let Some(im) = w.im {
            vm.hologram_grid[w.y][w.x].1 = im;
        }
    }

    #[cfg(feature = "biophysics")]
    for s in ctx.neuron_stimuli {
        let coord = (s.y, s.x);
        if let Some(neuron) = vm.neurons.get_mut(&coord) {
            neuron.i_inj += s.amount;
        } else {
            // Auto-Neurogenesis
            let mut neuron = Neuron::new();
            neuron.i_inj += s.amount;
            vm.neurons.insert(coord, neuron);
        }
    }

    for w in ctx.entropy_writes {
        vm.entropy_grid[w.y][w.x] = vm.entropy_grid[w.y][w.x]
            .saturating_add(w.val)
            .clamp(0, 100);
    }

    for req in ctx.mutation_requests {
        if req.strand_idx < vm.dna.helix.strands.len() {
            let strand_len = vm.dna.helix.strands[req.strand_idx].genes.len();
            if strand_len > 0 {
                let mut rng = rand::thread_rng();
                let g_idx = rng.gen_range(0..strand_len);
                // Mutate Arg
                if !vm.dna.helix.strands[req.strand_idx].genes[g_idx]
                    .args
                    .is_empty()
                {
                    let val = rng.gen_range(0..100);
                    vm.dna.helix.strands[req.strand_idx].genes[g_idx].args[0] =
                        Nucleotide::Number(val);
                    vm.output
                        .push(format!("MUTATION: Resonance hit strand {}", req.strand_idx));
                }
            }
        }
    }

    for req in ctx.spawn_requests {
        if vm.organelles.len() < crate::vm::MAX_ORGANELLES {
            vm.organelle_id_counter += 1;
            let new_org = crate::vm::nova::Organelle {
                stack: Vec::new(),
                ip: (0, 0), // Default start
                context_loc: (req.y, req.x),
                call_stack: Vec::new(),
                recursion_depth: 0,
                halted: false,
                kind: req.kind,
                direction: (0, 0),
                ttl: Some(100), // Finite life
                name: "Resonance Child".to_string(),
                traits: vec!["Harmonic".to_string()],
                id: vm.organelle_id_counter,
                tissue_id: None,
                genome_id: 0,
                energy: 50,
                experience: 0,
                stage: 0,
            };
            vm.organelles.push(new_org);
            vm.output
                .push(format!("HARMONIC: Spawned Wisp at {},{}", req.x, req.y));
        }
    }

    // 3.7 Apply Operator Registers
    for reg in ctx.operator_registers {
        vm.custom_operators.insert(reg.char_val, reg.strand_idx);
        vm.output.push(format!(
            "ORCA: Registered operator '{}' -> {}",
            reg.char_val, reg.strand_idx
        ));
    }

    // 3.75 Apply MIDI
    vm.midi_messages.extend(ctx.midi_events);

    // 4. Execution Phase
    for (op, args) in ctx.executions {
        if let Some(target) = vm.execute_gene_inner(op.clone(), &args) {
            vm.ip = target;
        }
    }
}

fn exec_midi_note(vm: &ChimeraVM, y: usize, x: usize, signal: u8, ctx: &mut SignalContext) {
    if signal == 0 {
        return;
    }
    // Layout:
    // . N .
    // C : V
    // . D .
    // N: Note (North) - 0-35 (Base 48 -> C3 to B5)
    // V: Velocity (East) - 0-35 (Scaled to 0-127)
    // C: Channel (West) - 0-15
    // D: Duration (South) - 0-35 (16th notes?)

    let n_val = peek(vm, y, x, -1, 0).unwrap_or(0);
    let v_val = peek(vm, y, x, 0, 1).unwrap_or(35); // Default full volume
    let c_val = peek(vm, y, x, 0, -1).unwrap_or(0);
    let d_val = peek(vm, y, x, 1, 0).unwrap_or(1);

    // Mapping
    // Note: 0-35 + 48 (C3)
    let note = (n_val.clamp(0, 35) + 48) as u8;
    // Velocity: 0-35 -> 0-127 (v * 127 / 35)
    let velocity = ((v_val.clamp(0, 35) as f32 / 35.0) * 127.0) as u8;
    let channel = c_val.clamp(0, 15) as u8;
    let duration = d_val.clamp(1, 35) as u8;

    ctx.midi_events.push(MidiEvent::NoteOn {
        channel,
        note,
        velocity,
        duration,
    });
}

fn exec_midi_cc(vm: &ChimeraVM, y: usize, x: usize, signal: u8, ctx: &mut SignalContext) {
    if signal == 0 {
        return;
    }
    // Layout:
    // . K .
    // C ; V
    // . . .
    // K: Knob/Controller (North)
    // V: Value (East)
    // C: Channel (West)

    let k_val = peek(vm, y, x, -1, 0).unwrap_or(0);
    let v_val = peek(vm, y, x, 0, 1).unwrap_or(0);
    let c_val = peek(vm, y, x, 0, -1).unwrap_or(0);

    let controller = k_val.clamp(0, 127) as u8;
    let value = ((v_val.clamp(0, 35) as f32 / 35.0) * 127.0) as u8;
    let channel = c_val.clamp(0, 15) as u8;

    ctx.midi_events.push(MidiEvent::ControlChange {
        channel,
        controller,
        value,
    });
}

fn exec_mutate(vm: &ChimeraVM, y: usize, x: usize, signal: u8, ctx: &mut SignalContext) {
    if let Some(s_idx) = peek(vm, y, x, 0, -1) {
        if signal > 0 {
            ctx.mutation_requests.push(MutationRequest {
                strand_idx: s_idx as usize,
            });
        }
    }
}

fn exec_electrode(vm: &ChimeraVM, y: usize, x: usize, signal: u8, ctx: &mut SignalContext) {
    if signal == 0 {
        return;
    }
    // E: Electrode
    // North: Amount (0-9/a-z)
    // Injects voltage into the electrical grid at this location.

    if let Some(val) = peek(vm, y, x, -1, 0) {
        ctx.voltage_writes.push(VoltageWrite {
            y,
            x,
            amount: val as f32,
        });
    }
}

fn exec_function_op(vm: &ChimeraVM, y: usize, x: usize, signal: u8, ctx: &mut SignalContext) {
    if signal == 0 {
        return;
    }
    // ƒ: Function Definition
    // North: Character (Glyph)
    // East: Strand Index

    if let (Some(c), Some(strand_idx)) = (peek_char(vm, y, x, -1, 0), peek(vm, y, x, 0, 1)) {
        ctx.operator_registers.push(OperatorRegister {
            char_val: c,
            strand_idx: strand_idx as usize,
        });
    }
}

fn exec_stack_io(vm: &ChimeraVM, y: usize, x: usize, signal: u8, ctx: &mut SignalContext) {
    if signal == 0 {
        return;
    }
    // North: Mode (0=Pop, 1=Push)
    let mode = peek(vm, y, x, -1, 0).unwrap_or(0);

    if mode == 1 {
        // Push: Read East -> Stack
        if let Some(val) = peek(vm, y, x, 0, 1) {
            ctx.executions
                .push((OpCode::Push, vec![Nucleotide::Number(val)]));
        }
    } else {
        // Pop: Stack -> South
        // Synthesize GWrite: Stack [..., val] -> GWrite(val, sy, sx)
        if let Some((sy, sx)) = vm.normalize_coords(y as i64 + 1, x as i64) {
            ctx.executions
                .push((OpCode::Push, vec![Nucleotide::Number(sy as i64)]));
            ctx.executions
                .push((OpCode::Push, vec![Nucleotide::Number(sx as i64)]));
            ctx.executions.push((OpCode::GWrite, vec![]));
        }
    }
}

fn exec_voltage(_vm: &ChimeraVM, _y: usize, _x: usize, _signal: u8, _ctx: &mut SignalContext) {
    // V: Voltmeter
    // North: Threshold
    // East: Output Bang '*' if Voltage > Threshold.

    #[cfg(feature = "elektra")]
    {
        // Remove underscore prefixes to use them
        let vm = _vm;
        let y = _y;
        let x = _x;
        let ctx = _ctx;

        let threshold = peek(vm, y, x, -1, 0).unwrap_or(0);

        // Check bounds
        if y < vm.voltage_grid.len() && x < vm.voltage_grid[0].len() {
            let voltage = vm.voltage_grid[y][x];
            if voltage > threshold as f32 {
                if let Some((ey, ex)) = vm.normalize_coords(y as i64, x as i64 + 1) {
                    ctx.grid_writes.push(GridWrite {
                        y: ey,
                        x: ex,
                        val: Value::Str("*".to_string()),
                    });
                }
            }
        }
    }
}

#[cfg(feature = "oracle")]
fn exec_oracle(vm: &ChimeraVM, y: usize, x: usize, ctx: &mut SignalContext) {
    // ?: Oracle Query
    // North: Fact ID (Int)
    // Query KB for fact(ID).
    // If found, Output '1' to South.

    let fact_id = peek(vm, y, x, -1, 0).unwrap_or(0);

    // Construct query: fact(ID)
    // We assume facts in KB are just Values.
    // If KB contains Value::Int(fact_id), it's a match.
    // Or if KB contains `fact(fact_id)`.
    // Let's support both.

    let mut found = false;

    // Direct check
    let query_val = Value::Int(fact_id);
    if vm.knowledge_base.contains(&query_val) {
        found = true;
    } else {
        // Predicate check: fact(ID)
        let goal = Value::Junction(
            JunctionType::Any,
            vec![Value::Str("fact".to_string()), query_val],
        );
        let mut solutions = Vec::new();
        oracle::solve(
            &[goal],
            HashMap::new(),
            &vm.knowledge_base,
            vm,
            &mut solutions,
            0,
        );
        if !solutions.is_empty() {
            found = true;
        }
    }

    if found {
        if let Some((sy, sx)) = vm.normalize_coords(y as i64 + 1, x as i64) {
            ctx.grid_writes.push(GridWrite {
                y: sy,
                x: sx,
                val: Value::Str("1".to_string()),
            });
        }
    }
}

fn exec_catalyze(vm: &ChimeraVM, y: usize, x: usize, signal: u8, ctx: &mut SignalContext) {
    if signal == 0 {
        return;
    }
    // Inputs: North (Catalyst ID), East (Target Strand)
    if let (Some(cat_id), Some(strand_idx)) = (peek(vm, y, x, -1, 0), peek(vm, y, x, 0, 1)) {
        ctx.executions.push((
            OpCode::Catalyze,
            vec![Nucleotide::Number(cat_id), Nucleotide::Number(strand_idx)],
        ));
    }
}

fn exec_unzip(vm: &ChimeraVM, y: usize, x: usize, signal: u8, ctx: &mut SignalContext) {
    if signal == 0 {
        return;
    }
    // Inputs: West (Strand), East (Index)
    let s_idx = peek(vm, y, x, 0, -1);
    let g_idx = peek(vm, y, x, 0, 1);

    if let (Some(s), Some(g)) = (s_idx, g_idx) {
        let s_val = s as usize;
        let g_val = g as usize;
        if s_val < vm.dna.helix.strands.len() {
            let strand = &vm.dna.helix.strands[s_val];
            if g_val < strand.genes.len() {
                let op_str = strand.genes[g_val].op.to_string();
                // Write southwards
                for (i, c) in op_str.chars().enumerate() {
                    if let Some((sy, sx)) = vm.normalize_coords(y as i64 + 1 + i as i64, x as i64) {
                        ctx.grid_writes.push(GridWrite {
                            y: sy,
                            x: sx,
                            val: Value::Str(c.to_string()),
                        });
                    }
                }
            }
        }
    }
}

fn exec_query(vm: &ChimeraVM, y: usize, x: usize, ctx: &mut SignalContext) {
    if let (Some(dir_code), Some(target_val)) = (peek(vm, y, x, -1, 0), peek(vm, y, x, 0, 1)) {
        let (dy, dx) = match dir_code % 4 {
            0 => (-1, 0), // N
            1 => (0, 1),  // E
            2 => (1, 0),  // S
            3 => (0, -1), // W
            _ => (0, 0),
        };

        let target_char = val_to_char(target_val);
        let actual_val = peek(vm, y, x, dy, dx);

        let is_match = if let Some(val) = actual_val {
            val_to_char(val) == target_char
        } else {
            false
        };

        let res = if is_match { 1 } else { 0 };

        if let Some((sy, sx)) = vm.normalize_coords(y as i64 + 1, x as i64) {
            ctx.grid_writes.push(GridWrite {
                y: sy,
                x: sx,
                val: Value::Str(val_to_char(res).to_string()),
            });
        }
    }
}

fn exec_teleport(vm: &ChimeraVM, y: usize, x: usize, ctx: &mut SignalContext) {
    if let (Some(x_val), Some(y_val), Some(val)) = (
        peek(vm, y, x, 0, -1),
        peek(vm, y, x, 0, 1),
        peek(vm, y, x, -1, 0),
    ) {
        if let Some((ty, tx)) = vm.normalize_coords(y_val, x_val) {
            ctx.grid_writes.push(GridWrite {
                y: ty,
                x: tx,
                val: Value::Str(val_to_char(val).to_string()),
            });
        }
    }
}

fn exec_laser(vm: &ChimeraVM, y: usize, x: usize, ctx: &mut SignalContext) {
    if let (Some(len), Some(dir)) = (peek(vm, y, x, 0, -1), peek(vm, y, x, -1, 0)) {
        let mut dy = match dir % 4 {
            0 => -1,
            1 => 0,
            2 => 1,
            3 => 0,
            _ => 0,
        };
        let mut dx = match dir % 4 {
            0 => 0,
            1 => 1,
            2 => 0,
            3 => -1,
            _ => 0,
        };

        let mut cy = y as i64;
        let mut cx = x as i64;

        for _ in 0..len.min(16) {
            if let Some((ny, nx)) = vm.normalize_coords(cy + dy, cx + dx) {
                ctx.next_signals[ny][nx] = ctx.next_signals[ny][nx].saturating_add(1);
                cy = ny as i64;
                cx = nx as i64;

                // Check for Mirror
                if let Value::Str(s) = &vm.grid[ny][nx] {
                    if s == "/" {
                        // Reflect /: (0,1)->(-1,0), (0,-1)->(1,0), (1,0)->(0,-1), (-1,0)->(0,1)
                        let old_dy = dy;
                        dy = -dx;
                        dx = -old_dy;
                    } else if s == "\\" {
                        // Reflect \: (0,1)->(1,0), (0,-1)->(-1,0), (1,0)->(0,1), (-1,0)->(0,-1)
                        let old_dy = dy;
                        dy = dx;
                        dx = old_dy;
                    }
                }
            } else {
                break;
            }
        }
    }
}

fn exec_resonate(vm: &ChimeraVM, y: usize, x: usize, signal: u8, ctx: &mut SignalContext) {
    if let (Some(freq), Some(amp)) = (peek(vm, y, x, 0, -1), peek(vm, y, x, 0, 1)) {
        if signal > 0 {
            ctx.resonance_writes.push(ResonanceWrite {
                y,
                x,
                freq: freq as f32 * 10.0,
                amp: amp as f32 * 2.0,
            });
        }
    }
}

fn exec_increment(vm: &ChimeraVM, y: usize, x: usize, ctx: &mut SignalContext) {
    if let Some(n) = peek(vm, y, x, -1, 0) {
        let max = peek(vm, y, x, 0, 1).unwrap_or(35);
        let res = if n >= max { 0 } else { n + 1 };
        if let Some((sy, sx)) = vm.normalize_coords(y as i64 + 1, x as i64) {
            ctx.grid_writes.push(GridWrite {
                y: sy,
                x: sx,
                val: Value::Str(val_to_char(res).to_string()),
            });
        }
    }
}

fn exec_random(vm: &ChimeraVM, y: usize, x: usize, ctx: &mut SignalContext) {
    let min = peek(vm, y, x, -1, 0).unwrap_or(0);
    let max = peek(vm, y, x, 0, 1).unwrap_or(35);
    let mut rng = rand::thread_rng();
    let range_min = min.min(max);
    let range_max = min.max(max);
    let res = rng.gen_range(range_min..=range_max);
    if let Some((sy, sx)) = vm.normalize_coords(y as i64 + 1, x as i64) {
        ctx.grid_writes.push(GridWrite {
            y: sy,
            x: sx,
            val: Value::Str(val_to_char(res).to_string()),
        });
    }
}

fn process_phages(vm: &ChimeraVM, ctx: &mut SignalContext) {
    for (i, org) in vm.organelles.iter().enumerate() {
        if org.kind == crate::vm::nova::OrganelleType::Phage {
            let (y, x) = org.context_loc;
            let (dy, dx) = org.direction;

            // Normalize new position
            // Note: Using i64 for calculation, handling wrap around via normalize_coords
            // But Phages might bounce instead of wrap?
            // Let's bounce on boundary to keep them contained "in the dish".
            // Or wrap? Standard Orca wraps? normalize_coords wraps.
            // Let's try to move.

            let next_pos = vm.normalize_coords(y as i64 + dy as i64, x as i64 + dx as i64);

            if let Some((ny, nx)) = next_pos {
                let cell_val = &vm.grid[ny][nx];

                // Interaction Logic
                let mut new_dir = (dy, dx);
                let mut bounced = false;

                match cell_val {
                    Value::Str(s) => {
                        match s.as_str() {
                            "*" | "!" => {
                                // Mutation
                                ctx.mutation_requests.push(MutationRequest {
                                    strand_idx: org.ip.0,
                                });
                            }
                            "H" | "h" => {
                                // Host / Infection
                                ctx.phage_clones.push(PhageCloneRequest {
                                    strand_idx: org.ip.0,
                                });
                            }
                            "#" => {
                                // Wall - Bounce
                                new_dir = (-dy, -dx);
                                bounced = true;
                            }
                            _ => {}
                        }
                    }
                    Value::Int(n) => {
                        // Numeric collision?
                        // Maybe change direction based on number?
                        if *n == 0 {
                            // Empty space, continue
                        } else {
                            // Non-empty, bounce?
                            // Let's just pass through numbers for now.
                        }
                    }
                    _ => {}
                }

                // If bounced, we stay put (or move back? or just turn?)
                // If we bounce, we update dir but NOT loc this tick.
                if bounced {
                    ctx.phage_updates.push(PhageUpdate {
                        organelle_idx: i,
                        new_loc: (y, x),
                        new_dir,
                    });
                } else {
                    ctx.phage_updates.push(PhageUpdate {
                        organelle_idx: i,
                        new_loc: (ny, nx),
                        new_dir,
                    });
                }
            } else {
                // Out of bounds (if no wrap)
                // Reverse direction
                ctx.phage_updates.push(PhageUpdate {
                    organelle_idx: i,
                    new_loc: (y, x),
                    new_dir: (-dy, -dx),
                });
            }
        }
    }
}

fn exec_sigil(vm: &ChimeraVM, y: usize, x: usize, signal: u8, ctx: &mut SignalContext) {
    if signal == 0 {
        return;
    }
    // Iterate all registered sigils and check if patterns match at (y,x)
    for sigil in vm.sigil_registry.values() {
        if nova_sigil::check_dynamic_pattern(vm, y, x, &sigil.pattern) {
            // Found a match!
            // We should consume the pattern to prevent infinite loops if the sigil doesn't move/change.
            // But we can't easily queue consumption here because `pattern` is on the sigil.
            // We need to queue the consumption actions.
            // `consume_dynamic_pattern` modifies the grid directly, which we can't do here easily (we have immutable vm ref).
            // So we queue writes.

            for (dy, dx, _) in &sigil.pattern {
                if let Some((ny, nx)) = vm.normalize_coords(y as i64 + *dy, x as i64 + *dx) {
                    ctx.grid_writes.push(GridWrite {
                        y: ny,
                        x: nx,
                        val: Value::Int(0),
                    });
                }
            }

            // Execute the strand
            ctx.executions.push((
                OpCode::Call,
                vec![Nucleotide::Number(sigil.strand_idx as i64)],
            ));

            // Only trigger one per tick per §?
            break;
        }
    }
}

#[cfg(feature = "oracle")]
fn check_kb_operator(vm: &ChimeraVM, s: &str) -> Option<i64> {
    // Check KB for operator("Char", StrandID)
    // Value::Junction(Any, ["operator", "Char", StrandID])

    // We iterate the KB to find it.
    // Optimisation: We could have a cache, but for now linear scan.

    let op_key = Value::Str("operator".to_string());
    let char_key = Value::Str(s.to_string());

    for fact in &vm.knowledge_base {
        if let Value::Junction(JunctionType::Any, args) = fact {
            if args.len() == 3 && args[0] == op_key && args[1] == char_key {
                if let Value::Int(id) = args[2] {
                    return Some(id);
                }
            }
        }
    }
    None
}

fn exec_clock(vm: &ChimeraVM, y: usize, x: usize, ctx: &mut SignalContext) {
    let rate = peek(vm, y, x, 0, 1).unwrap_or(1).max(1);
    let mod_val = peek(vm, y, x, -1, 0).unwrap_or(8).max(1);
    let res = (vm.tick_counter as i64 / rate) % mod_val;
    if let Some((sy, sx)) = vm.normalize_coords(y as i64 + 1, x as i64) {
        ctx.grid_writes.push(GridWrite {
            y: sy,
            x: sx,
            val: Value::Str(val_to_char(res).to_string()),
        });
    }
}

fn exec_write(vm: &ChimeraVM, y: usize, x: usize, ctx: &mut SignalContext) {
    if let (Some(val), Some(x_off), Some(y_off)) = (
        peek(vm, y, x, -1, 0),
        peek(vm, y, x, 0, -1),
        peek(vm, y, x, 0, 1),
    ) {
        if let Some((ty, tx)) = vm.normalize_coords(y as i64 + y_off, x as i64 + x_off) {
            ctx.grid_writes.push(GridWrite {
                y: ty,
                x: tx,
                val: Value::Str(val_to_char(val).to_string()),
            });
        }
    }
}

fn exec_offset(vm: &ChimeraVM, y: usize, x: usize, ctx: &mut SignalContext) {
    if let (Some(x_off), Some(y_off)) = (peek(vm, y, x, 0, -1), peek(vm, y, x, 0, 1)) {
        if let Some(val) = peek(vm, y, x, y_off, x_off) {
            if let Some((sy, sx)) = vm.normalize_coords(y as i64 + 1, x as i64) {
                ctx.grid_writes.push(GridWrite {
                    y: sy,
                    x: sx,
                    val: Value::Str(val_to_char(val).to_string()),
                });
            }
        }
    }
}

fn exec_gene_read(vm: &ChimeraVM, y: usize, x: usize, ctx: &mut SignalContext) {
    if let (Some(s_idx), Some(g_idx)) = (peek(vm, y, x, 0, -1), peek(vm, y, x, 0, 1)) {
        let s = s_idx as usize;
        let g = g_idx as usize;
        if s < vm.dna.helix.strands.len() {
            let strand = &vm.dna.helix.strands[s];
            if g < strand.genes.len() {
                let op_str = strand.genes[g].op.to_string();
                if let Some((sy, sx)) = vm.normalize_coords(y as i64 + 1, x as i64) {
                    ctx.grid_writes.push(GridWrite {
                        y: sy,
                        x: sx,
                        val: Value::Str(op_str),
                    });
                }
            }
        }
    }
}

fn exec_play(vm: &ChimeraVM, y: usize, x: usize, signal: u8, ctx: &mut SignalContext) {
    if let Some(s_idx) = peek(vm, y, x, 0, -1) {
        if signal > 0 {
            ctx.executions
                .push((OpCode::Call, vec![Nucleotide::Number(s_idx)]));
        }
    }
}

fn exec_kill(vm: &ChimeraVM, y: usize, x: usize, signal: u8, ctx: &mut SignalContext) {
    if let Some(s_idx) = peek(vm, y, x, 0, -1) {
        if signal > 0 {
            ctx.executions
                .push((OpCode::Push, vec![Nucleotide::Number(s_idx)]));
            ctx.executions.push((OpCode::Apoptosis, vec![]));
        }
    }
}

fn exec_synthesize(vm: &ChimeraVM, y: usize, x: usize, signal: u8, ctx: &mut SignalContext) {
    if let (Some(s_idx), Some(g_idx), Some(val)) = (
        peek(vm, y, x, 0, -1),
        peek(vm, y, x, 0, 1),
        peek(vm, y, x, -1, 0),
    ) {
        if signal > 0 {
            ctx.dna_writes.push(DnaWrite {
                strand_idx: s_idx as usize,
                gene_idx: g_idx as usize,
                val: Nucleotide::Number(val),
            });
        }
    }
}

fn propagate_directional(
    vm: &ChimeraVM,
    y: usize,
    x: usize,
    dy: i64,
    dx: i64,
    signal: u8,
    next_signals: &mut [Vec<u8>],
) {
    if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
        next_signals[ny][nx] = next_signals[ny][nx].saturating_add(signal);
    }
}

fn binary_op<F>(vm: &ChimeraVM, y: usize, x: usize, grid_writes: &mut Vec<GridWrite>, op: F)
where
    F: Fn(i64, i64) -> i64,
{
    if let (Some(n), Some(e)) = (peek(vm, y, x, -1, 0), peek(vm, y, x, 0, 1)) {
        let res = op(n, e);
        if let Some((sy, sx)) = vm.normalize_coords(y as i64 + 1, x as i64) {
            grid_writes.push(GridWrite {
                y: sy,
                x: sx,
                val: Value::Str(val_to_char(res).to_string()),
            });
        }
    }
}

fn exec_flux(vm: &ChimeraVM, y: usize, x: usize, signal: u8, ctx: &mut SignalContext) {
    if let Some(amount) = peek(vm, y, x, 0, -1) {
        if signal > 0 {
            if let Some((sy, sx)) = vm.normalize_coords(y as i64 + 1, x as i64) {
                ctx.entropy_writes.push(EntropyWrite {
                    y: sy,
                    x: sx,
                    val: amount,
                });
            }
        }
    }
}

fn exec_jam(vm: &ChimeraVM, y: usize, x: usize, signal: u8, ctx: &mut SignalContext) {
    if let Some(amount) = peek(vm, y, x, 0, -1) {
        if signal > 0 {
            if let Some((sy, sx)) = vm.normalize_coords(y as i64 + 1, x as i64) {
                ctx.entropy_writes.push(EntropyWrite {
                    y: sy,
                    x: sx,
                    val: -amount,
                });
            }
        }
    }
}

fn read_write_directional(
    vm: &ChimeraVM,
    y: usize,
    x: usize,
    read_dy: i64,
    read_dx: i64,
    write_dy: i64,
    write_dx: i64,
    ctx: &mut SignalContext,
) {
    if let Some(val) = peek(vm, y, x, read_dy, read_dx) {
        if let Some((wy, wx)) = vm.normalize_coords(y as i64 + write_dy, x as i64 + write_dx) {
            ctx.grid_writes.push(GridWrite {
                y: wy,
                x: wx,
                val: Value::Str(val_to_char(val).to_string()),
            });
        }
    }
}

fn exec_ether_send(vm: &ChimeraVM, y: usize, x: usize, signal: u8, ctx: &mut SignalContext) {
    if signal == 0 {
        return;
    }
    // Inputs: North (Channel), East (Value)
    if let (Some(channel), Some(val)) = (peek(vm, y, x, -1, 0), peek(vm, y, x, 0, 1)) {
        ctx.ether_writes.push(EtherWrite {
            channel,
            val: Value::Str(val_to_char(val).to_string()),
        });
    }
}

fn exec_ether_recv(vm: &ChimeraVM, y: usize, x: usize, signal: u8, ctx: &mut SignalContext) {
    if signal == 0 {
        return;
    }
    // Input: North (Channel)
    if let Some(channel) = peek(vm, y, x, -1, 0) {
        if let Some((sy, sx)) = vm.normalize_coords(y as i64 + 1, x as i64) {
            ctx.ether_reads.push(EtherRead {
                channel,
                y: sy,
                x: sx,
            });
        }
    }
}

fn exec_project_signal(_vm: &ChimeraVM, _y: usize, _x: usize, signal: u8, ctx: &mut SignalContext) {
    if signal > 0 {
        ctx.executions.push((OpCode::Project, vec![]));
    }
}

fn exec_wave(vm: &ChimeraVM, y: usize, x: usize, signal: u8, ctx: &mut SignalContext) {
    if signal == 0 {
        return;
    }
    // ~: Wave Operator (Hologram Interface)
    // North: Mode (0=ReadRe, 1=ReadIm, 2=WriteRe, 3=WriteIm)
    // East: Value (for Write, scaled by 100)
    // South: Output (for Read, scaled by 100)

    let mode = peek(vm, y, x, -1, 0).unwrap_or(0);

    match mode {
        0 => {
            // Read Re
            let val = vm.hologram_grid[y][x].0 * 100.0;
            if let Some((sy, sx)) = vm.normalize_coords(y as i64 + 1, x as i64) {
                ctx.grid_writes.push(GridWrite {
                    y: sy,
                    x: sx,
                    val: Value::Str(val_to_char(val as i64).to_string()),
                });
            }
        }
        1 => {
            // Read Im
            let val = vm.hologram_grid[y][x].1 * 100.0;
            if let Some((sy, sx)) = vm.normalize_coords(y as i64 + 1, x as i64) {
                ctx.grid_writes.push(GridWrite {
                    y: sy,
                    x: sx,
                    val: Value::Str(val_to_char(val as i64).to_string()),
                });
            }
        }
        2 => {
            // Write Re
            if let Some(val) = peek(vm, y, x, 0, 1) {
                ctx.holo_writes.push(HoloWrite {
                    y,
                    x,
                    re: Some(val as f64 / 100.0),
                    im: None,
                });
            }
        }
        3 => {
            // Write Im
            if let Some(val) = peek(vm, y, x, 0, 1) {
                ctx.holo_writes.push(HoloWrite {
                    y,
                    x,
                    re: None,
                    im: Some(val as f64 / 100.0),
                });
            }
        }
        _ => {}
    }
}

fn exec_psi(vm: &ChimeraVM, y: usize, x: usize, _signal: u8, ctx: &mut SignalContext) {
    // Ψ: Quantum Observer
    // North: Target Magnitude (scaled by 100)
    // If local hologram magnitude matches target, bang all neighbors.

    let target = peek(vm, y, x, -1, 0).unwrap_or(0);

    // Calculate local magnitude
    let (re, im) = vm.hologram_grid[y][x];
    let mag: f64 = (re * re + im * im).sqrt() * 100.0;

    if (mag as i64 - target).abs() <= 5 {
        // Match! Fire bang.
        let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dy, dx) in neighbors {
            if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
                ctx.next_signals[ny][nx] = ctx.next_signals[ny][nx].saturating_add(1);
            }
        }
    }
}

fn exec_phi(vm: &ChimeraVM, y: usize, x: usize, _signal: u8, ctx: &mut SignalContext) {
    // Φ: Golden Ratio Resonator
    // Checks neighbors: N + E == S ?

    if let (Some(n), Some(e), Some(s)) = (
        peek(vm, y, x, -1, 0),
        peek(vm, y, x, 0, 1),
        peek(vm, y, x, 1, 0),
    ) {
        if n + e == s {
            // Resonance! Gain Energy.
            ctx.executions
                .push((OpCode::Push, vec![Nucleotide::Number(10)]));
            ctx.executions.push((OpCode::Consume, vec![]));
        }
    }
}

fn exec_omega(vm: &ChimeraVM, y: usize, x: usize, _signal: u8, ctx: &mut SignalContext) {
    // Ω: Entropy Sink
    // Passive:
    // Reduces local entropy.
    // Increases hologram complexity.

    let entropy = vm.entropy_grid[y][x];
    if entropy > 0 {
        // Reduce entropy by 10
        ctx.entropy_writes.push(EntropyWrite { y, x, val: -10 });

        // Increase Hologram Phase/Complexity
        let (re, im) = vm.hologram_grid[y][x];
        ctx.holo_writes.push(HoloWrite {
            y,
            x,
            re: Some(re + 0.1),
            im: Some(im - 0.1),
        });
    }
}
