use crate::lsystem::LSystem;
use soroban::Column;

pub fn get_lsystem_for_column(col: &Column) -> LSystem {
    let earth = col.lower_active; // 0-4
    let heaven = col.upper_active; // bool

    // Axiom is always X (The tip of the plant)
    let axiom = "X";

    // Earth beads determine the stem/branching structure (F)
    // 0: Sparse
    // 4: Dense
    let f_rule = match earth {
        0 => "FF",              // Just length
        1 => "FF[+F]",          // One branch
        2 => "FF[-F]",          // Other side
        3 => "FF[+F][-F]",      // Both sides
        4 => "FF[+F]F[-F]",     // More complex
        _ => "F",
    };

    // Heaven bead determines the Apex (X)
    // If active, it produces a Flower (O)
    let x_rule = if heaven {
        "F[+X][-X]O" // Bushy top with Flower
    } else {
        "FX"         // Continue growing
    };

    let o_rule = "O"; // Flower symbol

    let rules = vec![
        ('X', x_rule),
        ('F', f_rule),
        ('O', o_rule),
    ];

    LSystem::new(axiom, rules)
}
