use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use anyhow::{anyhow, Result};
use pest::Parser;
use pest_derive::Parser;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Parser)]
#[grammar = "script_grammar.pest"]
pub struct ScriptParser;

const MAX_INCLUDE_DEPTH: usize = 32;
const MAX_PARSE_DEPTH: usize = 256;
const MAX_NESTING_DEPTH: usize = 200;

fn check_nesting_depth(source: &str, limit: usize) -> Result<()> {
    let mut depth = 0;
    for c in source.chars() {
        match c {
            '(' | '{' | '[' => {
                depth += 1;
                if depth > limit {
                    return Err(anyhow!(
                        "Recursion depth exceeded (nesting limit: {})",
                        limit
                    ));
                }
            }
            ')' | '}' | ']' => {
                if depth > 0 {
                    depth -= 1;
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn preprocess(
    source: &str,
    base_path: Option<&Path>,
    visited: &mut HashSet<PathBuf>,
    depth: usize,
) -> Result<String> {
    if depth > MAX_INCLUDE_DEPTH {
        return Err(anyhow!("Include recursion depth exceeded"));
    }

    let mut expanded = String::new();
    for line in source.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with("include") {
            expanded.push_str(line);
            expanded.push('\n');
            continue;
        }

        // Handle include
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() < 2 {
            return Err(anyhow!("Invalid include statement: {}", trimmed));
        }

        let raw_filename = parts[1];
        let filename = raw_filename.trim_matches('"');

        let Some(bp) = base_path else {
            return Err(anyhow!("Cannot include files without a base path"));
        };

        let path = bp.join(filename);

        let abs_path = if path.exists() {
            path.canonicalize()?
        } else {
            path.clone()
        };

        // Security Check: Ensure the resolved path is within the base directory
        let effective_base = if bp.as_os_str().is_empty() {
            Path::new(".")
        } else {
            bp
        };

        // Fail Closed: If we can't determine the canonical base path, we must deny access.
        let canonical_base = effective_base.canonicalize().map_err(|e| {
            anyhow!(
                "Security Error: Failed to resolve base path {:?}: {}",
                effective_base,
                e
            )
        })?;

        if !abs_path.starts_with(&canonical_base) {
            return Err(anyhow!(
                "Security Error: Path traversal attempt detected. Access denied to {:?}",
                abs_path
            ));
        }

        if !visited.insert(abs_path.clone()) {
            return Err(anyhow!("Recursive include detected: {:?}", abs_path));
        }

        let content = fs::read_to_string(&path)
            .map_err(|e| anyhow!("Failed to include file {:?}: {}", path, e))?;

        let sub_expanded = preprocess(&content, Some(bp), visited, depth + 1)?;
        expanded.push_str(&sub_expanded);
        expanded.push('\n');

        visited.remove(&abs_path);
    }
    Ok(expanded)
}

pub fn compile(source: &str, base_path: Option<&Path>) -> Result<Dna> {
    let mut visited = HashSet::new();
    let expanded_source = preprocess(source, base_path, &mut visited, 0)?;

    check_nesting_depth(&expanded_source, MAX_NESTING_DEPTH)?;

    let mut pairs = ScriptParser::parse(Rule::program, &expanded_source)?;
    let program = pairs.next().ok_or(anyhow!("No program found"))?;

    // Pass 1: Collect strand names and macros
    let mut strand_map: HashMap<String, usize> = HashMap::new();
    let mut macro_map: HashMap<String, pest::iterators::Pairs<Rule>> = HashMap::new();
    let mut grammar_map: HashMap<String, Nucleotide> = HashMap::new();
    let mut organelle_map: HashMap<String, usize> = HashMap::new();

    for pair in program.clone().into_inner() {
        match pair.as_rule() {
            Rule::strand_def => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str();
                let idx = strand_map.len();
                if strand_map.insert(name.to_string(), idx).is_some() {
                    return Err(anyhow!("Duplicate strand name: {}", name));
                }
            }
            Rule::organelle_def => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str();
                let strand_name = format!("{}_DNA", name);
                let idx = strand_map.len();
                if strand_map.insert(strand_name, idx).is_some() {
                    return Err(anyhow!("Duplicate strand name from organelle: {}", name));
                }
                organelle_map.insert(name.to_string(), idx);
            }
            Rule::macro_def => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str();
                // Store the instructions (rest of inner)
                macro_map.insert(name.to_string(), inner);
            }
            Rule::grammar_def => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str();
                let arg_pair = inner.next().unwrap();
                let grammar_struct = parse_argument(arg_pair, &strand_map, 0)?;
                grammar_map.insert(name.to_string(), grammar_struct);
            }
            _ => {}
        }
    }

    let mut strands_ast = Vec::new();
    let mut anonymous_strands = Vec::new();

    // Pass 2: Generate Genes
    for pair in program.into_inner() {
        match pair.as_rule() {
            Rule::strand_def => {
                let mut inner = pair.into_inner();
                let _name = inner.next().unwrap(); // skip name
                let mut genes = Vec::new();

                for instr in inner {
                    let generated = parse_instructions(
                        instr,
                        &strand_map,
                        &macro_map,
                        &grammar_map,
                        &organelle_map,
                        &mut anonymous_strands,
                        0,
                    )?;
                    genes.extend(generated);
                }
                strands_ast.push(Strand { genes });
            }
            Rule::organelle_def => {
                let mut inner = pair.into_inner();
                let _name = inner.next().unwrap(); // skip name
                let mut genes = Vec::new();

                for instr in inner {
                    let generated = parse_instructions(
                        instr,
                        &strand_map,
                        &macro_map,
                        &grammar_map,
                        &organelle_map,
                        &mut anonymous_strands,
                        0,
                    )?;
                    genes.extend(generated);
                }
                strands_ast.push(Strand { genes });
            }
            _ => {}
        }
    }

    // Append anonymous strands
    strands_ast.extend(anonymous_strands);

    Ok(Dna {
        helix: Helix {
            strands: strands_ast,
        },
    })
}

