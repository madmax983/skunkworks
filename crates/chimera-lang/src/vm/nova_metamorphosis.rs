#![cfg(feature = "nova")]

use crate::vm::nova::OrganelleType;
use crate::vm::ChimeraVM;

pub fn process_organelle_growth(vm: &mut ChimeraVM) {
    let organelles = &mut vm.organelles;
    let output = &mut vm.output;

    for org in organelles.iter_mut() {
        if org.halted {
            continue;
        }

        let threshold = 100 * (org.stage as i64 + 1);
        if org.experience >= threshold {
            org.stage += 1;
            // Consume XP? Or accumulate? Usually accumulate total XP.
            // Let's keep total XP.

            output.push(format!(
                "METAMORPHOSIS: {} evolved to Stage {}",
                org.name, org.stage
            ));

            // Apply Stage Effects
            match org.stage {
                1 => {
                    // Larva -> Pupa
                    org.name.push_str(" (Pupa)");
                    org.traits.push("Cocoon".to_string());
                    // Pupa is tough but immobile?
                    // We can't change logic easily here without complex state checks elsewhere.
                    // Just adding trait for now.
                }
                2 => {
                    // Pupa -> Imago
                    if org.name.ends_with(" (Pupa)") {
                        org.name = org.name.replace(" (Pupa)", " (Imago)");
                    } else {
                        org.name.push_str(" (Imago)");
                    }

                    // Remove Cocoon
                    if let Some(pos) = org.traits.iter().position(|x| *x == "Cocoon") {
                        org.traits.remove(pos);
                    }

                    org.traits.push("Wings".to_string());

                    // Evolution of Kind
                    if org.kind == OrganelleType::Worker {
                        // 50% chance to become specialized
                        if org.genome_id % 2 == 0 {
                            // Become Chloroplast (Producer)
                            // We can't easily change kind arbitrarily if it breaks logic, but OrganelleType is enum.
                            // But org.kind is public.
                            // However, we are inside iter_mut, so we can modify.
                            // But we shouldn't change kind that requires different fields? Organelle struct is uniform.
                            // So changing kind is safe!
                            // org.kind = OrganelleType::Chloroplast;
                            // Actually, let's keep Worker as "Imago Worker" (maybe faster?).
                        }
                    }

                    // Energy Boost
                    org.energy = org.energy.saturating_add(100);
                }
                _ => {
                    // Titan?
                    if org.stage == 3 {
                        if org.name.ends_with(" (Imago)") {
                            org.name = org.name.replace(" (Imago)", " (Titan)");
                        } else {
                            org.name.push_str(" (Titan)");
                        }
                        org.traits.push("Colossal".to_string());
                        org.energy = org.energy.saturating_add(500);
                    }
                }
            }
        }
    }
}
