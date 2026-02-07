#[cfg(all(test, feature = "nova"))]
mod tests {
    use crate::network::Network;
    use crate::validator::Validator;
    use crate::virus::{ViralEffect, Virus};
    use chimera_lang::ast::{Dna, Helix};

    fn dummy_dna() -> Dna {
        Dna {
            helix: Helix { strands: vec![] },
        }
    }

    #[test]
    fn test_infection_spread() {
        let mut network = Network::new();
        let v1 = Validator::genesis(1, 1000);
        network.add_validator(v1);

        // Add a highly infectious virus
        let virus = Virus::new("TestVirus", dummy_dna(), ViralEffect::EnergyDrain(10), 1.0);
        network.viral_pool = vec![virus.clone()];

        // Step (triggers spread_infection)
        network.step_silent();

        // Validator should be infected
        assert!(network.validators[0].has_infection(&ViralEffect::EnergyDrain(10)));
    }

    #[test]
    fn test_energy_drain() {
        let mut network = Network::new();
        let mut v1 = Validator::genesis(1, 1000);

        let virus = Virus::new("Leech", dummy_dna(), ViralEffect::EnergyDrain(100), 1.0);
        v1.infect(virus);
        network.add_validator(v1);

        let initial_energy = network.validators[0].energy;

        // Metabolism cost is 5, plus drain 100 = 105
        network.metabolism();

        let final_energy = network.validators[0].energy;
        assert_eq!(initial_energy - final_energy, 105);
    }

    #[test]
    fn test_sterility() {
        let mut v1 = Validator::genesis(1, 2000); // Plenty of energy
        assert!(v1.can_reproduce(1200));

        let virus = Virus::new("Sterile", dummy_dna(), ViralEffect::Sterility, 1.0);
        v1.infect(virus);

        assert!(!v1.can_reproduce(1200));
    }

    #[test]
    fn test_byzantine_flip() {
        let mut v1 = Validator::genesis(1, 1000);
        let virus = Virus::new("Chaos", dummy_dna(), ViralEffect::ByzantineFlip, 1.0);
        v1.infect(virus);

        assert!(v1.has_infection(&ViralEffect::ByzantineFlip));
    }
}