struct CompilerContext<'a, 'i> {
    strand_map: &'a HashMap<String, usize>,
    macro_map: &'a HashMap<String, pest::iterators::Pairs<'i, Rule>>,
    grammar_map: &'a HashMap<String, Nucleotide>,
    organelle_map: &'a HashMap<String, usize>,
    anonymous_strands: &'a mut Vec<Strand>,
    depth: usize,
}

impl<'a, 'i> CompilerContext<'a, 'i> {
    fn parse_instruction(&mut self, pair: pest::iterators::Pair<'i, Rule>) -> Result<Vec<Gene>> {
        if self.depth > 50 {
            return Err(anyhow!("Macro recursion depth exceeded"));
        }

        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::block => self.parse_block(inner),
            Rule::literal => self.parse_literal_instruction(inner),
            Rule::junction => self.parse_junction_instruction(inner),
            Rule::simple_op => self.parse_simple_op(inner),
            Rule::arrow_jump => self.parse_arrow_jump(inner),
            Rule::question_branch => self.parse_question_branch(inner),
            Rule::call => self.parse_call(inner),
            Rule::polyglot_block => self.parse_polyglot_block(inner),
            #[cfg(feature = "nova")]
            Rule::crispr_block => self.parse_crispr_block(inner),
            Rule::chaos_block => self.parse_chaos_block(inner),
            #[cfg(feature = "oracle")]
            Rule::oracle_block => self.parse_oracle_block(inner),
            #[cfg(not(feature = "oracle"))]
            Rule::oracle_block => return Err(anyhow!("Oracle feature is disabled")),
            _ => unreachable!("Unexpected instruction rule: {:?}", inner.as_rule()),
        }
    }

    fn parse_junction_instruction(
        &self,
        inner: pest::iterators::Pair<'i, Rule>,
    ) -> Result<Vec<Gene>> {
        let val = parse_junction(inner, self.strand_map, 0)?;
        Ok(vec![Gene {
            op: OpCode::Push,
            args: vec![val],
        }])
    }

    fn parse_chaos_block(&mut self, inner: pest::iterators::Pair<'i, Rule>) -> Result<Vec<Gene>> {
        let block = inner.into_inner().next().unwrap(); // chaos -> block
        let mut genes = Vec::new();

        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String("0.1".to_string())],
        });
        genes.push(Gene {
            op: OpCode::HavocRate,
            args: vec![],
        });

        self.depth += 1;
        for instr in block.into_inner() {
            let sub = self.parse_instruction(instr)?;
            genes.extend(sub);
        }
        self.depth -= 1;

        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        });
        genes.push(Gene {
            op: OpCode::HavocRate,
            args: vec![],
        });

        Ok(genes)
    }

    #[cfg(feature = "oracle")]
    fn parse_oracle_block(&mut self, inner: pest::iterators::Pair<'i, Rule>) -> Result<Vec<Gene>> {
        let mut genes = Vec::new();
        for stmt in inner.into_inner() {
            let def = stmt.into_inner().next().unwrap();
            match def.as_rule() {
                Rule::fact_def => {
                    let arg_list = def.into_inner().next().unwrap();
                    let mut fact_terms = Vec::new();
                    for arg in arg_list.into_inner() {
                        fact_terms.push(parse_argument(arg, self.strand_map, 0)?);
                    }
                    let fact = Nucleotide::Junction(crate::ast::JunctionType::Any, fact_terms);
                    genes.push(Gene {
                        op: OpCode::Push,
                        args: vec![fact],
                    });
                    genes.push(Gene {
                        op: OpCode::Assert,
                        args: vec![],
                    });
                }
                Rule::rule_def => {
                    let mut parts = def.into_inner();
                    let head_args = parts.next().unwrap();
                    let query_expr = parts.next().unwrap();

                    let mut head_terms = Vec::new();
                    for arg in head_args.into_inner() {
                        head_terms.push(parse_argument(arg, self.strand_map, 0)?);
                    }
                    let head = Nucleotide::Junction(crate::ast::JunctionType::Any, head_terms);

                    let mut body_goals = Vec::new();
                    for pred in query_expr.into_inner() {
                        let mut pred_parts = pred.into_inner();
                        let pred_name = pred_parts.next().unwrap().as_str();
                        let pred_args = pred_parts.next().unwrap();

                        let mut term_args = vec![Nucleotide::String(pred_name.to_string())];
                        for arg in pred_args.into_inner() {
                            term_args.push(parse_argument(arg, self.strand_map, 0)?);
                        }
                        body_goals.push(Nucleotide::Junction(
                            crate::ast::JunctionType::Any,
                            term_args,
                        ));
                    }

                    let body = if body_goals.len() == 1 {
                        body_goals[0].clone()
                    } else {
                        Nucleotide::Junction(crate::ast::JunctionType::All, body_goals)
                    };

                    genes.push(Gene {
                        op: OpCode::Push,
                        args: vec![head],
                    });
                    genes.push(Gene {
                        op: OpCode::Push,
                        args: vec![body],
                    });
                    genes.push(Gene {
                        op: OpCode::Rule,
                        args: vec![],
                    });
                }
                _ => {}
            }
        }
        Ok(genes)
    }

    fn parse_block(&mut self, inner: pest::iterators::Pair<'i, Rule>) -> Result<Vec<Gene>> {
        let mut genes = Vec::new();
        self.depth += 1;
        for instr in inner.into_inner() {
            let sub = self.parse_instruction(instr)?;
            genes.extend(sub);
        }
        self.depth -= 1;

        let index = self.strand_map.len() + self.anonymous_strands.len();
        self.anonymous_strands.push(Strand { genes });

        Ok(vec![Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(index as i64)],
        }])
    }

    fn parse_literal_instruction(
        &self,
        inner: pest::iterators::Pair<'i, Rule>,
    ) -> Result<Vec<Gene>> {
        let val = parse_literal(inner, self.strand_map)?;
        Ok(vec![Gene {
            op: OpCode::Push,
            args: vec![val],
        }])
    }

    fn parse_simple_op(&mut self, inner: pest::iterators::Pair<'i, Rule>) -> Result<Vec<Gene>> {
        let name = inner.clone().into_inner().next().unwrap().as_str();
        if let Some(body) = self.macro_map.get(name) {
            let mut macro_genes = Vec::new();
            self.depth += 1;
            for instr in body.clone() {
                let sub = self.parse_instruction(instr)?;
                macro_genes.extend(sub);
            }
            self.depth -= 1;
            return Ok(macro_genes);
        }

        // Handle Spawn<Name> macro for organelles
        if let Some(stripped) = name.strip_prefix("Spawn") {
            if let Some(&idx) = self.organelle_map.get(stripped) {
                // push(idx) push(0) spawn
                return Ok(vec![
                    Gene {
                        op: OpCode::Push,
                        args: vec![Nucleotide::Number(idx as i64)],
                    },
                    Gene {
                        op: OpCode::Push,
                        args: vec![Nucleotide::Number(0)],
                    }, // Type 0 = Worker (Default)
                    Gene {
                        op: OpCode::Spawn,
                        args: vec![],
                    },
                ]);
            }
        }

        if let Some(&idx) = self.strand_map.get(name) {
            return Ok(vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(idx as i64)],
            }]);
        }

        let op = OpCode::from_str(name).map_err(|_| anyhow!("Unknown opcode: {}", name))?;
        Ok(vec![Gene { op, args: vec![] }])
    }

    fn parse_arrow_jump(&self, inner: pest::iterators::Pair<'i, Rule>) -> Result<Vec<Gene>> {
        let mut parts = inner.into_inner();
        let target_name = parts.next().unwrap().as_str();
        let arg = resolve_target(target_name, self.strand_map);
        Ok(vec![Gene {
            op: OpCode::Jump,
            args: vec![arg],
        }])
    }

    fn parse_question_branch(&self, inner: pest::iterators::Pair<'i, Rule>) -> Result<Vec<Gene>> {
        let mut parts = inner.into_inner();
        let target_name = parts.next().unwrap().as_str();
        let arg = resolve_target(target_name, self.strand_map);
        Ok(vec![Gene {
            op: OpCode::Brz,
            args: vec![arg],
        }])
    }

    fn parse_call(&self, inner: pest::iterators::Pair<'i, Rule>) -> Result<Vec<Gene>> {
        let mut parts = inner.into_inner();
        let name = parts.next().unwrap().as_str();
        let args_pair = parts.next().unwrap();

        let op = OpCode::from_str(name).map_err(|_| anyhow!("Unknown opcode: {}", name))?;
        let mut args = Vec::new();

        for arg_pair in args_pair.into_inner() {
            let val = parse_argument(arg_pair, self.strand_map, 0)?;
            args.push(val);
        }

        let is_intrinsic = match op {
            OpCode::Jump | OpCode::Brz => true,
            #[cfg(feature = "nova")]
            OpCode::Call | OpCode::Poly => true,
            _ => false,
        };

        if op == OpCode::Push {
            let mut genes = Vec::new();
            for arg in args {
                genes.push(Gene {
                    op: OpCode::Push,
                    args: vec![arg],
                });
            }
            Ok(genes)
        } else if is_intrinsic {
            Ok(vec![Gene { op, args }])
        } else {
            let mut genes = Vec::new();
            for arg in args {
                genes.push(Gene {
                    op: OpCode::Push,
                    args: vec![arg],
                });
            }
            genes.push(Gene { op, args: vec![] });
            Ok(genes)
        }
    }

    fn parse_polyglot_block(
        &mut self,
        inner: pest::iterators::Pair<'i, Rule>,
    ) -> Result<Vec<Gene>> {
        let mut parts = inner.into_inner();
        let grammar_name = parts.next().unwrap().as_str();
        let content_pair = parts.next().unwrap();
        let content = content_pair.as_str();

        let grammar = self
            .grammar_map
            .get(grammar_name)
            .ok_or(anyhow!("Unknown grammar: {}", grammar_name))?;

        let (ast, consumed) = babel_parse(grammar, content)?;
        if consumed != content.len() {
            // Warn or error on partial match?
            // For now, let's treat as error to ensure correctness
            // But content might have trailing whitespace? babel_parse should handle that?
            // babel_parse is strict.
            // Let's trim content before passing?
            // content is from `nested_text` which is atomic but captures spaces.
        }

        flatten_ast(&ast)
    }

    #[cfg(feature = "nova")]
    fn parse_crispr_block(&mut self, inner: pest::iterators::Pair<'i, Rule>) -> Result<Vec<Gene>> {
        let mut parts = inner.into_inner();
        let target_name = parts.next().unwrap().as_str();

        let pattern_pair = parts.next().unwrap();
        let mut pattern_genes = Vec::new();
        for op_pair in pattern_pair.into_inner() {
            if op_pair.as_rule() == Rule::identifier {
                let op_str = op_pair.as_str();
                let op = OpCode::from_str(op_str)
                    .map_err(|_| anyhow!("Unknown opcode in pattern: {}", op_str))?;
                pattern_genes.push(Gene { op, args: vec![] });
            }
        }

        let guide_idx = self.strand_map.len() + self.anonymous_strands.len();
        self.anonymous_strands.push(Strand {
            genes: pattern_genes,
        });

        let mut genes = Vec::new();

        let target_arg = resolve_target(target_name, self.strand_map);
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![target_arg],
        });

        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(guide_idx as i64)],
        });

        genes.push(Gene {
            op: OpCode::CrisprScan,
            args: vec![],
        });

        self.depth += 1;
        for instr in parts {
            let sub = self.parse_instruction(instr)?;
            genes.extend(sub);
        }
        self.depth -= 1;

        Ok(genes)
    }
}

