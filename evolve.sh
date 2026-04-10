cat << 'INNER_EOF' > my_patch.diff
--- a/experiments/chimera-lang/src/prolouge_grammar.pest
+++ b/experiments/chimera-lang/src/prolouge_grammar.pest
@@ -3,7 +3,7 @@

 program = { SOI ~ section* ~ EOI }
-section = { forth_block | raku_block | orca_block | elektra_block | prolog_block | genetics_block | lisp_block | brainfuck_block | tui_block | piet_block }
+section = { forth_block | raku_block | orca_block | elektra_block | prolog_block | genetics_block | lisp_block | brainfuck_block | tui_block | piet_block | chaos_block }

 forth_block = { "forth" ~ "{" ~ instruction* ~ "}" }
 raku_block = { "raku" ~ "{" ~ hyper_instruction* ~ "}" }
@@ -15,6 +15,7 @@
 brainfuck_block = { "brainfuck" ~ "{" ~ (!"}" ~ ANY)* ~ "}" }
 tui_block = { "tui" ~ "{" ~ (!"}" ~ ANY)* ~ "}" }
 piet_block = { "piet" ~ "{" ~ (!"}" ~ ANY)* ~ "}" }
+chaos_block = { "chaos" ~ "{" ~ instruction* ~ "}" }

 instruction = { string | number | identifier }
 hyper_instruction = { (">>" ~ operator ~ "<<") | string | number | identifier }
--- a/experiments/chimera-lang/src/prolouge_compiler.rs
+++ b/experiments/chimera-lang/src/prolouge_compiler.rs
@@ -141,6 +141,11 @@
                 ));
                 genes.push(Gene::new(OpCode::Piet, vec![]));
             }
+            Rule::chaos_block => {
+                for instr in inner_block.into_inner() {
+                    genes.extend(compile_chaos_instr(instr)?);
+                }
+            }
             _ => {}
         }
     }
@@ -330,6 +335,27 @@
     Ok(genes)
 }

+fn compile_chaos_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
+    let mut genes = Vec::new();
+    let inner = pair.into_inner().next().unwrap();
+
+    if inner.as_rule() == Rule::identifier {
+        let op = inner.as_str().to_ascii_lowercase();
+        match op.as_str() {
+            "glitch" => genes.push(Gene::new(OpCode::Glitch, vec![])),
+            "havoc" => genes.push(Gene::new(OpCode::Havoc, vec![])),
+            "chaos" => genes.push(Gene::new(OpCode::Chaos, vec![])),
+            "entropy" => genes.push(Gene::new(OpCode::EntropySurge, vec![])),
+            _ => {
+                if let Ok(opcode) = OpCode::from_str(&op) {
+                    genes.push(Gene::new(opcode, vec![]));
+                } else {
+                    genes.push(Gene::new(OpCode::Unknown(op.to_string()), vec![]));
+                }
+            }
+        }
+    } else if inner.as_rule() == Rule::number {
+        let n: i64 = inner.as_str().parse()?;
+        genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(n)]));
+    }
+    Ok(genes)
+}
+
 #[cfg(test)]
 mod tests {
     use super::*;
@@ -361,4 +387,22 @@
         assert_eq!(genes[5].args[0], Nucleotide::String("b".to_string()));
         assert_eq!(genes[6].op, OpCode::Recombine);
     }
+
+    #[test]
+    fn test_chaos_block() {
+        let code = r#"
+chaos {
+    100
+    glitch
+    entropy
+}
+"#;
+        let dna = compile(code).unwrap();
+        let genes = &dna.helix.strands[0].genes;
+
+        assert_eq!(genes[0].op, OpCode::Push);
+        assert_eq!(genes[0].args[0], Nucleotide::Number(100));
+        assert_eq!(genes[1].op, OpCode::Glitch);
+        assert_eq!(genes[2].op, OpCode::EntropySurge);
+    }
 }
INNER_EOF
patch -p1 < my_patch.diff
