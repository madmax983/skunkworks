**[Optimizing AST Node Evaluation in chimera-lang]**
**Learning:** `Value` methods for processing tree node structures natively perform deep `.clone()` operations continuously which is extremely expensive, resulting in repeated heap reallocations.
**Action:** Used `&Value` references when evaluating AST trees (`apply_binary_op_recursive_ref`) instead of consuming `Value` during AST mapping operations, saving exponential heap allocations.