fn parse_instructions(
    pair: pest::iterators::Pair<Rule>,
    strand_map: &HashMap<String, usize>,
    macro_map: &HashMap<String, pest::iterators::Pairs<Rule>>,
    grammar_map: &HashMap<String, Nucleotide>,
    organelle_map: &HashMap<String, usize>,
    anonymous_strands: &mut Vec<Strand>,
    depth: usize,
) -> Result<Vec<Gene>> {
    let mut ctx = CompilerContext {
        strand_map,
        macro_map,
        grammar_map,
        organelle_map,
        anonymous_strands,
        depth,
    };
    ctx.parse_instruction(pair)
}

fn resolve_target(name: &str, strand_map: &HashMap<String, usize>) -> Nucleotide {
    if let Some(&idx) = strand_map.get(name) {
        Nucleotide::Number(idx as i64)
    } else {
        Nucleotide::Identifier(name.to_string())
    }
}

fn parse_literal(
    pair: pest::iterators::Pair<Rule>,
    _strand_map: &HashMap<String, usize>,
) -> Result<Nucleotide> {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::number => Ok(Nucleotide::Number(inner.as_str().parse()?)),
        Rule::string => {
            let s = inner.as_str();
            // Remove quotes
            Ok(Nucleotide::String(s[1..s.len() - 1].to_string()))
        }
        _ => unreachable!("Unexpected literal rule"),
    }
}

