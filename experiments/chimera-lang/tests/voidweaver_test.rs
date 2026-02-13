#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::vm::nova_weaver::{WeaverGraph, compile_graph};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::ast::Nucleotide;

    #[test]
    fn test_compile_linear_graph() {
        let mut graph = WeaverGraph::new();
        // Node 0: Push(10) at (0,0)
        let n0 = graph.add_node(OpCode::Push, 0.0, 0.0);
        if let Some(node) = graph.nodes.get_mut(&n0) {
            node.args.push(Nucleotide::Number(10));
        }

        // Node 1: Push(20) at (10,0) - Should be second
        let n1 = graph.add_node(OpCode::Push, 10.0, 0.0);
        if let Some(node) = graph.nodes.get_mut(&n1) {
            node.args.push(Nucleotide::Number(20));
        }

        // Node 2: Add at (20,0)
        let _n2 = graph.add_node(OpCode::Add, 20.0, 0.0);

        let genes = compile_graph(&graph);
        assert_eq!(genes.len(), 3);
        assert_eq!(genes[0].op, OpCode::Push);
        assert_eq!(genes[0].args[0], Nucleotide::Number(10));
        assert_eq!(genes[1].op, OpCode::Push);
        assert_eq!(genes[1].args[0], Nucleotide::Number(20));
        assert_eq!(genes[2].op, OpCode::Add);
    }

    #[test]
    fn test_compile_spatial_sort() {
        let mut graph = WeaverGraph::new();
        // Node A at (10, 10)
        let _nA = graph.add_node(OpCode::Add, 10.0, 10.0);
        // Node B at (0, 0) -> Should be first
        let _nB = graph.add_node(OpCode::Push, 0.0, 0.0);

        let genes = compile_graph(&graph);
        assert_eq!(genes.len(), 2);
        assert_eq!(genes[0].op, OpCode::Push);
        assert_eq!(genes[1].op, OpCode::Add);
    }
}
