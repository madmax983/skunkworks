#[cfg(feature = "nova")]
use chimera_lang::ast::Dna;

#[cfg(feature = "nova")]
#[derive(Clone, Debug, PartialEq)]
pub enum ViralEffect {
    EnergyDrain(i64),
    Sterility,
    ByzantineFlip,
}

#[cfg(feature = "nova")]
#[derive(Clone, Debug)]
pub struct Virus {
    pub name: String,
    pub dna: Dna, // Flavor: The genetic code of the virus
    pub effect: ViralEffect,
    pub infectivity: f64, // 0.0 to 1.0 chance of infection per tick
}

#[cfg(feature = "nova")]
impl Virus {
    pub fn new(name: &str, dna: Dna, effect: ViralEffect, infectivity: f64) -> Self {
        Virus {
            name: name.to_string(),
            dna,
            effect,
            infectivity,
        }
    }
}