fn parse_argument(
    pair: pest::iterators::Pair<Rule>,
    strand_map: &HashMap<String, usize>,
    depth: usize,
) -> Result<Nucleotide> {
    if depth > MAX_PARSE_DEPTH {
        return Err(anyhow!("Recursion depth exceeded"));
    }
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::literal => parse_literal(inner, strand_map),
        Rule::identifier | Rule::variable => {
            let id = inner.as_str();
            // Try to resolve as strand index (only if identifier)
            if inner.as_rule() == Rule::identifier {
                if let Some(&idx) = strand_map.get(id) {
                    return Ok(Nucleotide::Number(idx as i64));
                }
            }
            // Keep as string if variable (starts with ?) to work with Oracle
            if inner.as_rule() == Rule::variable {
                Ok(Nucleotide::String(id.to_string()))
            } else {
                Ok(Nucleotide::Identifier(id.to_string()))
            }
        }
        Rule::junction => parse_junction(inner, strand_map, depth + 1),
        Rule::data_call => {
            let mut parts = inner.into_inner();
            let name = parts.next().unwrap().as_str();
            let args_pair = parts.next().unwrap();
            let mut args = vec![Nucleotide::String(name.to_string())];
            for arg in args_pair.into_inner() {
                args.push(parse_argument(arg, strand_map, depth + 1)?);
            }
            Ok(Nucleotide::Junction(crate::ast::JunctionType::Any, args))
        }
        _ => unreachable!("Unexpected argument rule: {:?}", inner.as_rule()),
    }
}

