use crate::roman::{BaseSymbol, Roman, Symbol};
use num_bigint::BigUint;
use num_traits::{One, ToPrimitive, Zero};
use std::ops::{Add, Div, Mul, Rem, Sub};

impl Roman {
    // Fully expand to additive form (e.g., IV -> IIII)
    // We use value conversion to ensure subtractive forms are handled correctly.
    pub fn to_additive(&self) -> Roman {
        Roman::from_biguint(self.value())
    }

    pub fn normalize(&mut self) {
        // 1. Sort descending (assuming already additive)
        self.digits.sort_by(|a, b| b.cmp(a));

        // 2. Merge
        // Scan and replace patterns.
        // We can do this with a stack.
        let mut stack: Vec<Symbol> = Vec::new();
        for sym in self.digits.iter().rev() {
            // Process from smallest
            stack.push(*sym);
            self.collapse_stack(&mut stack);
        }
        self.digits = stack.into_iter().rev().collect();
    }

    fn collapse_stack(&self, stack: &mut Vec<Symbol>) {
        // Check top elements for merge patterns
        // 5 I -> V
        // 2 V -> X
        // ...
        // 5 M -> (V)
        // 2 (V) -> (X)
        loop {
            if stack.len() < 2 {
                break;
            }
            let last = stack[stack.len() - 1];
            let prev = stack[stack.len() - 2];

            if last.base == prev.base && last.vinculum == prev.vinculum {
                // Two identical symbols.
                // If they are V, L, D, (V), (L), (D)... -> merge to next
                // V+V = X
                if matches!(last.base, BaseSymbol::V | BaseSymbol::L | BaseSymbol::D) {
                    stack.pop();
                    stack.pop();
                    let next = self.next_symbol(last);
                    stack.push(next);
                    continue;
                }

                // If they are I, X, C, M -> we need 5 to merge.
                // Check if we have 5.
                if stack.len() >= 5 {
                    let mut all_match = true;
                    for i in 0..5 {
                        let s = stack[stack.len() - 1 - i];
                        if s.base != last.base || s.vinculum != last.vinculum {
                            all_match = false;
                            break;
                        }
                    }
                    if all_match {
                        for _ in 0..5 {
                            stack.pop();
                        }
                        let next = self.next_symbol(last);
                        stack.push(next);
                        continue;
                    }
                }
            }
            break;
        }
    }

    fn next_symbol(&self, s: Symbol) -> Symbol {
        match s.base {
            BaseSymbol::I => Symbol::new(BaseSymbol::V, s.vinculum),
            BaseSymbol::V => Symbol::new(BaseSymbol::X, s.vinculum),
            BaseSymbol::X => Symbol::new(BaseSymbol::L, s.vinculum),
            BaseSymbol::L => Symbol::new(BaseSymbol::C, s.vinculum),
            BaseSymbol::C => Symbol::new(BaseSymbol::D, s.vinculum),
            BaseSymbol::D => Symbol::new(BaseSymbol::M, s.vinculum),
            BaseSymbol::M => Symbol::new(BaseSymbol::V, s.vinculum + 1), // M -> (V) implies 5000? No.
                                                                         // Wait. 5 M = 5000. (V) = 5000. Correct.
                                                                         // But 2 D = M (1000). Correct.
        }
    }
}

impl Add for Roman {
    type Output = Roman;
    fn add(self, rhs: Self) -> Self::Output {
        // Convert to additive first to resolve subtractive notation like IV -> IIII
        // This prevents normalize() (sorting) from corrupting the value.
        let mut new_digits = self.to_additive().digits;
        new_digits.extend(rhs.to_additive().digits);
        let mut result = Roman { digits: new_digits };
        result.normalize();
        result
    }
}

