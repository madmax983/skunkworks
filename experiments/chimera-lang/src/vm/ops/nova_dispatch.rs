use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use crate::value::Value;

#[cfg(feature = "nova")]
use crate::vm::akashic;
#[cfg(feature = "nova")]
use crate::vm::alchemy;
#[cfg(feature = "nova")]
use crate::vm::babel;
#[cfg(feature = "nova")]
use crate::vm::babel_chaos;
#[cfg(feature = "nova")]
use crate::vm::bard;
#[cfg(feature = "nova")]
use crate::vm::codex;
#[cfg(feature = "nova")]
use crate::vm::evolution;
#[cfg(feature = "nova")]
use crate::vm::meta;
#[cfg(feature = "nova")]
use crate::vm::nova;
#[cfg(feature = "nova")]
use crate::vm::nova_alchemy_prime;
#[cfg(feature = "nova")]
use crate::vm::nova_botany;
#[cfg(feature = "nova")]
use crate::vm::nova_cambrian;
#[cfg(feature = "nova")]
use crate::vm::nova_cartography;
#[cfg(feature = "nova")]
use crate::vm::nova_crystal;
#[cfg(feature = "nova")]
use crate::vm::nova_flux;
#[cfg(feature = "nova")]
use crate::vm::nova_functional;
#[cfg(feature = "nova")]
use crate::vm::nova_genetics;
#[cfg(feature = "nova")]
use crate::vm::nova_geology;
#[cfg(feature = "nova")]
use crate::vm::nova_guild;
#[cfg(feature = "nova")]
use crate::vm::nova_ley;
#[cfg(feature = "nova")]
use crate::vm::nova_metazoa;
#[cfg(feature = "nova")]
use crate::vm::nova_morphogenesis;
#[cfg(feature = "nova")]
use crate::vm::nova_planes;
#[cfg(feature = "nova")]
use crate::vm::nova_raku;
#[cfg(feature = "nova")]
use crate::vm::nova_security;
#[cfg(feature = "nova")]
use crate::vm::nova_sigil;
#[cfg(feature = "nova")]
use crate::vm::nova_strings;
#[cfg(feature = "nova")]
use crate::vm::nova_ward;
#[cfg(feature = "nova")]
use crate::vm::nova_weaver;

impl crate::vm::ChimeraVM {
    #[cfg(feature = "nova")]
    fn apply_simulation_log(&mut self, msg: &str) -> crate::vm::ops::Dispatch {
        self.output.push(msg.to_string());
        crate::vm::ops::Dispatch::Jump(self.ip.0, self.ip.1 + 1)
    }

