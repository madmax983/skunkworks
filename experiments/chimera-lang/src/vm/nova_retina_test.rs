#[cfg(all(test, feature = "nova"))]
mod tests {
    use crate::ast::{Dna, Helix, Strand};
    use crate::opcode::OpCode;
    use crate::vm::ChimeraVM;
    use crate::vm::Value;

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes: vec![] }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_retina_draw() {
        let mut vm = make_vm();

        // Color: Red (R=255, G=0, B=0) -> 0xFF0000 -> 16711680
        let color = 0xFF0000;
        let char_code = 'X' as i64;
        let y = 10;
        let x = 20;

        // OpCode expects Stack: [ ..., packed_color, char_code, y, x ] (x is top)
        vm.stack.push(Value::Int(color));
        vm.stack.push(Value::Int(char_code));
        vm.stack.push(Value::Int(y));
        vm.stack.push(Value::Int(x));

        // Execute RetinaDraw manually
        vm.execute_gene_inner(OpCode::RetinaDraw, &[]);

        // Verify buffer
        let (ch, (r, g, b)) = vm.retina.buffer[10][20];
        assert_eq!(ch, 'X');
        assert_eq!(r, 255);
        assert_eq!(g, 0);
        assert_eq!(b, 0);
    }

    #[test]
    fn test_retina_clear() {
        let mut vm = make_vm();

        // Draw something first
        vm.retina.draw(0, 0, 'A', 255, 255, 255);

        // Clear with Blue (0x0000FF)
        let color = 0x0000FF;
        vm.stack.push(Value::Int(color));

        vm.execute_gene_inner(OpCode::RetinaClear, &[]);

        // Check 0,0
        let (ch, (r, g, b)) = vm.retina.buffer[0][0];
        assert_eq!(ch, ' '); // Clear sets char to space
        assert_eq!(r, 0);
        assert_eq!(g, 0);
        assert_eq!(b, 255);

        // Check another random pixel
        let (ch2, (r2, g2, b2)) = vm.retina.buffer[10][10];
        assert_eq!(ch2, ' ');
        assert_eq!(r2, 0);
        assert_eq!(g2, 0);
        assert_eq!(b2, 255);
    }

    #[test]
    fn test_retina_size() {
        let mut vm = make_vm();

        vm.execute_gene_inner(OpCode::RetinaSize, &[]);

        // Stack should have [width, height] (height on top)
        // Wait, logic: push(w); push(h). Top is h.
        let h = vm.stack.pop().unwrap();
        let w = vm.stack.pop().unwrap();

        assert_eq!(h, Value::Int(32));
        assert_eq!(w, Value::Int(64));
    }
}
