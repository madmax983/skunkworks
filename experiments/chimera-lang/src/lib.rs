use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ChimeraParser;

pub mod ast;
pub mod opcode;
pub mod tui;
pub mod vm;

#[cfg(test)]
mod cortex_test;
mod havoc_repro;
#[cfg(all(test, feature = "nova"))]
mod nova_biolum_test;
#[cfg(test)]
mod nova_cerebellum_test;
#[cfg(all(test, feature = "nova"))]
mod nova_conjugation_test;
#[cfg(test)]
mod nova_crispr_test;
#[cfg(all(test, feature = "nova"))]
mod nova_gravity_test;
#[cfg(test)]
mod nova_hormone_test;
#[cfg(test)]
mod nova_quantum_test;
#[cfg(test)]
mod nova_spore_test;
mod nova_test;
#[cfg(test)]
<<<<<<< sentry-chimera-nova-tests-12531520350046153491
mod cortex_test;
#[cfg(test)]
mod sentry_nova_test;
=======
mod nova_waste_test;
>>>>>>> trunk