    #[cfg(feature = "nova")]
    pub(crate) fn exec_nova_dispatch(
        &mut self,
        op: OpCode,
        args: &[Nucleotide],
    ) -> crate::vm::ops::Dispatch {
        match op {
            OpCode::Remap | OpCode::Restore | OpCode::Mirror => self.exec_prion_op(op, args).into(),
            OpCode::AkashicWrite
            | OpCode::AkashicRead
            | OpCode::AkashicSave
            | OpCode::AkashicLoad
            | OpCode::Karma
            | OpCode::Miracle => {
                akashic::exec_akashic_op(self, op, args);
                crate::vm::ops::Dispatch::Handled
            }
            OpCode::Blackbox => {
                let dump = self.blackbox.dump();
                self.stack.push(Value::Str(dump));
                crate::vm::ops::Dispatch::Handled
            }
            OpCode::Invoke => nova_sigil::exec_invoke(self, op, args).into(),
            OpCode::Inscribe => nova_sigil::exec_inscribe(self, op, args).into(),
            OpCode::Ward => nova_ward::exec_ward(self, op, args).into(),
            OpCode::AutoCast => nova_sigil::exec_auto_cast(self, op, args).into(),
            OpCode::Vaccinate | OpCode::Verify | OpCode::Audit => {
                nova_security::exec_security_op(self, op, args).into()
            }
            OpCode::Morph => {
                nova_morphogenesis::exec_morph(self);
                crate::vm::ops::Dispatch::Handled
            }
            OpCode::Morphogen => nova_cambrian::exec_morphogen(self, op, args).into(),
            OpCode::HoxSwitch => nova_cambrian::exec_hox_switch(self, op, args).into(),
            OpCode::Adhere => nova_cambrian::exec_adhere(self, op, args).into(),
            OpCode::Grow => {
                nova_morphogenesis::exec_grow(self);
                crate::vm::ops::Dispatch::Handled
            }
            OpCode::Plant => {
                nova_botany::exec_plant(self);
                crate::vm::ops::Dispatch::Handled
            }
            OpCode::Signal | OpCode::Receive => nova::exec_nova_op(self, op, args).into(),
            OpCode::Define | OpCode::Undefine | OpCode::Dictionary => {
                meta::exec_meta_op(self, op, args).into()
            }
            OpCode::Operator => nova::exec_operator(self, args).into(),
            OpCode::Grammar
            | OpCode::Parse
            | OpCode::ParserMatch
            | OpCode::ParserRegex
            | OpCode::ParserSeq
            | OpCode::ParserAlt
            | OpCode::ParserMany
            | OpCode::ParserOpt
            | OpCode::ParserSeqN
            | OpCode::ParserAltN
            | OpCode::Tongue
            | OpCode::Generate
            | OpCode::Scribe
            | OpCode::BabelCompile
            | OpCode::GridGrammar
            | OpCode::BabelLive
            | OpCode::DefineRule
            | OpCode::Ouroboros => babel::exec_babel_op(self, op, args).into(),
            OpCode::Superpose
            | OpCode::Collapse
            | OpCode::Observe
            | OpCode::Interfere
            | OpCode::Project
            | OpCode::Refract => nova::exec_nova_op(self, op, args).into(),
            OpCode::Levenshtein
            | OpCode::Soundex
            | OpCode::Anagram
            | OpCode::Cipher
            | OpCode::Pangram => nova::exec_nova_op(self, op, args).into(),
            OpCode::Transposon => self.exec_transposon().into(),
            OpCode::Horcrux
            | OpCode::Rebirth
            | OpCode::Resonate
            | OpCode::SonicClaim
            | OpCode::Dampen
            | OpCode::ListenFreq
            | OpCode::ChronosSplice
            | OpCode::Emit
            | OpCode::Smell
            | OpCode::Track
            | OpCode::Fire
            | OpCode::Salvo
            | OpCode::Reflector
            | OpCode::Prism
            | OpCode::Lens
            | OpCode::Claim
            | OpCode::Cede
            | OpCode::Sovereignty
            | OpCode::Tax
            | OpCode::Offer
            | OpCode::Buy
            | OpCode::Invest
            | OpCode::Divest
            | OpCode::Balance
            | OpCode::Ticker
            | OpCode::Splice
            | OpCode::Frankenstein
            | OpCode::Relativity
            | OpCode::Graviton
            | OpCode::EventHorizon
            | OpCode::Aeolus
            | OpCode::Storm
            | OpCode::SenseWind
            | OpCode::SenseMoisture
            | OpCode::QuantumJump
            | OpCode::Isomerize
            | OpCode::Spirit
            | OpCode::Alchemy
            | OpCode::Meme
            | OpCode::Conceive
            | OpCode::Propagate
            | OpCode::Forget
            | OpCode::Shibboleth
            | OpCode::Infect
            | OpCode::Outbreak
            | OpCode::Sanitize
            | OpCode::Drift
            | OpCode::Poly
            | OpCode::Chronostasis
            | OpCode::Prophecy
            | OpCode::Sing
            | OpCode::Listen
            | OpCode::Hyphae
            | OpCode::Connect
            | OpCode::Transport
            | OpCode::SporeCloud
            | OpCode::Brainfuck
            | OpCode::TuiDraw
            | OpCode::MosaicDraw
            | OpCode::Irradiate
            | OpCode::SenseMutagen
            | OpCode::Devour
            | OpCode::Evolve
            | OpCode::Glitch
            | OpCode::Scramble
            | OpCode::Metamorphosis
            | OpCode::Pigment
            | OpCode::Glyph
            | OpCode::SensePigment
            | OpCode::SenseGlyph
            | OpCode::Rift
            | OpCode::Seal
            | OpCode::Shape
            | OpCode::Void
            | OpCode::Supernova
            | OpCode::Singularity
            | OpCode::Simulate
            | OpCode::Dream
            | OpCode::Lucid
            | OpCode::Chemotaxis
            | OpCode::Identity
            | OpCode::Differentiate
            | OpCode::Sporulate
            | OpCode::TimeLoop
            | OpCode::Germinate
            | OpCode::Paradox
            | OpCode::Spawn
            | OpCode::Incubate
            | OpCode::Methylate
            | OpCode::Demethylate
            | OpCode::Telomerase
            | OpCode::TLen
            | OpCode::Recombine
            | OpCode::SIndex
            | OpCode::CrisprScan
            | OpCode::Cas9Cut
            | OpCode::Ligase
            | OpCode::Mitosis
            | OpCode::Apoptosis
            | OpCode::Integrase
            | OpCode::Excision
            | OpCode::Secrete
            | OpCode::Detect
            | OpCode::Absorb
            | OpCode::Migrate
            | OpCode::Detox
            | OpCode::WRead
            | OpCode::Call
            | OpCode::Exec
            | OpCode::Ret
            | OpCode::Bind
            | OpCode::Unbind
            | OpCode::Entangle
            | OpCode::Decohere
            | OpCode::Conjugate
            | OpCode::Gravitate
            | OpCode::Lumine
            | OpCode::SenseLight
            | OpCode::Broadcast
            | OpCode::Tune
            | OpCode::PhaseShift
            | OpCode::Membrane
            | OpCode::Osmosis
            | OpCode::Symbiosis
            | OpCode::Lysis
            | OpCode::Reflex
            | OpCode::Compile
            | OpCode::Decompile
            | OpCode::Sonar
            | OpCode::Eval
            | OpCode::LispEval
            | OpCode::Map
            | OpCode::Fold
            | OpCode::Filter
            | OpCode::Zip
            | OpCode::Match
            | OpCode::Bury
            | OpCode::Exhume
            | OpCode::Seance
            | OpCode::Mourn
            | OpCode::TimeWarp
            | OpCode::Chronos
            | OpCode::Retroscope
            | OpCode::Reincarnate
            | OpCode::Piet
            | OpCode::Befunge
            | OpCode::Origami
            | OpCode::Terraform
            | OpCode::SenseBiome
            | OpCode::RetinaDraw
            | OpCode::RetinaClear
            | OpCode::RetinaSize
            | OpCode::Scanline
            | OpCode::Rasterize
            | OpCode::EgregoreLink
            | OpCode::EgregoreTithe
            | OpCode::EgregoreChannel
            | OpCode::EgregoreDictate
            | OpCode::EgregoreQuery
            | OpCode::EgregoreSummon
            | OpCode::Sacrifice
            | OpCode::Entropy
            | OpCode::Stabilize
            | OpCode::Disintegrate
            | OpCode::Tsunami
            | OpCode::Dry
            | OpCode::Harmonize
            | OpCode::Choir
            | OpCode::Mix
            | OpCode::Brew
            | OpCode::Splash
            | OpCode::Fossilize
            | OpCode::Unearth
            | OpCode::CarbonDate
            | OpCode::Logistics
            | OpCode::Sow
            | OpCode::Harvest
            | OpCode::Draw
            | OpCode::Fate
            | OpCode::Shuffle
            | OpCode::Knot
            | OpCode::Unknot
            | OpCode::Cord
            | OpCode::ReadCord
            | OpCode::Tangle
            | OpCode::Quipu
            | OpCode::Pray
            | OpCode::Genesis
            | OpCode::Retrograde
            | OpCode::Synthesize
            | OpCode::Catalyze
            | OpCode::VoidRift
            | OpCode::VoidCast
            | OpCode::Chaos
            | OpCode::TuiMod
            | OpCode::Cambrian
            | OpCode::Prologue
            | OpCode::PrologueEsolang
            | OpCode::GlitchArt
            | OpCode::OrcaWeaver
            | OpCode::ElektraWeaver
            | OpCode::PrologWeaver
            | OpCode::Rune
            | OpCode::BioHack
            | OpCode::SelfReplicate
            | OpCode::Forge
            | OpCode::Speak
            | OpCode::Mandelbrot
            | OpCode::Julia
            | OpCode::Zoom
            | OpCode::Pan
            | OpCode::Iterate
            | OpCode::Escape
            | OpCode::QuantumScribe
            | OpCode::QuantumScan
            | OpCode::HoloInvoke
            | OpCode::HoloSpeak
            | OpCode::Luciferin
            | OpCode::Photophore
            | OpCode::Etymology => nova::exec_nova_op(self, op, args).into(),
            OpCode::EvoPopSize
            | OpCode::EvoLoad
            | OpCode::EvoStore
            | OpCode::EvoScore
            | OpCode::EvoBreed
            | OpCode::EvoMutate
            | OpCode::EvoReplace
            | OpCode::EvoClear
            | OpCode::EvoSave => evolution::exec_evo_op(self, op, args).into(),
            #[cfg(feature = "nova")]
            OpCode::Codex => {
                if let Some(Value::Int(id)) = self.stack.pop() {
                    if let Some(spell) = self.codex.get_spell(id as usize) {
                        codex::exec_spell(self, &spell);
                    } else {
                        self.output
                            .push(format!("Error: Invalid Codex spell ID {}", id));
                    }
                } else {
                    self.output
                        .push("Error: Codex requires spell ID (Int)".to_string());
                }
                crate::vm::ops::Dispatch::Handled
            }
            OpCode::Fluid => crate::vm::ops::Dispatch::Handled,
            OpCode::Weave | OpCode::Unravel => nova_weaver::exec_weave_op(self, op, args).into(),
            OpCode::Mutagen => self.exec_mutagen_op().into(),
            OpCode::Scavenge => self.exec_scavenge_op().into(),
            OpCode::Digest => self.exec_digest_op().into(),
            OpCode::EntropySurge => {
                nova_flux::exec_entropy_surge(self);
                crate::vm::ops::Dispatch::Handled
            }
            OpCode::QuantumTunnel => nova_flux::exec_quantum_tunnel(self).into(),
            OpCode::Chain | OpCode::Curry | OpCode::Quote => {
                nova_functional::exec_functional_op(self, op, args).into()
            }
            OpCode::Crossover => nova_genetics::exec_crossover(self).into(),
            OpCode::Orca => {
                self.orca_mode = !self.orca_mode;
                let status = if self.orca_mode { "ON" } else { "OFF" };
                self.output
                    .push(format!("ORCA: Signal Processing {}", status));
                crate::vm::ops::Dispatch::Handled
            }
            OpCode::Glossolalia | OpCode::Clarify | OpCode::Confuse => {
                babel_chaos::exec_babel_chaos_op(self, op, args);
                crate::vm::ops::Dispatch::Handled
            }
            OpCode::Crucible => {
                alchemy::exec_crucible_op(self, op, args);
                crate::vm::ops::Dispatch::Handled
            }
            OpCode::Chr => self.exec_char_op().into(),
            OpCode::Guild => nova_guild::exec_guild(self).into(),
            OpCode::Charter => nova_guild::exec_charter(self).into(),
            OpCode::Logos => {
                self.logos_mode = !self.logos_mode;
                let status = if self.logos_mode { "ON" } else { "OFF" };
                self.output
                    .push(format!("LOGOS: Logic Chemistry {}", status));
                crate::vm::ops::Dispatch::Handled
            }
            OpCode::Note | OpCode::Rest | OpCode::Tempo | OpCode::Perform | OpCode::Compose => {
                bard::exec_bard_op(self, op, args);
                crate::vm::ops::Dispatch::Handled
            }
            OpCode::Scan | OpCode::Locate | OpCode::Chart | OpCode::Atlas => {
                nova_cartography::exec_cartography_op(self, op, args);
                crate::vm::ops::Dispatch::Handled
            }
            OpCode::Pocket | OpCode::Unpocket => nova::exec_nova_op(self, op, args).into(),
            OpCode::Quake
            | OpCode::Erode
            | OpCode::Sediment
            | OpCode::Tectonics
            | OpCode::Volcano => {
                nova_geology::exec_geology_op(self, op, args);
                crate::vm::ops::Dispatch::Handled
            }
            OpCode::LeySense | OpCode::LeyTap | OpCode::LeyWarp | OpCode::LeyShift => {
                nova_ley::exec_ley_op(self, op, args).into()
            }
            OpCode::Nucleate | OpCode::Accrete | OpCode::Shatter | OpCode::Anneal => {
                nova_crystal::exec_crystal_op(self, op, args);
                crate::vm::ops::Dispatch::Handled
            }
            OpCode::Dimension | OpCode::DRead | OpCode::DWrite | OpCode::DMerge | OpCode::DView => {
                nova_planes::exec_planes_op(self, op, args);
                crate::vm::ops::Dispatch::Handled
            }
            OpCode::StringNew | OpCode::StringPluck | OpCode::StringTune | OpCode::StringListen => {
                nova_strings::exec_string_op(self, op, args).into()
            }
            OpCode::Bond => nova_metazoa::exec_bond(self, op, args).into(),
            OpCode::Unbond => nova_metazoa::exec_unbond(self, op, args).into(),
            OpCode::Signify => nova_metazoa::exec_signify(self, op, args).into(),
            OpCode::Tissue => nova_metazoa::exec_tissue(self, op, args).into(),
            OpCode::MeshNet
            | OpCode::MeshGrow
            | OpCode::MeshPrune
            | OpCode::MeshSend
            | OpCode::MeshRecv => {
                crate::vm::nova::exec_nova_op(self, op, args);
                crate::vm::ops::Dispatch::Handled
            }
            OpCode::Reactor | OpCode::Reaction => {
                crate::vm::nova::exec_nova_op(self, op, args);
                crate::vm::ops::Dispatch::Handled
            }
            OpCode::AbsorbGeometry => {
                nova_alchemy_prime::exec_absorb_geometry(self, op, args).into()
            }
            OpCode::ProjectGeometry => {
                nova_alchemy_prime::exec_project_geometry(self, op, args).into()
            }
            OpCode::HyperAdd
            | OpCode::HyperSub
            | OpCode::HyperMul
            | OpCode::HyperDiv
            | OpCode::Reduce
            | OpCode::Cross
            | OpCode::ZipWith => {
                nova_raku::exec_raku_op(self, op, args);
                crate::vm::ops::Dispatch::Handled
            }
            OpCode::Flock
            | OpCode::Poincare
            | OpCode::GrayScott
            | OpCode::Locus
            | OpCode::Neuro
            | OpCode::Platter
            | OpCode::MillerLattice
            | OpCode::HyperSystem
            | OpCode::PhysicsPbd
            | OpCode::FerrousCore
            | OpCode::Verge
            | OpCode::CymaticOcean
            | OpCode::QuantumGarden
            | OpCode::Hologram => {
                let msg = match op {
                    OpCode::Flock => "Flocking step simulated.",
                    OpCode::Poincare => "Poincare hyperbolic geometry evaluated.",
                    OpCode::GrayScott => "Gray-Scott diffusion evaluated.",
                    OpCode::Locus => "Locus topology evaluated.",
                    OpCode::Neuro => "Neural network simulated.",
                    OpCode::Platter => "Platter heatmap simulated.",
                    OpCode::MillerLattice => "Miller Lattice simulation triggered.",
                    OpCode::HyperSystem => "Hyper System monitoring triggered.",
                    OpCode::PhysicsPbd => "Physics PBD position-based dynamics triggered.",
                    OpCode::FerrousCore => "Ferrous Core magnetic field triggered.",
                    OpCode::Verge => "Verge Computer logic triggered.",
                    OpCode::CymaticOcean => "Cymatic Ocean simulation triggered.",
                    OpCode::QuantumGarden => "Quantum Garden simulation triggered.",
                    OpCode::Hologram => "Hologram logic triggered.",
                    _ => unreachable!(),
                };
                self.apply_simulation_log(msg)
            }
            OpCode::Tardis => {
                crate::vm::nova_pachinko::exec_tardis(self);
                crate::vm::ops::Dispatch::Jump(self.ip.0, self.ip.1 + 1)
            }
            OpCode::Pachinko => {
                crate::vm::nova_pachinko::exec_pachinko(self);
                crate::vm::ops::Dispatch::Jump(self.ip.0, self.ip.1 + 1)
            }
            OpCode::Automaton => {
                crate::vm::nova_pachinko::exec_automaton(self);
                crate::vm::ops::Dispatch::Jump(self.ip.0, self.ip.1 + 1)
            }
            OpCode::Syncopation => {
                crate::vm::nova_pachinko::exec_syncopation(self);
                crate::vm::ops::Dispatch::Jump(self.ip.0, self.ip.1 + 1)
            }
            OpCode::Choreography => {
                crate::vm::nova_pachinko::exec_choreography(self);
                crate::vm::ops::Dispatch::Jump(self.ip.0, self.ip.1 + 1)
            }
            OpCode::Runes => {
                crate::vm::nova_pachinko::exec_runes(self);
                crate::vm::ops::Dispatch::Jump(self.ip.0, self.ip.1 + 1)
            }
            OpCode::EntropicRain => {
                crate::vm::nova::exec_entropic_rain(self);
                crate::vm::ops::Dispatch::Jump(self.ip.0, self.ip.1 + 1)
            }
            OpCode::SpectralScribe => {
                crate::vm::nova::exec_spectral_scribe(self);
                crate::vm::ops::Dispatch::Jump(self.ip.0, self.ip.1 + 1)
            }

            OpCode::SpqrRsa => {
                crate::vm::nova::exec_spqr_rsa(self);
                crate::vm::ops::Dispatch::Jump(self.ip.0, self.ip.1 + 1)
            }

            OpCode::ClockworkConcerto => {
                crate::vm::nova::exec_clockwork_concerto(self);
                crate::vm::ops::Dispatch::Jump(self.ip.0, self.ip.1 + 1)
            }

            OpCode::GeneticLuthier => {
                crate::vm::nova::exec_genetic_luthier(self);
                crate::vm::ops::Dispatch::Jump(self.ip.0, self.ip.1 + 1)
            }

            OpCode::SorobanSpecter => {
                crate::vm::nova::exec_soroban_specter(self);
                crate::vm::ops::Dispatch::Jump(self.ip.0, self.ip.1 + 1)
            }

            OpCode::ChromaticCode => {
                crate::vm::nova::exec_chromatic_code(self);
                crate::vm::ops::Dispatch::Jump(self.ip.0, self.ip.1 + 1)
            }
            OpCode::ChaosHologram => {
                crate::vm::nova::exec_chaos_hologram(self);
                crate::vm::ops::Dispatch::Handled
            }
            OpCode::HologramText => {
                crate::vm::nova::exec_hologram_text(self);
                crate::vm::ops::Dispatch::Jump(self.ip.0, self.ip.1 + 1)
            }
            #[cfg(feature = "oracle")]
            OpCode::Divergence => nova::exec_nova_op(self, op, args).into(),
            _ => crate::vm::ops::Dispatch::Unhandled,
        }
    }
}
