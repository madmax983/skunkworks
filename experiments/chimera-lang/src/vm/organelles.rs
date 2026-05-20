use std::io::Read;
use crate::opcode::OpCode;
use crate::value::Value;
#[cfg(feature = "nova")]
use crate::vm::nova::Organelle;
#[cfg(feature = "nova")]
use crate::vm::{
    alchemy, nova, nova_botany, nova_flux, nova_metazoa, nova_savant, nova_ward,
    MAX_CALL_STACK_DEPTH, MAX_CHORUS_SIZE, MAX_ORGANELLES,
};
#[cfg(feature = "nova")]
use rand::Rng;

impl crate::vm::ChimeraVM {
    #[cfg(feature = "nova")]
    pub(crate) fn process_organelles(&mut self) {
        let active_organelles = std::mem::take(&mut self.organelles);
        let mut next_organelles = Vec::new();

        for mut organelle in active_organelles {
            let (cy, cx) = organelle.context_loc;
            let dilation = self.time_grid[cy][cx];

            // 0 = Stasis (Skip tick)
            if dilation == 0 {
                next_organelles.push(organelle);
                continue;
            }

            let ticks = dilation as usize;
            let mut keep = true;
            for _ in 0..ticks {
                if !self.tick_organelle(&mut organelle) {
                    keep = false;
                    break;
                }
            }
            if keep {
                next_organelles.push(organelle);
            }
        }
        self.organelles.extend(next_organelles);
    }

    #[cfg(feature = "nova")]
    pub(crate) fn tick_organelle(&mut self, organelle: &mut Organelle) -> bool {
        if organelle.halted {
            return false;
        }

        std::mem::swap(&mut self.stack, &mut organelle.stack);
        std::mem::swap(&mut self.ip, &mut organelle.ip);
        std::mem::swap(&mut self.context_loc, &mut organelle.context_loc);
        std::mem::swap(&mut self.call_stack, &mut organelle.call_stack);
        std::mem::swap(&mut self.recursion_depth, &mut organelle.recursion_depth);

        self.active_organelle_kind = Some(organelle.kind.clone());
        self.energy = self.energy.saturating_sub(1);

        match organelle.kind {
            nova::OrganelleType::Chloroplast => {
                let (cy, cx) = self.context_loc;
                let light = self.light_grid[cy][cx];
                if light > 0 {
                    self.energy = self.energy.saturating_add(light / 10);
                }
            }
            nova::OrganelleType::Mitochondria => {
                self.energy = self.energy.saturating_add(1);
            }
            nova::OrganelleType::Lysosome => {
                let (cy, cx) = self.context_loc;
                let waste = self.waste_grid[cy][cx];
                if waste > 0 {
                    let consumed = waste.min(10);
                    self.waste_grid[cy][cx] -= consumed;
                    self.energy = self.energy.saturating_add(consumed / 5);
                }
            }
            nova::OrganelleType::Ribosome => {
                self.process_ribosome(organelle);
            }
            nova::OrganelleType::Void => {
                self.process_void_organelle(organelle);
            }
            nova::OrganelleType::Alchemist => {
                let (cy, cx) = self.context_loc;
                if alchemy::perform_alchemy(self, cy, cx) {
                    self.energy = self.energy.saturating_sub(5);
                }

                // Brownian Motion
                let mut rng = rand::thread_rng();
                let dy = rng.gen_range(-1..=1);
                let dx = rng.gen_range(-1..=1);
                organelle.direction = (dy, dx);
            }
            nova::OrganelleType::Seed => {
                if !nova_botany::tick_seed(self, organelle) {
                    organelle.halted = true;
                }
            }
            nova::OrganelleType::Wisp => {
                if !nova_flux::tick_wisp(self, organelle) {
                    organelle.halted = true;
                }
            }
            nova::OrganelleType::Choir => {
                self.process_choir_organelle(organelle);
            }
            nova::OrganelleType::MadScientist => {
                self.process_mad_scientist(organelle);
            }
            nova::OrganelleType::Phage => {
                // Phages are processed in nova_signals::process_signals
            }
            nova::OrganelleType::Savant => {
                nova_savant::process_savant(self, organelle);
            }
            nova::OrganelleType::Metazoan => {
                nova_metazoa::process_metazoan(self, organelle);
            }
            nova::OrganelleType::Worker => {}
        }

        if !matches!(
            organelle.kind,
            nova::OrganelleType::Ribosome
                | nova::OrganelleType::Void
                | nova::OrganelleType::Alchemist
                | nova::OrganelleType::Seed
                | nova::OrganelleType::Choir
                | nova::OrganelleType::MadScientist
                | nova::OrganelleType::Phage
                | nova::OrganelleType::Savant
        ) {
            self.execute_organelle_dna(organelle);
        }

        if let Some(new_kind) = self.signal_differentiation.take() {
            organelle.kind = new_kind;
        }

        std::mem::swap(&mut self.stack, &mut organelle.stack);
        std::mem::swap(&mut self.ip, &mut organelle.ip);
        std::mem::swap(&mut self.context_loc, &mut organelle.context_loc);
        std::mem::swap(&mut self.call_stack, &mut organelle.call_stack);
        std::mem::swap(&mut self.recursion_depth, &mut organelle.recursion_depth);

        if let Some(ttl) = organelle.ttl {
            if ttl <= 1 {
                return false;
            }
            organelle.ttl = Some(ttl - 1);
        }

        !organelle.halted
    }

