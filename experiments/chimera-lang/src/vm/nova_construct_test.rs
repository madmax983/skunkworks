#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use super::super::nova_construct::*;
    use crate::opcode::OpCode;
    use crate::vm::Value;

    #[test]
    fn test_compile_construct() {
        let mut construct = ConstructState::new();

        // Literal 10
        construct.add_node(OpCode::Push, Some(Value::Int(10)));
        let id_10 = construct.selected_node.unwrap();

        // Literal 20
        construct.add_node(OpCode::Push, Some(Value::Int(20)));
        let id_20 = construct.selected_node.unwrap();

        // Add
        construct.add_node(OpCode::Add, None);
        let id_add = construct.selected_node.unwrap();

        // Link 10 -> Add (0)
        construct.linking_from = Some(id_10);
        construct.selected_node = Some(id_add);
        construct.complete_link();

        // Link 20 -> Add (1)
        construct.linking_from = Some(id_20);
        construct.selected_node = Some(id_add);
        construct.complete_link();

        let strand = construct.compile().expect("Compilation failed");

        assert_eq!(strand.genes.len(), 3);
        // Order depends on root sort (X pos).
        // All nodes have X=0.0 (default cursor).
        // Sorting might be unstable.
        // But logic visits inputs. Root is Add.
        // Inputs: 10 (idx 0), 20 (idx 1).
        // Visit 10 -> Emit Push(10).
        // Visit 20 -> Emit Push(20).
        // Emit Add.

        assert_eq!(strand.genes[0].op, OpCode::Push);
        assert_eq!(strand.genes[1].op, OpCode::Push);
        assert_eq!(strand.genes[2].op, OpCode::Add);
    }
}