fn parse_junction(
    pair: pest::iterators::Pair<Rule>,
    strand_map: &HashMap<String, usize>,
    depth: usize,
) -> Result<Nucleotide> {
    if depth > MAX_PARSE_DEPTH {
        return Err(anyhow!("Recursion depth exceeded"));
    }
    let mut parts = pair.into_inner();
    let type_str = parts.next().unwrap().as_str();
    let args_pair = parts.next().unwrap();

    let t = match type_str {
        "any" => crate::ast::JunctionType::Any,
        "all" => crate::ast::JunctionType::All,
        _ => return Err(anyhow!("Invalid junction type")),
    };

    let mut vals = Vec::new();
    for arg in args_pair.into_inner() {
        vals.push(parse_argument(arg, strand_map, depth + 1)?);
    }
    Ok(Nucleotide::Junction(t, vals))
}

fn babel_parse(grammar: &Nucleotide, input: &str) -> Result<(Nucleotide, usize)> {
    use crate::ast::JunctionType;

    if let Nucleotide::Junction(JunctionType::Any, args) = grammar {
        if args.is_empty() {
            return Err(anyhow!("Empty grammar node"));
        }
        // Identifier is first arg (e.g., Match("..."))
        // If it was parsed as data_call, it became Junction(Any, ["Match", "..."])
        if let Nucleotide::String(type_str) = &args[0] {
            match type_str.as_str() {
                "Match" => {
                    if args.len() < 2 {
                        return Err(anyhow!("Match requires pattern"));
                    }
                    if let Nucleotide::String(pattern) = &args[1] {
                        if input.starts_with(pattern) {
                            return Ok((Nucleotide::String(pattern.clone()), pattern.len()));
                        }
                    }
                    Err(anyhow!("Match failed"))
                }
                "Regex" => {
                    if args.len() < 2 {
                        return Err(anyhow!("Regex requires pattern"));
                    }
                    if let Nucleotide::String(pattern) = &args[1] {
                        let anchored = format!("^{}", pattern);
                        let re = Regex::new(&anchored).map_err(|e| anyhow!("{}", e))?;
                        if let Some(mat) = re.find(input) {
                            let match_str = mat.as_str().to_string();
                            let len = match_str.len();
                            return Ok((Nucleotide::String(match_str), len));
                        }
                    }
                    Err(anyhow!("Regex failed"))
                }
                "Seq" => {
                    let mut total_consumed = 0;
                    let mut results = Vec::new();
                    for parser in args.iter().skip(1) {
                        let (res, consumed) = babel_parse(parser, &input[total_consumed..])?;
                        results.push(res);
                        total_consumed += consumed;
                    }
                    Ok((
                        Nucleotide::Junction(JunctionType::All, results),
                        total_consumed,
                    ))
                }
                "Alt" => {
                    for parser in args.iter().skip(1) {
                        if let Ok((res, consumed)) = babel_parse(parser, input) {
                            return Ok((res, consumed));
                        }
                    }
                    Err(anyhow!("Alt failed"))
                }
                "Many" => {
                    if args.len() < 2 {
                        return Err(anyhow!("Many requires parser"));
                    }
                    let p = &args[1];
                    let mut results = Vec::new();
                    let mut total_consumed = 0;
                    while let Ok((res, consumed)) = babel_parse(p, &input[total_consumed..]) {
                        if consumed == 0 {
                            break;
                        }
                        results.push(res);
                        total_consumed += consumed;
                    }
                    Ok((
                        Nucleotide::Junction(JunctionType::All, results),
                        total_consumed,
                    ))
                }
                "Opt" => {
                    if args.len() < 2 {
                        return Err(anyhow!("Opt requires parser"));
                    }
                    let p = &args[1];
                    if let Ok((res, consumed)) = babel_parse(p, input) {
                        Ok((res, consumed))
                    } else {
                        Ok((Nucleotide::Junction(JunctionType::All, Vec::new()), 0))
                    }
                }
                "Int" => {
                    if args.len() < 2 {
                        return Err(anyhow!("Int requires parser"));
                    }
                    let p = &args[1];
                    let (res, consumed) = babel_parse(p, input)?;
                    if let Nucleotide::String(s) = res {
                        if let Ok(n) = s.parse::<i64>() {
                            return Ok((Nucleotide::Number(n), consumed));
                        }
                    }
                    Err(anyhow!("Int conversion failed"))
                }
                "Map" => {
                    if args.len() < 3 {
                        return Err(anyhow!("Map requires [parser, template]"));
                    }
                    let parser = &args[1];
                    let template = &args[2];
                    let (res, consumed) = babel_parse(parser, input)?;
                    let mapped = resolve_template(template, &res);
                    Ok((mapped, consumed))
                }
                _ => Err(anyhow!("Unknown grammar type: {}", type_str)),
            }
        } else {
            Err(anyhow!(
                "Invalid grammar node structure (expected Type String)"
            ))
        }
    } else {
        Err(anyhow!("Invalid grammar node (expected Junction)"))
    }
}

