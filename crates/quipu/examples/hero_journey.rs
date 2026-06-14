use quipu::{Cord, Quipu};

fn main() {
    let mut harvest_record = Quipu::new();

    let potatoes = Cord::from(123);
    harvest_record.add_cord(potatoes);

    let maize = Cord::from(45);
    harvest_record.add_cord(maize);

    let total = harvest_record.cords[0]
        .checked_add(&harvest_record.cords[1])
        .unwrap();

    assert_eq!(total.value(), 168);

    println!("{}", total);
}