    #[cfg(feature = "nova")]
    fn process_mad_scientist(&mut self, organelle: &mut Organelle) {
        let (cy, cx) = self.context_loc;
        let mut rng = rand::thread_rng();

        // 10% chance to do science
        if rng.gen_bool(0.1) {
            let experiment = rng.gen_range(0..4);
            match experiment {
                0 => {
                    // Irradiate
                    self.mutagen_grid[cy][cx] = self.mutagen_grid[cy][cx].saturating_add(50);
                    self.output
                        .push(format!("MAD SCIENTIST: Irradiated {},{}", cx, cy));
                }
                1 => {
                    // Alchemy
                    if alchemy::perform_alchemy(self, cy, cx) {
                        self.output
                            .push(format!("MAD SCIENTIST: Transmuted {},{}", cx, cy));
                    } else {
                        self.output
                            .push(format!("MAD SCIENTIST: Failed alchemy at {},{}", cx, cy));
                    }
                }
                2 => {
                    // Entropy Surge
                    if nova_flux::exec_entropy_surge(self).is_some() {
                        self.output
                            .push("MAD SCIENTIST: Triggered ENTROPY SURGE!".to_string());
                    }
                }
                3 => {
                    // Spawn
                    if self.organelles.len() < MAX_ORGANELLES {
                        self.organelle_id_counter += 1;
                        let new_org = Organelle {
                            stack: Vec::new(),
                            ip: (0, 0),
                            context_loc: (cy, cx),
                            call_stack: Vec::new(),
                            recursion_depth: 0,
                            halted: false,
                            kind: nova::OrganelleType::Worker,
                            direction: (0, 0),
                            ttl: None,
                            name: "Igor".to_string(),
                            traits: vec!["Assistant".to_string()],
                            id: self.organelle_id_counter,
                            tissue_id: None,
                            genome_id: 0,
                            energy: 10,
                            experience: 0,
                            stage: 0,
                        };
                        self.organelles.push(new_org);
                        self.output
                            .push(format!("MAD SCIENTIST: Created life at {},{}", cx, cy));
                    }
                }
                _ => {}
            }
            self.energy = self.energy.saturating_sub(10);
        }

        // Brownian Motion
        let dy = rng.gen_range(-1..=1);
        let dx = rng.gen_range(-1..=1);
        organelle.direction = (dy, dx);
    }

    #[cfg(feature = "nova")]
    fn process_void_organelle(&mut self, organelle: &mut Organelle) {
        let (cy, cx) = self.context_loc;

        // Entropy Trail
        self.entropy_grid[cy][cx] = self.entropy_grid[cy][cx].saturating_add(10).min(100);

        // Void consumes grid cell if not 0
        let val = self.grid[cy][cx].clone();
        if !matches!(val, Value::Int(0)) {
            self.grid[cy][cx] = Value::Int(0);
            self.energy = self.energy.saturating_add(1);
            self.output.push(format!("VOID: Consumed at {},{}", cx, cy));

            // Void Song: Check for elemental strings
            if let Value::Str(s) = val {
                let note = match s.as_str() {
                    "Fire" => Some("Do"),
                    "Water" => Some("Re"),
                    "Earth" => Some("Mi"),
                    "Air" => Some("Fa"),
                    "Spirit" => Some("Sol"),
                    _ => None,
                };

                if let Some(n) = note {
                    self.chorus_buffer.push_back(n.to_string());
                    if self.chorus_buffer.len() > MAX_CHORUS_SIZE {
                        self.chorus_buffer.pop_front();
                    }
                    self.output.push(format!("VOID SONG: {}", n));
                    if let Some(_target) = nova::check_chorus_chords(self) {
                        // Void ignores calls, but maybe we can trigger global effect?
                        // For now, ignore jump for Void.
                    }
                }
            }
        }

        // Brownian Motion
        let mut rng = rand::thread_rng();
        let dy = rng.gen_range(-1..=1);
        let dx = rng.gen_range(-1..=1);
        organelle.direction = (dy, dx);
    }

