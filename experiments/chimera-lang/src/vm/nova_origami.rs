use crate::vm::ChimeraVM;
use crate::vm::Value;

/// Performs the `exec_origami` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_origami
/// ```
pub fn exec_origami(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        let extension_factor = match val {
            Value::Int(n) => n as f32 / 100.0,
            _ => {
                vm.output
                    .push("Error: Origami extension factor must be an Int (0-100)".to_string());
                return None;
            }
        };

        let params = origami::MiuraParams {
            a: 1.0,
            b: 1.0,
            gamma: 1.4, // standard 80 deg ish
            orientation: origami::Orientation::Horizontal,
        };

        let mesh = origami::generate_miura_grid(params, (8, 8), extension_factor);
        vm.output.push(format!(
            "Origami: Folded Miura-ori mesh with {} vertices at extension {}",
            mesh.len(),
            extension_factor
        ));
    } else {
        vm.output
            .push("Error: Stack underflow for Origami fold".to_string());
    }
    None
}
