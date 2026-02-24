#[cfg(all(test, feature = "nova"))]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_invest_divest() {
        // [ push(10) invest() balance() push(5) divest() balance() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Invest,
                args: vec![],
            },
            Gene {
                op: OpCode::Balance,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Divest,
                args: vec![],
            },
            Gene {
                op: OpCode::Balance,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.energy = 50;

        vm.step(); // push(10)
        vm.step(); // Invest 10
        vm.step(); // Balance -> Stack: [10]
        vm.step(); // push(5) -> Stack: [10, 5]
        vm.step(); // Divest 5 -> Stack: [10]
        vm.step(); // Balance -> Stack: [10, 5]

        assert_eq!(vm.stack[0], Value::Int(10));
        assert_eq!(vm.stack[1], Value::Int(5));

        assert_eq!(vm.market.get_balance(0), 5);
    }

    #[test]
    fn test_market_trade() {
        // Strand 0 (Seller): [ push(10) invest() push("Secret") push(10) offer() ]
        // Strand 1 (Buyer):  [ push(10) invest() push("Secret") push(10) buy() ]

        let seller_genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Invest,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("Secret".to_string())],
            },
            Gene {
                op: OpCode::Offer,
                args: vec![],
            },
        ];

        let buyer_genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Invest,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("Secret".to_string())],
            },
            Gene {
                op: OpCode::Buy,
                args: vec![],
            },
        ];

        let dna = Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![
                    Strand {
                        genes: seller_genes,
                    },
                    Strand { genes: buyer_genes },
                ],
            },
        };

        let mut vm = ChimeraVM::new(dna);
        // Give enough energy
        vm.energy = 100;

        // Run Seller (Strand 0)
        // 5 instructions
        for _ in 0..5 {
            vm.step();
        }

        assert_eq!(vm.market.get_balance(0), 10);
        assert_eq!(vm.market.asks.len(), 1);

        // Run Buyer (Strand 1)
        // Switch strand takes 1 step
        vm.step();
        // 5 instructions
        for _ in 0..5 {
            vm.step();
        }

        // Buyer should have bought item
        // Stack: [Cost, Item] ?
        // Buy: `[ ..., max_price, query ] -> [ ..., item, cost ]`
        // My stack order: [10, "Secret", 10] (after push 10)
        // Buy pops: max_price=10, query="Secret".
        // Returns: item="Secret", cost=10.
        // Stack: ["Secret", 10] (Top is cost)

        let cost = vm.stack.pop().unwrap();
        let item = vm.stack.pop().unwrap();

        assert_eq!(cost, Value::Int(10));
        assert_eq!(item, Value::Str("Secret".to_string()));

        // Balances:
        // Seller: 10 (initial) + 10 (sale) = 20
        // Buyer: 10 (initial) - 10 (buy) = 0
        assert_eq!(vm.market.get_balance(0), 20);
        assert_eq!(vm.market.get_balance(1), 0);
    }
}