    #[cfg(feature = "nova")]
    fn process_choir_organelle(&mut self, organelle: &mut Organelle) {
        let song_len = organelle.traits.len();
        if song_len > 0 {
            let idx = organelle.recursion_depth % song_len;
            let note = &organelle.traits[idx];

            self.chorus_buffer.push_back(note.clone());
            if self.chorus_buffer.len() > MAX_CHORUS_SIZE {
                self.chorus_buffer.pop_front();
            }
            self.output.push(format!("CHOIR: {}", note));

            if let Some(target) = nova::check_chorus_chords(self) {
                // Choir triggers HOST to jump
                // self.ip is the HOST IP context (because we swapped)
                // wait, tick_organelle SWAPPED self.ip with organelle.ip.
                // So self.ip is ORGANELLE IP.
                // organelle.ip is HOST IP.

                // We want to update HOST IP.
                // So we update organelle.ip.

                if organelle.call_stack.len() < MAX_CALL_STACK_DEPTH {
                    organelle.call_stack.push(organelle.ip); // Save old Host IP
                    organelle.ip = (target, 0); // Jump Host to target
                    self.output
                        .push(format!("CHOIR: Triggered host jump to {}", target));
                }
            }

            organelle.recursion_depth = (organelle.recursion_depth + 1) % song_len;
            self.energy = self.energy.saturating_sub(1);
        }
    }

    #[cfg(feature = "nova")]
    fn ribosome_binary_op<F>(&mut self, op: F)
    where
        F: Fn(i64, i64) -> Option<i64>,
    {
        if self.stack.len() >= 2 {
            let b = self.stack.pop().unwrap();
            let a = self.stack.pop().unwrap();
            if let (Value::Int(ia), Value::Int(ib)) = (a, b) {
                if let Some(res) = op(ia, ib) {
                    self.stack.push(Value::Int(res));
                }
            }
        }
    }

