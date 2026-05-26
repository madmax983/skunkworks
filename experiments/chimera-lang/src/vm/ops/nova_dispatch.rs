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
    pub(crate) fn exec_nova_dispatch(
        &mut self,
        op: OpCode,
        args: &[Nucleotide],
    ) -> Option<Option<(usize, usize)>> {
        match op {
            OpCode::Remap | OpCode::Restore | OpCode::Mirror => Some(self.exec_prion_op(op, args)),
            OpCode::AkashicWrite
            | OpCode::AkashicRead
            | OpCode::AkashicSave
            | OpCode::AkashicLoad
            | OpCode::Karma
            | OpCode::Miracle => {
                akashic::exec_akashic_op(self, op, args);
                Some(None)
            }
            OpCode::Blackbox => {
                let dump = self.blackbox.dump();
                self.stack.push(Value::Str(dump));
                Some(None)
            }
            OpCode::Invoke => Some(nova_sigil::exec_invoke(self, op, args)),
            OpCode::Inscribe => Some(nova_sigil::exec_inscribe(self, op, args)),
            OpCode::Ward => Some(nova_ward::exec_ward(self, op, args)),
            OpCode::AutoCast => Some(nova_sigil::exec_auto_cast(self, op, args)),
            OpCode::Vaccinate | OpCode::Verify | OpCode::Audit => {
                Some(nova_security::exec_security_op(self, op, args))
            }
            OpCode::Morph => {
                nova_morphogenesis::exec_morph(self);
                Some(None)
            }
            OpCode::Morphogen => Some(nova_cambrian::exec_morphogen(self, op, args)),
            OpCode::HoxSwitch => Some(nova_cambrian::exec_hox_switch(self, op, args)),
            OpCode::Adhere => Some(nova_cambrian::exec_adhere(self, op, args)),
            OpCode::Grow => {
                nova_morphogenesis::exec_grow(self);
                Some(None)
            }
            OpCode::Plant => {
                nova_botany::exec_plant(self);
                Some(None)
            }
            OpCode::Signal | OpCode::Receive => Some(nova::exec_nova_op(self, op, args)),
            OpCode::Define | OpCode::Undefine | OpCode::Dictionary => {
                Some(meta::exec_meta_op(self, op, args))
            }
            OpCode::Operator => Some(nova::exec_operator(self, args)),
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
            | OpCode::Ouroboros => Some(babel::exec_babel_op(self, op, args)),
            OpCode::Superpose
            | OpCode::Collapse
            | OpCode::Observe
            | OpCode::Interfere
            | OpCode::Project
            | OpCode::Refract => Some(nova::exec_nova_op(self, op, args)),
            OpCode::Levenshtein
            | OpCode::Soundex
            | OpCode::Anagram
            | OpCode::Cipher
            | OpCode::Pangram => Some(nova::exec_nova_op(self, op, args)),
            OpCode::Transposon => Some(self.exec_transposon()),
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
            | OpCode::Prolouge
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
            | OpCode::Etymology => Some(nova::exec_nova_op(self, op, args)),
            OpCode::EvoPopSize
            | OpCode::EvoLoad
            | OpCode::EvoStore
            | OpCode::EvoScore
            | OpCode::EvoBreed
            | OpCode::EvoMutate
            | OpCode::EvoReplace
            | OpCode::EvoClear
            | OpCode::EvoSave => Some(evolution::exec_evo_op(self, op, args)),
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
                Some(None)
            }
            OpCode::Fluid => Some(None),
            OpCode::Weave | OpCode::Unravel => Some(nova_weaver::exec_weave_op(self, op, args)),
            OpCode::Mutagen => Some(self.exec_mutagen_op()),
            OpCode::Scavenge => Some(self.exec_scavenge_op()),
            OpCode::Digest => Some(self.exec_digest_op()),
            OpCode::EntropySurge => {
                nova_flux::exec_entropy_surge(self);
                Some(None)
            }
            OpCode::QuantumTunnel => Some(nova_flux::exec_quantum_tunnel(self)),
            OpCode::Chain | OpCode::Curry | OpCode::Quote => {
                Some(nova_functional::exec_functional_op(self, op, args))
            }
            OpCode::Crossover => Some(nova_genetics::exec_crossover(self)),
            OpCode::Orca => {
                self.orca_mode = !self.orca_mode;
                let status = if self.orca_mode { "ON" } else { "OFF" };
                self.output
                    .push(format!("ORCA: Signal Processing {}", status));
                Some(None)
            }
            OpCode::Glossolalia | OpCode::Clarify | OpCode::Confuse => {
                babel_chaos::exec_babel_chaos_op(self, op, args);
                Some(None)
            }
            OpCode::Crucible => {
                alchemy::exec_crucible_op(self, op, args);
                Some(None)
            }
            OpCode::Chr => Some(self.exec_char_op()),
            OpCode::Guild => Some(nova_guild::exec_guild(self)),
            OpCode::Charter => Some(nova_guild::exec_charter(self)),
            OpCode::Logos => {
                self.logos_mode = !self.logos_mode;
                let status = if self.logos_mode { "ON" } else { "OFF" };
                self.output
                    .push(format!("LOGOS: Logic Chemistry {}", status));
                Some(None)
            }
            OpCode::Note | OpCode::Rest | OpCode::Tempo | OpCode::Perform | OpCode::Compose => {
                bard::exec_bard_op(self, op, args);
                Some(None)
            }
            OpCode::Scan | OpCode::Locate | OpCode::Chart | OpCode::Atlas => {
                nova_cartography::exec_cartography_op(self, op, args);
                Some(None)
            }
            OpCode::Pocket | OpCode::Unpocket => Some(nova::exec_nova_op(self, op, args)),
            OpCode::Quake
            | OpCode::Erode
            | OpCode::Sediment
            | OpCode::Tectonics
            | OpCode::Volcano => {
                nova_geology::exec_geology_op(self, op, args);
                Some(None)
            }
            OpCode::LeySense | OpCode::LeyTap | OpCode::LeyWarp | OpCode::LeyShift => {
                Some(nova_ley::exec_ley_op(self, op, args))
            }
            OpCode::Nucleate | OpCode::Accrete | OpCode::Shatter | OpCode::Anneal => {
                nova_crystal::exec_crystal_op(self, op, args);
                Some(None)
            }
            OpCode::Dimension | OpCode::DRead | OpCode::DWrite | OpCode::DMerge | OpCode::DView => {
                nova_planes::exec_planes_op(self, op, args);
                Some(None)
            }
            OpCode::StringNew | OpCode::StringPluck | OpCode::StringTune | OpCode::StringListen => {
                Some(nova_strings::exec_string_op(self, op, args))
            }
            OpCode::Bond => Some(nova_metazoa::exec_bond(self, op, args)),
            OpCode::Unbond => Some(nova_metazoa::exec_unbond(self, op, args)),
            OpCode::Signify => Some(nova_metazoa::exec_signify(self, op, args)),
            OpCode::Tissue => Some(nova_metazoa::exec_tissue(self, op, args)),
            OpCode::MeshNet
            | OpCode::MeshGrow
            | OpCode::MeshPrune
            | OpCode::MeshSend
            | OpCode::MeshRecv => {
                crate::vm::nova::exec_nova_op(self, op, args);
                Some(None)
            }
            OpCode::Reactor | OpCode::Reaction => {
                crate::vm::nova::exec_nova_op(self, op, args);
                Some(None)
            }
            OpCode::AbsorbGeometry => {
                Some(nova_alchemy_prime::exec_absorb_geometry(self, op, args))
            }
            OpCode::ProjectGeometry => {
                Some(nova_alchemy_prime::exec_project_geometry(self, op, args))
            }
            OpCode::HyperAdd
            | OpCode::HyperSub
            | OpCode::HyperMul
            | OpCode::HyperDiv
            | OpCode::Reduce
            | OpCode::Cross
            | OpCode::ZipWith => {
                nova_raku::exec_raku_op(self, op, args);
                Some(None)
            }
            OpCode::Flock => {
                self.output.push("Flocking step simulated.".to_string());
                Some(Some((self.ip.0, self.ip.1 + 1)))
            }
            OpCode::Poincare => {
                self.output
                    .push("Poincare hyperbolic geometry evaluated.".to_string());
                Some(Some((self.ip.0, self.ip.1 + 1)))
            }
            #[cfg(feature = "oracle")]
            OpCode::Divergence => Some(nova::exec_nova_op(self, op, args)),
            _ => None,
        }
    }
}
