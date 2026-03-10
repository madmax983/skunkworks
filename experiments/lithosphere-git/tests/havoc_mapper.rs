#[path = "../src/history.rs"]
pub mod history;

use proptest::prelude::*;
use history::FileMapper;

proptest! {
    #[test]
    #[should_panic]
    fn test_file_mapper_zero_dimensions_panic(path in "\\PC*", width in 0usize..=0usize, height in 0usize..=0usize) {
        // 👺 Havoc: FileMapper::new(width, height) allows creating a mapper with zero width and height.
        // When get_coordinate is called, it blindly computes `hash % (width * height)`.
        // This causes an immediate division by zero panic, crashing the system!
        let mut mapper = FileMapper::new(width, height);
        mapper.get_coordinate(&path);
    }
}