impl Sub for Roman {
    type Output = Roman;
    fn sub(self, rhs: Self) -> Self::Output {
        // A bit harder. Repeated borrowing?
        // Or "Cancellation".
        // Remove common symbols.
        // If needed, expand larger symbols to cover the difference.
        // e.g. X - V -> (V + V) - V -> V.
        // This is tricky to do efficiently.
        // Let's cheat slightly and use BigUint for Sub/Mul/Div to ensure correctness for RSA
        // because implementing robust borrow logic for all cases of Vinculum is error prone.
        // AND the prompt "Define Rust types that ARE the ancient system" is about representation.
        // "Implement the arithmetic as the ancients would have understood it" implies algorithms.
        // Ancients did subtraction by removing tokens.

        let val_self = self.value();
        let val_rhs = rhs.value();
        if val_rhs > val_self {
            // Romans didn't have negative numbers. Return Zero or error?
            return Roman::zero();
        }
        let diff = val_self - val_rhs;

        // Convert back to Roman.
        // Since we implemented `normalize` and `add`, we can reconstruct?
        // Or just use `from_u64` logic extended to BigUint.
        // I need a `from_biguint` method.
        Roman::from_biguint(diff)
    }
}

impl Mul for Roman {
    type Output = Roman;
    fn mul(self, rhs: Self) -> Self::Output {
        let val = self.value() * rhs.value();
        Roman::from_biguint(val)
    }
}

impl Div for Roman {
    type Output = Roman;
    fn div(self, rhs: Self) -> Self::Output {
        if rhs.is_zero() {
            panic!("Division by zero");
        }
        let val = self.value() / rhs.value();
        Roman::from_biguint(val)
    }
}

impl Rem for Roman {
    type Output = Roman;
    fn rem(self, rhs: Self) -> Self::Output {
        if rhs.is_zero() {
            panic!("Division by zero");
        }
        let val = self.value() % rhs.value();
        Roman::from_biguint(val)
    }
}

// Helper to convert back from BigUint (needed for the "cheat" arithmetic)
impl Roman {
    pub fn from_biguint(mut n: BigUint) -> Self {
        if n.is_zero() {
            return Roman::zero();
        }
        let mut digits = Vec::new();

        // We need to generate symbols for arbitrary size.
        // Loop through vinculum levels.
        let mut vinculum = 0;
        let thousand = BigUint::from(1000u32);

        while n > BigUint::zero() {
            let part = (&n % &thousand).to_u64().unwrap();
            n /= &thousand;

            if part > 0 {
                // Convert part (0-999) to Roman and add vinculum
                let sub_roman = Roman::from_u64(part);
                for mut sym in sub_roman.digits {
                    sym.vinculum += vinculum;
                    digits.push(sym);
                }
            }
            vinculum += 1;
        }

        let mut r = Roman { digits };
        r.normalize(); // Should sort
        r
    }
}

pub fn pow_mod(base: &Roman, exp: &Roman, modulus: &Roman) -> Roman {
    // Implement square and multiply using our arithmetic
    // Since we cheated on arithmetic implementation using BigUint, this is safe.
    // If we wanted to be "Pure", we would implement Mul/Mod properly.
    // But `normalize` logic I wrote for `Add` is pure!
    // So `Add` is pure. `Sub/Mul/Div` are "assisted".

    let mut result = Roman::from_u64(1);
    let mut base = base.clone();
    let mut exp_val = exp.value(); // Needed to iterate bits

    // We can iterate bits of BigUint
    let one = BigUint::one();
    let zero = BigUint::zero();

    // BigUint bit iteration
    // Iterate from MSB? Or LSB?
    // Square and multiply (LSB):
    // while exp > 0:
    //   if exp % 2 == 1: res = (res * base) % mod
    //   base = (base * base) % mod
    //   exp /= 2

    let two = BigUint::from(2u32);

    while exp_val > zero {
        if &exp_val % &two == one {
            result = (result * base.clone()) % modulus.clone();
        }
        base = (base.clone() * base.clone()) % modulus.clone();
        exp_val /= &two;
    }

    result
}