    #[cfg(feature = "nova")]
    fn process_ribosome_bang(&mut self, cy: usize, cx: usize) {
        // Bang: Trigger all neighbors
        let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dy, dx) in neighbors {
            if let Some((ny, nx)) = self.normalize_coords(cy as i64 + dy, cx as i64 + dx) {
                let mask = match (dy, dx) {
                    (-1, 0) => 1,
                    (1, 0) => 2,
                    (0, 1) => 4,
                    (0, -1) => 8,
                    _ => 0,
                };
                if (self.membranes[cy][cx] & mask) == 0 {
                    // Spawn ephemeral Ribosome
                    if self.organelles.len() < MAX_ORGANELLES {
                        self.organelle_id_counter += 1;
                        let new_org = Organelle {
                            stack: Vec::new(),
                            ip: (0, 0),
                            context_loc: (ny, nx),
                            call_stack: Vec::new(),
                            recursion_depth: 0,
                            halted: false,
                            kind: nova::OrganelleType::Ribosome,
                            direction: (dy as i8, dx as i8),
                            ttl: Some(1),
                            name: "Spark".to_string(),
                            traits: vec!["Ephemeral".to_string()],
                            id: self.organelle_id_counter,
                            tissue_id: None,
                            genome_id: 0,
                            energy: 10,
                            experience: 0,
                            stage: 0,
                        };
                        self.organelles.push(new_org);
                    } else {
                        self.output
                            .push("Error: Organelle limit exceeded in Bang".to_string());
                    }
                }
            }
        }
    }

    #[cfg(feature = "nova")]
    fn process_ribosome_read(&mut self, cy: usize, cx: usize, push_zero_on_fail: bool) {
        // Offset Read: [dy, dx] -> [val]
        if self.stack.len() >= 2 {
            let x_off = self.stack.pop().unwrap();
            let y_off = self.stack.pop().unwrap();
            if let (Value::Int(dx), Value::Int(dy)) = (x_off, y_off) {
                if let Some((ny, nx)) = self.normalize_coords(cy as i64 + dy, cx as i64 + dx) {
                    self.stack.push(self.grid[ny][nx].clone());
                } else if push_zero_on_fail {
                    self.stack.push(Value::Int(0));
                }
            }
        }
    }

    #[cfg(feature = "nova")]
    fn process_ribosome_write(&mut self, cy: usize, cx: usize) {
        // Offset Write: [val, dy, dx] -> []
        if self.stack.len() >= 3 {
            let x_off = self.stack.pop().unwrap();
            let y_off = self.stack.pop().unwrap();
            let val = self.stack.pop().unwrap();
            if let (Value::Int(dx), Value::Int(dy)) = (x_off, y_off) {
                if let Some((ny, nx)) = self.normalize_coords(cy as i64 + dy, cx as i64 + dx) {
                    self.grid[ny][nx] = val;
                }
            }
        }
    }

    #[cfg(feature = "nova")]
    fn process_ribosome(&mut self, organelle: &mut Organelle) {
        let (cy, cx) = self.context_loc;
        let val = self.grid[cy][cx].clone();

        self.exec_ribosome_command(organelle, val, cy, cx);
        self.update_ribosome_position(organelle, cy, cx);
    }

    #[cfg(feature = "nova")]
    fn exec_ribosome_command(
        &mut self,
        organelle: &mut Organelle,
        val: Value,
        cy: usize,
        cx: usize,
    ) {
        match val {
            Value::Int(n) => self.stack.push(Value::Int(n)),
            Value::Junction(t, vals) => self.stack.push(Value::Junction(t, vals)),
            Value::Superposition(s) => self.stack.push(Value::Superposition(s)),
            Value::Symbol(id) => self.stack.push(Value::Symbol(id)),
            Value::Color(r, g, b) => self.stack.push(Value::Color(r, g, b)),
            Value::Str(s) => match s.as_str() {
                ">" => organelle.direction = (0, 1),
                "<" => organelle.direction = (0, -1),
                "^" => organelle.direction = (-1, 0),
                "v" => organelle.direction = (1, 0),
                "+" => self.ribosome_binary_op(|a, b| Some(a.wrapping_add(b))),
                "-" => self.ribosome_binary_op(|a, b| Some(a.wrapping_sub(b))),
                "*" => self.process_ribosome_bang(cy, cx),
                "o" => self.process_ribosome_read(cy, cx, true),
                "x" => self.process_ribosome_write(cy, cx),
                "?" => {
                    let mut rng = rand::thread_rng();
                    self.stack.push(Value::Int(rng.gen_range(0..10)));
                }
                "mul" => {
                    self.ribosome_binary_op(|a, b| Some(a.wrapping_mul(b)));
                }
                "/" => self.ribosome_binary_op(|a, b| {
                    if b != 0 {
                        Some(a.wrapping_div(b))
                    } else {
                        None
                    }
                }),
                "%" => self.ribosome_binary_op(|a, b| {
                    if b != 0 {
                        Some(a.wrapping_rem(b))
                    } else {
                        None
                    }
                }),
                "=" => {
                    if self.stack.len() >= 2 {
                        let b = self.stack.pop().unwrap();
                        let a = self.stack.pop().unwrap();
                        if a == b {
                            self.stack.push(Value::Int(1));
                        } else {
                            self.stack.push(Value::Int(0));
                        }
                    }
                }
                "!" => {
                    if let Some(Value::Int(i)) = self.stack.pop() {
                        self.stack.push(Value::Int(if i == 0 { 1 } else { 0 }));
                    }
                }
                ":" => self.process_ribosome_read(cy, cx, false),
                ";" => self.process_ribosome_write(cy, cx),
                _ => {
                    if let Ok(op) = s.parse::<OpCode>() {
                        if let Some(target) = self.execute_gene_inner(op, &[]) {
                            self.ip = target;
                        }
                    }
                }
            },
        }
    }

    #[cfg(feature = "nova")]
    fn update_ribosome_position(&mut self, organelle: &mut Organelle, cy: usize, cx: usize) {
        let (dy, dx) = organelle.direction;
        if let Some((mut new_y, mut new_x)) =
            self.normalize_coords(cy as i64 + dy as i64, cx as i64 + dx as i64)
        {
            let mask = match (dy, dx) {
                (-1, 0) => 1,
                (1, 0) => 2,
                (0, 1) => 4,
                (0, -1) => 8,
                _ => 0,
            };
            if (self.membranes[cy][cx] & mask) == 0 {
                if let Some(&(py, px)) = self.portals.get(&(new_y, new_x)) {
                    self.output.push(format!(
                        "PORTAL: Teleported from {},{} to {},{}",
                        new_x, new_y, px, py
                    ));
                    new_y = py;
                    new_x = px;
                }
                self.context_loc = (new_y, new_x);
                if let Some(target) = nova_ward::check_ward_trigger(self) {
                    self.ip = target;
                }
            }
        }
    }

    #[cfg(feature = "nova")]
    fn execute_organelle_dna(&mut self, organelle: &mut Organelle) {
        if self.energy > 0 && self.ip.0 < self.dna.helix.strands.len() {
            let strand_len = self.dna.helix.strands[self.ip.0].genes.len();
            if self.ip.1 < strand_len {
                let (gene_op, gene_args) = {
                    let gene = &self.dna.helix.strands[self.ip.0].genes[self.ip.1];
                    (gene.op.clone(), gene.args.clone())
                };

                let jump_target = self.execute_gene(gene_op, &gene_args);

                if let Some(target) = jump_target {
                    self.ip = target;
                } else {
                    self.ip.1 += 1;
                }
            } else {
                organelle.halted = true;
            }
        } else {
            organelle.halted = true;
        }
    }
}