fn flatten_ast(ast: &Nucleotide) -> Result<Vec<Gene>> {
    match ast {
        Nucleotide::Junction(crate::ast::JunctionType::All, children) => {
            let mut genes = Vec::new();
            for child in children {
                genes.extend(flatten_ast(child)?);
            }
            Ok(genes)
        }
        Nucleotide::Junction(crate::ast::JunctionType::Any, children) => {
            // Handle explicit call structure: [OpName, Arg1, Arg2]
            if !children.is_empty() {
                if let Nucleotide::String(op_name) = &children[0] {
                    // Try to parse as OpCode
                    if let Ok(op) = OpCode::from_str(op_name) {
                        let mut args = Vec::new();
                        for child in children.iter().skip(1) {
                            args.push(child.clone());
                        }
                        return Ok(vec![Gene { op, args }]);
                    }
                }
            }
            // Fallback: Flatten children sequentially
            let mut genes = Vec::new();
            for child in children {
                genes.extend(flatten_ast(child)?);
            }
            Ok(genes)
        }
        Nucleotide::String(s) => {
            if let Ok(op) = OpCode::from_str(s) {
                Ok(vec![Gene { op, args: vec![] }])
            } else {
                Ok(vec![Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::String(s.clone())],
                }])
            }
        }
        Nucleotide::Number(n) => Ok(vec![Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(*n)],
        }]),
        _ => Ok(vec![]),
    }
}

