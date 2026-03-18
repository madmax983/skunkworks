**[Havoc] Size overflow in gray-scott**
**Trigger:** Passing usize::MAX to GrayScott::new
**Defense:** The initialization checks for integer overflow correctly, and we proved it panics gracefully.
