use quipu::{Cord, Quipu};

fn main() {
    // 1. Create a new Quipu to record the harvest.
    let mut harvest_record = Quipu::new();

    // 2. Record 123 sacks of potatoes.
    //    - 1 Hundred (Simple)
    //    - 2 Tens (Simple, Simple)
    //    - 3 Units (Long Knot with 3 turns)
    let potatoes = Cord::from(123);
    harvest_record.add_cord(potatoes);

    // 3. Record 45 sacks of maize.
    //    - 4 Tens (Simple x4)
    //    - 5 Units (Long Knot with 5 turns)
    let maize = Cord::from(45);
    harvest_record.add_cord(maize);

    // 4. Calculate the total harvest.
    //    The Incas performed arithmetic by moving knots or combining cords.
    let total = harvest_record.cords[0]
        .checked_add(&harvest_record.cords[1])
        .unwrap();

    assert_eq!(total.value(), 168);

    // Display the total cord (TUI representation)
    // Output:
    // ●          (1 Hundred)
    // ● ● ● ● ● ● (6 Tens)
    // ≡8         (8 Units)
    println!("{}", total);
}