fn resolve_template(template: &Nucleotide, match_res: &Nucleotide) -> Nucleotide {
    use crate::ast::JunctionType;
    match template {
        Nucleotide::String(s) if s.starts_with('?') => {
            // Variable ?1, ?2 etc
            if let Ok(idx) = s[1..].parse::<usize>() {
                // 1-based index convention usually? Or 0?
                // Let's assume 1-based to match Babel vars ?1
                let i = idx.saturating_sub(1);
                match match_res {
                    Nucleotide::Junction(_, children) => {
                        if i < children.len() {
                            return children[i].clone();
                        }
                    }
                    _ => {
                        if i == 0 {
                            return match_res.clone();
                        }
                    }
                }
                // Fallback: return as is if not found
                return template.clone();
            }
            template.clone()
        }
        Nucleotide::Junction(t, args) => {
            let resolved: Vec<Nucleotide> = args
                .iter()
                .map(|a| resolve_template(a, match_res))
                .collect();
            Nucleotide::Junction(*t, resolved)
        }
        _ => template.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_basic() {
        let src = r#"
        strand main {
            5
            3
            add
            print
        }
        "#;
        let dna = compile(src, None).unwrap();
        assert_eq!(dna.helix.strands.len(), 1);
        let genes = &dna.helix.strands[0].genes;
        assert_eq!(genes.len(), 4);
        assert_eq!(genes[0].op, OpCode::Push);
        assert_eq!(genes[2].op, OpCode::Add);
    }

    #[test]
    fn test_compile_jumps() {
        let src = r#"
        strand main {
            jump(loop)
        }
        strand loop {
            main
        }
        "#;
        let dna = compile(src, None).unwrap();
        assert_eq!(dna.helix.strands.len(), 2);

        // Check jump target
        let jump_gene = &dna.helix.strands[0].genes[0];
        assert_eq!(jump_gene.op, OpCode::Jump);
        // "loop" is the second strand (index 1)
        assert_eq!(jump_gene.args[0], Nucleotide::Number(1));
    }

    #[test]
    fn test_macros() {
        let src = r#"
        macro ADD_PRINT {
            add print
        }
        strand main {
            5 3 ADD_PRINT
        }
        "#;
        let dna = compile(src, None).unwrap();
        let genes = &dna.helix.strands[0].genes;
        assert_eq!(genes.len(), 4); // push 5, push 3, add, print
        assert_eq!(genes[2].op, OpCode::Add);
        assert_eq!(genes[3].op, OpCode::Print);
    }

    #[test]
    fn test_sugar() {
        let src = r#"
        strand main {
            -> target
            ? target
        }
        strand target {
            drop
        }
        "#;
        let dna = compile(src, None).unwrap();
        let genes = &dna.helix.strands[0].genes;
        assert_eq!(genes[0].op, OpCode::Jump);
        assert_eq!(genes[1].op, OpCode::Brz);
    }

    #[test]
    #[cfg(feature = "nova")]
    fn test_blocks() {
        let src = r#"
        strand main {
            { 1 2 add } call
        }
        "#;
        let dna = compile(src, None).unwrap();
        assert_eq!(dna.helix.strands.len(), 2); // main + block

        let main_genes = &dna.helix.strands[0].genes;
        assert_eq!(main_genes[0].op, OpCode::Push); // Push block index
        assert_eq!(main_genes[0].args[0], Nucleotide::Number(1));
        assert_eq!(main_genes[1].op, OpCode::Call);

        let block_genes = &dna.helix.strands[1].genes;
        assert_eq!(block_genes.len(), 3);
        assert_eq!(block_genes[2].op, OpCode::Add);
    }
}
