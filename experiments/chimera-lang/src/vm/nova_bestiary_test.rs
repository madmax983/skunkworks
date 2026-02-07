#[cfg(test)]
#[cfg(feature = "nova")]
mod tests {
    use crate::ast::{Strand, Gene};
    use crate::opcode::OpCode;
    use crate::vm::nova_bestiary;

    fn make_gene(op: OpCode) -> Gene {
        Gene { op, args: vec![] }
    }

    #[test]
    fn test_traits_analysis() {
        let genes = vec![
            make_gene(OpCode::Consume),
            make_gene(OpCode::Consume),
            make_gene(OpCode::Consume), // 3 Consumes -> Voracious
            make_gene(OpCode::Photosynthesize), // -> Autotrophic
            make_gene(OpCode::Migrate), // -> Nomadic
        ];
        let strand = Strand { genes };

        let traits = nova_bestiary::analyze_traits(&strand);

        assert!(traits.contains(&"Voracious".to_string()));
        assert!(traits.contains(&"Autotrophic".to_string()));
        assert!(traits.contains(&"Nomadic".to_string()));
        assert!(!traits.contains(&"Dormant".to_string()));
    }

    #[test]
    fn test_name_generation() {
        let traits = vec!["Voracious".to_string()];
        let name1 = nova_bestiary::generate_name(12345, &traits);
        let name2 = nova_bestiary::generate_name(12345, &traits);

        assert_eq!(name1, name2);
        assert!(name1.contains("Devourer"));
    }

    #[test]
    fn test_face_generation() {
        let traits = vec!["Voracious".to_string()];
        let face1 = nova_bestiary::generate_face(12345, &traits);
        let face2 = nova_bestiary::generate_face(12345, &traits);

        assert_eq!(face1, face2);
        assert_eq!(face1.len(), 4);
    }
}
