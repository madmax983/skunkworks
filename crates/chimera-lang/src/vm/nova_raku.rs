use crate::ast::{JunctionType, Nucleotide};
use crate::opcode::OpCode;
use crate::value::Value;
use crate::vm::ChimeraVM;

pub fn exec_raku_op(
    vm: &mut ChimeraVM,
    op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    match op {
        OpCode::HyperAdd => exec_hyper(vm, |a, b| a.wrapping_add(b)),
        OpCode::HyperSub => exec_hyper(vm, |a, b| a.wrapping_sub(b)),
        OpCode::HyperMul => exec_hyper(vm, |a, b| a.wrapping_mul(b)),
        OpCode::HyperDiv => exec_hyper(vm, |a, b| if b != 0 { a.wrapping_div(b) } else { 0 }),
        OpCode::Reduce => exec_reduce(vm),
        OpCode::Cross => exec_cross(vm),
        OpCode::ZipWith => exec_zip_with(vm),
        _ => {}
    }
    None
}

fn exec_hyper<F>(vm: &mut ChimeraVM, op_fn: F)
where
    F: Fn(i64, i64) -> i64,
{
    if vm.stack.len() < 2 {
        vm.output.push("Error: Stack underflow for Hyper op".to_string());
        return;
    }
    let b = vm.stack.pop().unwrap();
    let a = vm.stack.pop().unwrap();

    let a_list = as_list(a);
    let b_list = as_list(b);

    if a_list.len() != b_list.len() && a_list.len() != 1 && b_list.len() != 1 {
        vm.output.push(format!(
            "Error: Hyper op shape mismatch: {} vs {}",
            a_list.len(),
            b_list.len()
        ));
        return;
    }

    let max_len = std::cmp::max(a_list.len(), b_list.len());
    let mut results = Vec::new();

    for i in 0..max_len {
        let va = if a_list.len() == 1 { &a_list[0] } else { &a_list[i] };
        let vb = if b_list.len() == 1 { &b_list[0] } else { &b_list[i] };

        if let (Value::Int(ia), Value::Int(ib)) = (va, vb) {
            results.push(Value::Int(op_fn(*ia, *ib)));
        } else {
            // Fallback for non-integers? Just push 0 or skip?
            // Raku would probably coerce or error.
            vm.output.push("Error: Type mismatch in Hyper op".to_string());
            results.push(Value::Int(0));
        }
    }

    vm.stack.push(Value::Junction(JunctionType::All, results));
}

fn exec_reduce(vm: &mut ChimeraVM) {
    if vm.stack.len() < 2 {
        vm.output.push("Error: Stack underflow for Reduce".to_string());
        return;
    }
    let op_val = vm.stack.pop().unwrap();
    let list_val = vm.stack.pop().unwrap();

    if let Value::Str(op_str) = op_val {
        let list = as_list(list_val);
        if list.is_empty() {
            // Identity value? 0 for +, 1 for *.
            // Hard to guess. Just push 0 for now or maybe error.
            vm.stack.push(Value::Int(0));
            return;
        }

        let mut acc = list[0].clone();
        for i in 1..list.len() {
            if let Some(res) = apply_op(&acc, &list[i], &op_str) {
                acc = res;
            } else {
                vm.output.push(format!("Error: Invalid reduce step with {}", op_str));
                return;
            }
        }
        vm.stack.push(acc);
    } else {
        vm.output.push("Error: Reduce expects operator string".to_string());
    }
}

fn exec_cross(vm: &mut ChimeraVM) {
    if vm.stack.len() < 3 {
        vm.output.push("Error: Stack underflow for Cross".to_string());
        return;
    }
    let op_val = vm.stack.pop().unwrap();
    let b = vm.stack.pop().unwrap();
    let a = vm.stack.pop().unwrap();

    if let Value::Str(op_str) = op_val {
        let a_list = as_list(a);
        let b_list = as_list(b);
        let mut results = Vec::new();

        for va in &a_list {
            for vb in &b_list {
                if let Some(res) = apply_op(va, vb, &op_str) {
                    results.push(res);
                } else {
                    results.push(Value::Int(0)); // Error fallback
                }
            }
        }
        vm.stack.push(Value::Junction(JunctionType::All, results));
    } else {
        vm.output.push("Error: Cross expects operator string".to_string());
    }
}

fn exec_zip_with(vm: &mut ChimeraVM) {
    if vm.stack.len() < 3 {
        vm.output.push("Error: Stack underflow for ZipWith".to_string());
        return;
    }
    let op_val = vm.stack.pop().unwrap();
    let b = vm.stack.pop().unwrap();
    let a = vm.stack.pop().unwrap();

    if let Value::Str(op_str) = op_val {
        let a_list = as_list(a);
        let b_list = as_list(b);
        let min_len = std::cmp::min(a_list.len(), b_list.len());
        let mut results = Vec::new();

        for i in 0..min_len {
            if let Some(res) = apply_op(&a_list[i], &b_list[i], &op_str) {
                results.push(res);
            } else {
                results.push(Value::Int(0));
            }
        }
        vm.stack.push(Value::Junction(JunctionType::All, results));
    } else {
        vm.output.push("Error: ZipWith expects operator string".to_string());
    }
}

fn as_list(v: Value) -> Vec<Value> {
    match v {
        Value::Junction(_, list) => list,
        _ => vec![v],
    }
}

fn apply_op(a: &Value, b: &Value, op: &str) -> Option<Value> {
    match (a, b) {
        (Value::Int(ia), Value::Int(ib)) => match op {
            "+" => Some(Value::Int(ia.wrapping_add(*ib))),
            "-" => Some(Value::Int(ia.wrapping_sub(*ib))),
            "*" => Some(Value::Int(ia.wrapping_mul(*ib))),
            "/" => Some(Value::Int(if *ib != 0 { ia.wrapping_div(*ib) } else { 0 })),
            "%" => Some(Value::Int(if *ib != 0 { ia.wrapping_rem(*ib) } else { 0 })),
            "&" => Some(Value::Int(ia & ib)),
            "|" => Some(Value::Int(ia | ib)),
            "^" => Some(Value::Int(ia ^ ib)),
            _ => None,
        },
        _ => None,
    }
}
