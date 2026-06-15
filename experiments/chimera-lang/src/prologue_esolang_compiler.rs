//! Compilation pipeline for the PrologueEsolang esoteric language.
//!
//! Exposes parsers that translate multi-paradigm script blocks (e.g., Forth, Lisp, Orca)
//! into a standard unified `Dna` construct.

use anyhow::{anyhow, Result};
use pest::Parser;
use pest_derive::Parser;
use std::str::FromStr;

use crate::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
use crate::opcode::OpCode;

/// The parser for the PrologueEsolang esoteric language.
///
/// Handles the syntax mapping for multiple esoteric blocks (like Forth, Orca, Lisp)
/// into the unified AST based on rules within `prologue_esolang_grammar.pest`.
#[derive(Parser)]
#[allow(missing_docs)]
#[grammar = "prologue_esolang_grammar.pest"]
pub struct PrologueEsolangParser;

/// Evaluates an esoteric PrologueEsolang script and synthesizes a biological [`Dna`] structure.
///
/// PrologueEsolang acts as the "Mad Scientist" layer, allowing code mixing from completely different paradigms like Forth, Raku, and Orca in one file.
/// This compiler translates those varied esoteric syntaxes into a unified sequence of standard Chimera [`Gene`] structures.
///
/// Helper function to extract block content while preserving spatial whitespace.
/// This is critical for 2D esolangs like Befunge and Piet where leading spaces matter.
fn extract_block_content_preserve_whitespace(block_str: &str, block_type: &str) -> String {
    let mut content = block_str.trim_start_matches(block_type).trim_start();
    if content.starts_with('{') && content.ends_with('}') {
        content = &content[1..content.len() - 1];
    }

    // Strip only the first newline to keep subsequent lines indented properly
    let res = content
        .strip_prefix("\r\n")
        .unwrap_or_else(|| content.strip_prefix('\n').unwrap_or(content));

    // Trailing whitespace can be safely removed
    res.trim_end().to_string()
}

/// # Examples
///
/// ```
/// use chimera_lang::prologue_esolang_compiler::compile;
/// use chimera_lang::opcode::OpCode;
///
/// // Crossing a Forth instruction block into our biological VM.
/// let source = r#"
/// forth {
///     5 3 add
/// }
/// "#;
/// let dna = compile(source).unwrap();
/// assert_eq!(dna.helix.strands.len(), 1);
/// assert_eq!(dna.helix.strands[0].genes[2].op, OpCode::Add);
/// ```
///
/// # Details
/// - **Paradigms**: Supports blocks defined via `forth`, `raku`, `orca`, `elektra`, `prolog`, and `genetics`.
/// - **Integration**: All blocks are compiled into a single flattened [`Strand`] in the returned DNA.
/// - **Panics**: Returns a `Result::Err` if a block contains syntax invalid for its declared paradigm.
///
/// For more details on the VM execution, see [`crate::vm::ChimeraVM`].
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
pub fn compile(source: &str) -> Result<Dna> {
    let mut pairs = PrologueEsolangParser::parse(Rule::program, source)?;
    let program = pairs.next().ok_or(anyhow!("No program found"))?;

    let mut genes = Vec::new();

    for section in program.into_inner() {
        if section.as_rule() == Rule::EOI {
            break;
        }

        if section.as_rule() != Rule::section {
            continue;
        }

        let inner_block = section.into_inner().next().unwrap();
        match inner_block.as_rule() {
            #[cfg(feature = "git")]
            Rule::git_associates_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_git_associates_instr(instr)?);
                }
            }
            #[cfg(not(feature = "git"))]
            Rule::git_associates_block => {}
            #[cfg(feature = "git")]
            Rule::git_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_git_instr(instr)?);
                }
            }
            #[cfg(not(feature = "git"))]
            Rule::git_block => {}
            Rule::forth_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_forth_instr(instr)?);
                }
            }
            #[cfg(feature = "nova")]
            Rule::raku_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_raku_instr(instr)?);
                }
            }
            #[cfg(not(feature = "nova"))]
            Rule::raku_block => {}
            Rule::orca_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_orca_instr(instr)?);
                }
            }
            Rule::elektra_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_elektra_instr(instr)?);
                }
            }
            Rule::prolog_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_prolog_instr(instr)?);
                }
            }
            Rule::genetics_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_genetics_instr(instr)?);
                }
            }
            Rule::lisp_block => {
                // The lisp block has everything between { and } as a single chunk
                let content = inner_block.as_str();
                // We strip off the "lisp {" and "}" parts safely by taking the inner span
                let mut content = content.trim_start_matches("lisp").trim();
                if content.starts_with('{') && content.ends_with('}') {
                    content = &content[1..content.len() - 1];
                }
                genes.extend(crate::lisp::compile_fragment(content)?);
            }
            Rule::befunge_block => {
                let content =
                    extract_block_content_preserve_whitespace(inner_block.as_str(), "befunge");
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(content)]));
                genes.push(Gene::new(OpCode::Befunge, vec![]));
            }
            Rule::brainfuck_block => {
                let content =
                    extract_block_content_preserve_whitespace(inner_block.as_str(), "brainfuck");
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(content)]));
                genes.push(Gene::new(
                    OpCode::Push,
                    vec![Nucleotide::String("".to_string())],
                )); // Empty input
                genes.push(Gene::new(OpCode::Brainfuck, vec![]));
            }
            Rule::tui_block => {
                let content =
                    extract_block_content_preserve_whitespace(inner_block.as_str(), "tui");
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(content)]));
                genes.push(Gene::new(OpCode::TuiDraw, vec![]));
            }
            Rule::tui_mod_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_tui_mod_instr(instr)?);
                }
            }
            Rule::mosaic_block => {
                let content =
                    extract_block_content_preserve_whitespace(inner_block.as_str(), "mosaic");
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(content)]));
                genes.push(Gene::new(OpCode::MosaicDraw, vec![]));
            }
            Rule::piet_block => {
                let content =
                    extract_block_content_preserve_whitespace(inner_block.as_str(), "piet");
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(content)]));
                genes.push(Gene::new(OpCode::Piet, vec![]));
            }
            Rule::acoustic_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_acoustic_instr(instr)?);
                }
            }
            Rule::regex_block => {
                let content =
                    extract_block_content_preserve_whitespace(inner_block.as_str(), "regex");
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(content)]));
                genes.push(Gene::new(OpCode::ParserRegex, vec![]));
            }
            Rule::chaos_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_chaos_instr(instr)?);
                }
            }
            Rule::esolang_block => {
                let content =
                    extract_block_content_preserve_whitespace(inner_block.as_str(), "esolang");
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(content)]));
                #[cfg(feature = "nova")]
                genes.push(Gene::new(OpCode::Eval, vec![]));
            }
            Rule::madness_block => {
                let content =
                    extract_block_content_preserve_whitespace(inner_block.as_str(), "madness");
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(content)]));
                genes.push(Gene::new(OpCode::PrologueEsolang, vec![]));
            }
            Rule::prologue_esolang_block => {
                let content = extract_block_content_preserve_whitespace(
                    inner_block.as_str(),
                    "prologue_esolang",
                );
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(content)]));
                genes.push(Gene::new(OpCode::TuiDraw, vec![]));
                genes.push(Gene::new(OpCode::PrologueEsolang, vec![]));
            }
            Rule::miller_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_miller_instr(instr)?);
                }
            }
            Rule::hyper_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_hyper_instr(instr)?);
                }
            }
            Rule::physics_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_physics_instr(instr)?);
                }
            }
            Rule::ferrous_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_ferrous_instr(instr)?);
                }
            }
            Rule::tardis_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_tardis_instr(instr)?);
                }
            }
            Rule::pachinko_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_pachinko_instr(instr)?);
                }
            }
            Rule::automaton_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_automaton_instr(instr)?);
                }
            }
            Rule::syncopation_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_syncopation_instr(instr)?);
                }
            }
            Rule::choreography_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_choreography_instr(instr)?);
                }
            }
            Rule::quipu_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_quipu_instr(instr)?);
                }
            }
            Rule::origami_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_origami_instr(instr)?);
                }
            }
            Rule::weave_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_weave_instr(instr)?);
                }
            }
            Rule::fluid_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_fluid_instr(instr)?);
                }
            }
            Rule::flocking_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_flocking_instr(instr)?);
                }
            }

            Rule::gray_scott_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_gray_scott_instr(instr)?);
                }
            }
            Rule::locus_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_locus_instr(instr)?);
                }
            }
            Rule::neuro_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_neuro_instr(instr)?);
                }
            }
            Rule::platter_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_platter_instr(instr)?);
                }
            }
            Rule::market_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_market_instr(instr)?);
                }
            }
            Rule::poincare_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_poincare_instr(instr)?);
                }
            }
            Rule::reactor_block => {
                let content =
                    extract_block_content_preserve_whitespace(inner_block.as_str(), "reactor");
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(content)]));
                genes.push(Gene::new(OpCode::Reactor, vec![]));
            }
            _ => {}
        }
    }

    Ok(Dna {
        evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    })
}

fn compile_tui_mod_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::identifier => {
            let op = inner.as_str().to_ascii_lowercase();
            if op == "glitch" {
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(0)]));
                genes.push(Gene::new(OpCode::TuiMod, vec![]));
            } else if op == "shake" {
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(1)]));
                genes.push(Gene::new(OpCode::TuiMod, vec![]));
            } else if op == "message" {
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(2)]));
                genes.push(Gene::new(OpCode::TuiMod, vec![]));
            } else {
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(op)]));
            }
        }
        Rule::number => {
            let n: i64 = inner.as_str().parse()?;
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(n)]));
        }
        Rule::string => {
            let s = inner.as_str();
            let content = if s.starts_with('"') && s.ends_with('"') {
                &s[1..s.len() - 1]
            } else {
                s
            };
            genes.push(Gene::new(
                OpCode::Push,
                vec![Nucleotide::String(content.to_string())],
            ));
        }
        _ => {}
    }

    Ok(genes)
}

fn compile_choreography_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::identifier => {
            let op = inner.as_str().to_ascii_lowercase();
            if op == "simulate" {
                genes.push(Gene::new(OpCode::Choreography, vec![]));
            } else if let Ok(opcode) = OpCode::from_str(&op) {
                genes.push(Gene::new(opcode, vec![]));
            } else {
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(op)]));
            }
        }
        Rule::number => {
            let num = inner.as_str().parse::<i64>().unwrap();
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(num)]));
        }
        Rule::string => {
            let s = inner.as_str().trim_matches('"').to_string();
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(s)]));
        }
        _ => {}
    }

    Ok(genes)
}

fn compile_syncopation_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::identifier => {
            let op = inner.as_str().to_ascii_lowercase();
            if op == "simulate" {
                genes.push(Gene::new(OpCode::Syncopation, vec![]));
            } else if let Ok(opcode) = OpCode::from_str(&op) {
                genes.push(Gene::new(opcode, vec![]));
            } else {
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(op)]));
            }
        }
        Rule::number => {
            let n: i64 = inner.as_str().parse()?;
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(n)]));
        }
        Rule::string => {
            let s = inner.as_str();
            let content = &s[1..s.len() - 1]; // Strip quotes
            genes.push(Gene::new(
                OpCode::Push,
                vec![Nucleotide::String(content.to_string())],
            ));
        }
        _ => {}
    }

    Ok(genes)
}

fn compile_pachinko_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::identifier => {
            let op = inner.as_str().to_ascii_lowercase();
            if op == "simulate" {
                genes.push(Gene::new(OpCode::Pachinko, vec![]));
            } else if let Ok(opcode) = OpCode::from_str(&op) {
                genes.push(Gene::new(opcode, vec![]));
            } else {
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(op)]));
            }
        }
        Rule::number => {
            let n: i64 = inner.as_str().parse()?;
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(n)]));
        }
        Rule::string => {
            let s = inner.as_str();
            genes.push(Gene::new(
                OpCode::Push,
                vec![Nucleotide::String(s[1..s.len() - 1].to_string())],
            ));
        }
        _ => {}
    }

    Ok(genes)
}

#[cfg(feature = "git")]
fn compile_git_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::identifier => {
            let op = inner.as_str().to_ascii_lowercase();
            if op == "ancestry" {
                genes.push(Gene::new(OpCode::Ancestry, vec![]));
            } else if op == "excavate" {
                genes.push(Gene::new(OpCode::Excavate, vec![]));
            } else if op == "evolution" {
                genes.push(Gene::new(OpCode::Evolution, vec![]));
            } else if let Ok(opcode) = OpCode::from_str(&op) {
                genes.push(Gene::new(opcode, vec![]));
            } else {
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(op)]));
            }
        }
        Rule::number => {
            let n: i64 = inner.as_str().parse()?;
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(n)]));
        }
        Rule::string => {
            let s = inner.as_str();
            let s = &s[1..s.len() - 1]; // Strip quotes
            genes.push(Gene::new(
                OpCode::Push,
                vec![Nucleotide::String(s.to_string())],
            ));
        }
        _ => {}
    }

    Ok(genes)
}

fn compile_forth_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::number => {
            let n: i64 = inner.as_str().parse()?;
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(n)]));
        }
        Rule::string => {
            let s = inner.as_str();
            let content = s[1..s.len() - 1].to_string();
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(content)]));
        }
        Rule::identifier => {
            let id = inner.as_str();
            if id.eq_ignore_ascii_case("print") {
                genes.push(Gene::new(OpCode::Print, vec![]));
            } else if id.eq_ignore_ascii_case("add") {
                genes.push(Gene::new(OpCode::Add, vec![]));
            } else if id.eq_ignore_ascii_case("sub") {
                genes.push(Gene::new(OpCode::Sub, vec![]));
            } else if let Ok(op) = OpCode::from_str(id) {
                genes.push(Gene::new(op, vec![]));
            } else {
                genes.push(Gene::new(
                    OpCode::Push,
                    vec![Nucleotide::String(id.to_string())],
                ));
            }
        }
        _ => {}
    }
    Ok(genes)
}

fn compile_automaton_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::identifier => {
            let op = inner.as_str().to_ascii_lowercase();
            if op == "simulate" {
                genes.push(Gene::new(OpCode::Automaton, vec![]));
            } else if let Ok(opcode) = OpCode::from_str(&op) {
                genes.push(Gene::new(opcode, vec![]));
            } else {
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(op)]));
            }
        }
        Rule::number => {
            let n: i64 = inner.as_str().parse()?;
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(n)]));
        }
        Rule::string => {
            let s = inner.as_str();
            let content = s[1..s.len() - 1].to_string();
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(content)]));
        }
        _ => {}
    }
    Ok(genes)
}

fn compile_fluid_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::identifier => {
            let op = inner.as_str().to_ascii_lowercase();
            if op == "fluid" {
                genes.push(Gene::new(OpCode::Fluid, vec![]));
            } else if let Ok(opcode) = OpCode::from_str(&op) {
                genes.push(Gene::new(opcode, vec![]));
            } else {
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(op)]));
            }
        }
        Rule::number => {
            let n: i64 = inner.as_str().parse()?;
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(n)]));
        }
        Rule::string => {
            let s = inner.as_str();
            genes.push(Gene::new(
                OpCode::Push,
                vec![Nucleotide::String(s[1..s.len() - 1].to_string())],
            ));
        }
        _ => {}
    }

    Ok(genes)
}

fn compile_ferrous_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::identifier => {
            let op = inner.as_str().to_ascii_lowercase();
            if op == "simulate" {
                genes.push(Gene::new(OpCode::FerrousCore, vec![]));
            } else if let Ok(opcode) = OpCode::from_str(&op) {
                genes.push(Gene::new(opcode, vec![]));
            } else {
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(op)]));
            }
        }
        Rule::number => {
            let n: i64 = inner.as_str().parse()?;
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(n)]));
        }
        Rule::string => {
            let s = inner.as_str();
            genes.push(Gene::new(
                OpCode::Push,
                vec![Nucleotide::String(s[1..s.len() - 1].to_string())],
            ));
        }
        _ => {}
    }

    Ok(genes)
}

fn compile_tardis_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::identifier => {
            let op = inner.as_str().to_ascii_lowercase();
            if op == "simulate" {
                genes.push(Gene::new(OpCode::Tardis, vec![]));
            } else if let Ok(opcode) = OpCode::from_str(&op) {
                genes.push(Gene::new(opcode, vec![]));
            } else {
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(op)]));
            }
        }
        Rule::number => {
            let n: i64 = inner.as_str().parse()?;
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(n)]));
        }
        Rule::string => {
            let s = inner.as_str();
            genes.push(Gene::new(
                OpCode::Push,
                vec![Nucleotide::String(s[1..s.len() - 1].to_string())],
            ));
        }
        _ => {}
    }

    Ok(genes)
}

fn compile_miller_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::identifier => {
            let op = inner.as_str().to_ascii_lowercase();
            if op == "simulate" {
                genes.push(Gene::new(OpCode::MillerLattice, vec![]));
            } else if let Ok(opcode) = OpCode::from_str(&op) {
                genes.push(Gene::new(opcode, vec![]));
            } else {
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(op)]));
            }
        }
        Rule::number => {
            let n: i64 = inner.as_str().parse()?;
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(n)]));
        }
        Rule::string => {
            let s = inner.as_str();
            genes.push(Gene::new(
                OpCode::Push,
                vec![Nucleotide::String(s[1..s.len() - 1].to_string())],
            ));
        }
        _ => {}
    }

    Ok(genes)
}

fn compile_hyper_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::identifier => {
            let op = inner.as_str().to_ascii_lowercase();
            if op == "simulate" {
                genes.push(Gene::new(OpCode::HyperSystem, vec![]));
            } else if let Ok(opcode) = OpCode::from_str(&op) {
                genes.push(Gene::new(opcode, vec![]));
            } else {
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(op)]));
            }
        }
        Rule::number => {
            let n: i64 = inner.as_str().parse()?;
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(n)]));
        }
        Rule::string => {
            let s = inner.as_str();
            genes.push(Gene::new(
                OpCode::Push,
                vec![Nucleotide::String(s[1..s.len() - 1].to_string())],
            ));
        }
        _ => {}
    }

    Ok(genes)
}

fn compile_physics_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::identifier => {
            let op = inner.as_str().to_ascii_lowercase();
            if op == "simulate" {
                genes.push(Gene::new(OpCode::PhysicsPbd, vec![]));
            } else if let Ok(opcode) = OpCode::from_str(&op) {
                genes.push(Gene::new(opcode, vec![]));
            } else {
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(op)]));
            }
        }
        Rule::number => {
            let n: i64 = inner.as_str().parse()?;
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(n)]));
        }
        Rule::string => {
            let s = inner.as_str();
            genes.push(Gene::new(
                OpCode::Push,
                vec![Nucleotide::String(s[1..s.len() - 1].to_string())],
            ));
        }
        _ => {}
    }

    Ok(genes)
}

fn compile_weave_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::identifier => {
            let op = inner.as_str().to_ascii_lowercase();
            if op == "weave" {
                genes.push(Gene::new(OpCode::Weave, vec![]));
            } else if op == "unravel" {
                genes.push(Gene::new(OpCode::Unravel, vec![]));
            } else if let Ok(opcode) = OpCode::from_str(&op) {
                genes.push(Gene::new(opcode, vec![]));
            } else {
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(op)]));
            }
        }
        Rule::number => {
            let n: i64 = inner.as_str().parse()?;
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(n)]));
        }
        Rule::string => {
            let mut s = inner.as_str().to_string();
            if s.starts_with('"') && s.ends_with('"') {
                s = s[1..s.len() - 1].to_string();
            }
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(s)]));
        }
        _ => {}
    }

    Ok(genes)
}

fn compile_poincare_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::identifier => {
            let op = inner.as_str().to_ascii_lowercase();
            if op == "hyperbolic" {
                genes.push(Gene::new(OpCode::Poincare, vec![]));
            } else if let Ok(opcode) = OpCode::from_str(&op) {
                genes.push(Gene::new(opcode, vec![]));
            } else {
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(op)]));
            }
        }
        Rule::number => {
            let n: i64 = inner.as_str().parse()?;
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(n)]));
        }
        Rule::string => {
            let s = inner.as_str();
            let content = s[1..s.len() - 1].to_string();
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(content)]));
        }
        _ => {}
    }
    Ok(genes)
}

fn compile_market_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::identifier => {
            let op = inner.as_str().to_ascii_lowercase();
            #[cfg(feature = "nova")]
            if op == "invest" {
                genes.push(Gene::new(OpCode::Invest, vec![]));
            } else if op == "divest" {
                genes.push(Gene::new(OpCode::Divest, vec![]));
            } else if op == "buy" {
                genes.push(Gene::new(OpCode::Buy, vec![]));
            } else if op == "offer" {
                genes.push(Gene::new(OpCode::Offer, vec![]));
            } else if op == "balance" {
                genes.push(Gene::new(OpCode::Balance, vec![]));
            } else if op == "ticker" {
                genes.push(Gene::new(OpCode::Ticker, vec![]));
            } else if let Ok(opcode) = OpCode::from_str(&op) {
                genes.push(Gene::new(opcode, vec![]));
            } else {
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(op)]));
            }
            #[cfg(not(feature = "nova"))]
            if let Ok(opcode) = OpCode::from_str(&op) {
                genes.push(Gene::new(opcode, vec![]));
            } else {
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(op)]));
            }
        }
        Rule::number => {
            let n: i64 = inner.as_str().parse()?;
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(n)]));
        }
        Rule::string => {
            let s = inner.as_str().trim_matches('"').to_string();
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(s)]));
        }
        _ => {}
    }
    Ok(genes)
}

fn compile_flocking_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::identifier => {
            let op = inner.as_str().to_ascii_lowercase();
            if op == "simulate" {
                genes.push(Gene::new(OpCode::Flock, vec![]));
            } else if let Ok(opcode) = OpCode::from_str(&op) {
                genes.push(Gene::new(opcode, vec![]));
            } else {
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(op)]));
            }
        }
        Rule::number => {
            let n: i64 = inner.as_str().parse()?;
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(n)]));
        }
        Rule::string => {
            let s = inner.as_str();
            let content = s[1..s.len() - 1].to_string();
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(content)]));
        }
        _ => {}
    }
    Ok(genes)
}

fn compile_origami_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::identifier => {
            let op = inner.as_str().to_ascii_lowercase();
            if op == "fold" {
                genes.push(Gene::new(OpCode::Origami, vec![]));
            } else if let Ok(opcode) = OpCode::from_str(&op) {
                genes.push(Gene::new(opcode, vec![]));
            } else {
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(op)]));
            }
        }
        Rule::number => {
            let n: i64 = inner.as_str().parse()?;
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(n)]));
        }
        Rule::string => {
            let s = inner.as_str();
            let content = s[1..s.len() - 1].to_string();
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(content)]));
        }
        _ => {}
    }
    Ok(genes)
}

fn compile_quipu_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::number => {
            let n: i64 = inner.as_str().parse()?;
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(n)]));
            genes.push(Gene::new(OpCode::Quipu, vec![]));
        }
        Rule::string => {
            let s = inner.as_str();
            let content = s[1..s.len() - 1].to_string();
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(content)]));
            genes.push(Gene::new(OpCode::Quipu, vec![]));
        }
        Rule::identifier => {
            let id = inner.as_str();
            match id {
                "tie" => genes.push(Gene::new(OpCode::Knot, vec![])),
                "untie" => genes.push(Gene::new(OpCode::Unknot, vec![])),
                "select" => genes.push(Gene::new(OpCode::Cord, vec![])),
                "read" => genes.push(Gene::new(OpCode::ReadCord, vec![])),
                "tangle" => genes.push(Gene::new(OpCode::Tangle, vec![])),
                _ => {
                    genes.push(Gene::new(
                        OpCode::Push,
                        vec![Nucleotide::String(id.to_string())],
                    ));
                    genes.push(Gene::new(OpCode::Quipu, vec![]));
                }
            }
        }
        _ => {}
    }
    Ok(genes)
}

fn compile_chaos_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::identifier => {
            let op = inner.as_str().to_ascii_lowercase();
            if op == "glitch" {
                genes.push(Gene::new(OpCode::Glitch, vec![]));
            } else if op == "chaos" {
                genes.push(Gene::new(OpCode::Chaos, vec![]));
            } else if op == "entropy" {
                genes.push(Gene::new(OpCode::EntropySurge, vec![]));
            } else if let Ok(opcode) = OpCode::from_str(&op) {
                genes.push(Gene::new(opcode, vec![]));
            } else {
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(op)]));
            }
        }
        Rule::number => {
            let n: i64 = inner.as_str().parse()?;
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(n)]));
        }
        Rule::string => {
            let s = inner.as_str();
            let content = s[1..s.len() - 1].to_string();
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(content)]));
        }
        _ => {}
    }
    Ok(genes)
}

#[cfg(feature = "nova")]
fn compile_raku_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    // hyper_instruction = { (">>" ~ operator ~ "<<") | string | number | identifier }
    let s = pair.as_str();
    if s.starts_with(">>") && s.ends_with("<<") {
        // Find inner
        let mut inner = pair.into_inner();
        let id = inner.next().unwrap().as_str();
        if id == "+" || id == "add" {
            genes.push(Gene::new(OpCode::HyperAdd, vec![]));
        } else if id == "-" || id == "sub" {
            genes.push(Gene::new(OpCode::HyperSub, vec![]));
        } else if id == "*" || id == "mul" {
            genes.push(Gene::new(OpCode::HyperMul, vec![]));
        } else if id == "/" || id == "div" {
            genes.push(Gene::new(OpCode::HyperDiv, vec![]));
        } else {
            // If it's a general operator
            genes.push(Gene::new(
                OpCode::Push,
                vec![Nucleotide::String(id.to_string())],
            ));
            genes.push(Gene::new(OpCode::ZipWith, vec![]));
        }
    } else {
        // Just normal instruction parse
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::number => {
                let n: i64 = inner.as_str().parse()?;
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(n)]));
            }
            Rule::string => {
                let string_val = inner.as_str();
                let content = string_val[1..string_val.len() - 1].to_string();
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(content)]));
            }
            Rule::identifier => {
                genes.push(Gene::new(
                    OpCode::Push,
                    vec![Nucleotide::String(inner.as_str().to_string())],
                ));
            }
            _ => {}
        }
    }
    Ok(genes)
}

fn compile_orca_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let mut inner = pair.into_inner();
    let op = inner.next().unwrap().as_str();
    let y: i64 = inner.next().unwrap().as_str().parse()?;
    let x: i64 = inner.next().unwrap().as_str().parse()?;

    // Push args and call Orca or related
    genes.push(Gene::new(
        OpCode::Push,
        vec![Nucleotide::String(op.to_string())],
    ));
    genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(y)]));
    genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(x)]));
    #[cfg(feature = "nova")]
    genes.push(Gene::new(OpCode::Orca, vec![]));
    Ok(genes)
}

fn compile_elektra_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let mut inner = pair.into_inner();
    let kind = inner.next().unwrap().as_str();
    let y: i64 = inner.next().unwrap().as_str().parse()?;
    let x: i64 = inner.next().unwrap().as_str().parse()?;

    if kind == "battery" {
        genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(9)])); // 9V default
        genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(y)]));
        genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(x)]));
        #[cfg(feature = "elektra")]
        genes.push(Gene::new(OpCode::Battery, vec![]));
    } else if kind == "ground" {
        genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(y)]));
        genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(x)]));
        #[cfg(feature = "elektra")]
        genes.push(Gene::new(OpCode::Ground, vec![]));
    } else {
        // generic
        genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(y)]));
        genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(x)]));
        if let Ok(op) = OpCode::from_str(kind) {
            genes.push(Gene::new(op, vec![]));
        } else {
            genes.push(Gene::new(
                OpCode::Push,
                vec![Nucleotide::String(kind.to_string())],
            ));
        }
    }
    Ok(genes)
}

fn compile_prolog_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str();
    let arg = inner.next().unwrap().as_str();

    let fact = Nucleotide::Junction(
        JunctionType::Any,
        vec![
            Nucleotide::String(name.to_string()),
            Nucleotide::String(arg.to_string()),
        ],
    );

    genes.push(Gene::new(OpCode::Push, vec![fact]));
    #[cfg(feature = "oracle")]
    #[cfg(feature = "oracle")]
    genes.push(Gene::new(OpCode::Assert, vec![]));
    Ok(genes)
}

fn compile_genetics_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let mut inner = pair.into_inner();
    let op = inner.next().unwrap().as_str();
    let arg1 = inner.next().unwrap().as_str();
    let arg2 = inner.next().unwrap().as_str();

    genes.push(Gene::new(
        OpCode::Push,
        vec![Nucleotide::String(arg1.to_string())],
    ));
    genes.push(Gene::new(
        OpCode::Push,
        vec![Nucleotide::String(arg2.to_string())],
    ));

    if op == "splice" {
        genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(0)])); // Splice method
        #[cfg(feature = "nova")]
        #[cfg(feature = "nova")]
        genes.push(Gene::new(OpCode::Splice, vec![]));
    } else if let Ok(opcode) = OpCode::from_str(&op.to_ascii_lowercase()) {
        genes.push(Gene::new(opcode, vec![]));
    } else {
        // Fallback for custom or unknown genetic opcodes
        genes.push(Gene::new(OpCode::Unknown(op.to_string()), vec![]));
    }

    Ok(genes)
}

#[cfg(feature = "resonance")]
fn compile_acoustic_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let mut inner = pair.into_inner();
    let op = inner.next().unwrap().as_str().to_ascii_lowercase();

    if op == "pluck" {
        let strength: i64 = inner.next().unwrap().as_str().parse()?;
        genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(strength)]));
        genes.push(Gene::new(OpCode::Pluck, vec![]));
    } else if op == "oscillate" {
        let freq: i64 = inner.next().unwrap().as_str().parse()?;
        let strength: i64 = inner.next().unwrap().as_str().parse()?;
        genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(freq)]));
        genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(strength)]));
        genes.push(Gene::new(OpCode::Oscillate, vec![]));
    }
    Ok(genes)
}

#[cfg(not(feature = "resonance"))]
fn compile_acoustic_instr(_pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    Ok(Vec::new())
}

fn compile_gray_scott_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::identifier => {
            let op = inner.as_str().to_ascii_lowercase();
            if op == "simulate" {
                genes.push(Gene::new(OpCode::GrayScott, vec![]));
            } else if let Ok(opcode) = OpCode::from_str(&op) {
                genes.push(Gene::new(opcode, vec![]));
            } else {
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(op)]));
            }
        }
        Rule::number => {
            let n: i64 = inner.as_str().parse()?;
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(n)]));
        }
        Rule::string => {
            let s = inner.as_str();
            genes.push(Gene::new(
                OpCode::Push,
                vec![Nucleotide::String(s[1..s.len() - 1].to_string())],
            ));
        }
        _ => {}
    }

    Ok(genes)
}

fn compile_locus_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::identifier => {
            let op = inner.as_str().to_ascii_lowercase();
            if op == "simulate" {
                genes.push(Gene::new(OpCode::Locus, vec![]));
            } else if let Ok(opcode) = OpCode::from_str(&op) {
                genes.push(Gene::new(opcode, vec![]));
            } else {
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(op)]));
            }
        }
        Rule::number => {
            let n: i64 = inner.as_str().parse()?;
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(n)]));
        }
        Rule::string => {
            let s = inner.as_str();
            genes.push(Gene::new(
                OpCode::Push,
                vec![Nucleotide::String(s[1..s.len() - 1].to_string())],
            ));
        }
        _ => {}
    }

    Ok(genes)
}

fn compile_neuro_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::identifier => {
            let op = inner.as_str().to_ascii_lowercase();
            if op == "simulate" {
                genes.push(Gene::new(OpCode::Neuro, vec![]));
            } else if let Ok(opcode) = OpCode::from_str(&op) {
                genes.push(Gene::new(opcode, vec![]));
            } else {
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(op)]));
            }
        }
        Rule::number => {
            let n: i64 = inner.as_str().parse()?;
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(n)]));
        }
        Rule::string => {
            let s = inner.as_str();
            genes.push(Gene::new(
                OpCode::Push,
                vec![Nucleotide::String(s[1..s.len() - 1].to_string())],
            ));
        }
        _ => {}
    }

    Ok(genes)
}

fn compile_platter_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::identifier => {
            let op = inner.as_str().to_ascii_lowercase();
            if op == "simulate" {
                genes.push(Gene::new(OpCode::Platter, vec![]));
            } else if let Ok(opcode) = OpCode::from_str(&op) {
                genes.push(Gene::new(opcode, vec![]));
            } else {
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(op)]));
            }
        }
        Rule::number => {
            let n: i64 = inner.as_str().parse()?;
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(n)]));
        }
        Rule::string => {
            let s = inner.as_str();
            genes.push(Gene::new(
                OpCode::Push,
                vec![Nucleotide::String(s[1..s.len() - 1].to_string())],
            ));
        }
        _ => {}
    }

    Ok(genes)
}

#[cfg(feature = "git")]
fn compile_git_associates_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::identifier => {
            let op = inner.as_str().to_ascii_lowercase();
            if op == "history" {
                genes.push(Gene::new(OpCode::GitHistory, vec![]));
            } else if op == "diff" {
                genes.push(Gene::new(OpCode::GitDiffWorkspace, vec![]));
            } else if let Ok(opcode) = OpCode::from_str(&op) {
                genes.push(Gene::new(opcode, vec![]));
            } else {
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(op)]));
            }
        }
        Rule::number => {
            let n: i64 = inner.as_str().parse()?;
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(n)]));
        }
        Rule::string => {
            let s = inner.as_str();
            let s = &s[1..s.len() - 1]; // Strip quotes
            genes.push(Gene::new(
                OpCode::Push,
                vec![Nucleotide::String(s.to_string())],
            ));
        }
        _ => {}
    }

    Ok(genes)
}

#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "nova")]
    #[test]
    fn test_prologue_esolang_compiler_esolang() {
        let source = r#"
        esolang {
            "Hello World" print
        }
        "#;
        let dna = compile(source).unwrap();
        let genes = &dna.helix.strands[0].genes;

        assert_eq!(genes[0].op, OpCode::Push);
        assert_eq!(
            genes[0].args[0],
            Nucleotide::String("            \"Hello World\" print".to_string())
        );
        assert_eq!(genes[1].op, OpCode::Eval);
    }

    #[test]
    fn test_genetics_block() {
        let code = r#"
genetics {
    splice dna1 dna2
    recombine a b
}
"#;
        let dna = compile(code).unwrap();
        let genes = &dna.helix.strands[0].genes;

        // "splice dna1 dna2" -> Push("dna1"), Push("dna2"), Push(0), Splice
        assert_eq!(genes[0].op, OpCode::Push);
        assert_eq!(genes[0].args[0], Nucleotide::String("dna1".to_string()));
        assert_eq!(genes[1].op, OpCode::Push);
        assert_eq!(genes[1].args[0], Nucleotide::String("dna2".to_string()));
        assert_eq!(genes[2].op, OpCode::Push);
        assert_eq!(genes[2].args[0], Nucleotide::Number(0));
        assert_eq!(genes[3].op, OpCode::Splice);

        // "recombine a b" -> Push("a"), Push("b"), Recombine
        assert_eq!(genes[4].op, OpCode::Push);
        assert_eq!(genes[4].args[0], Nucleotide::String("a".to_string()));
        assert_eq!(genes[5].op, OpCode::Push);
        assert_eq!(genes[5].args[0], Nucleotide::String("b".to_string()));
        assert_eq!(genes[6].op, OpCode::Recombine);
    }

    #[test]
    fn test_quipu_block() {
        let code = r#"
quipu {
    100
    tie
    "knot"
    quipu
    untie
    select
    read
    tangle
}
"#;
        let dna = compile(code).unwrap();
        let genes = &dna.helix.strands[0].genes;

        assert_eq!(genes[0].op, OpCode::Push);
        assert_eq!(genes[0].args[0], Nucleotide::Number(100));
        assert_eq!(genes[1].op, OpCode::Quipu);

        assert_eq!(genes[2].op, OpCode::Knot);

        assert_eq!(genes[3].op, OpCode::Push);
        assert_eq!(genes[3].args[0], Nucleotide::String("knot".to_string()));
        assert_eq!(genes[4].op, OpCode::Quipu);

        assert_eq!(genes[5].op, OpCode::Push);
        assert_eq!(genes[5].args[0], Nucleotide::String("quipu".to_string()));
        assert_eq!(genes[6].op, OpCode::Quipu);

        assert_eq!(genes[7].op, OpCode::Unknot);
        assert_eq!(genes[8].op, OpCode::Cord);
        assert_eq!(genes[9].op, OpCode::ReadCord);
        assert_eq!(genes[10].op, OpCode::Tangle);
    }

    #[test]
    fn test_poincare_block() {
        let code = r#"
poincare {
    100
    hyperbolic
}
"#;
        let dna = compile(code).unwrap();
        let genes = &dna.helix.strands[0].genes;

        assert_eq!(genes[0].op, OpCode::Push);
        assert_eq!(genes[0].args[0], Nucleotide::Number(100));
        assert_eq!(genes[1].op, OpCode::Poincare);
    }

    #[test]
    fn test_chaos_block() {
        let code = r#"
chaos {
    100
    glitch
    entropy
}
"#;
        let dna = compile(code).unwrap();
        let genes = &dna.helix.strands[0].genes;

        assert_eq!(genes[0].op, OpCode::Push);
        assert_eq!(genes[0].args[0], Nucleotide::Number(100));
        assert_eq!(genes[1].op, OpCode::Glitch);
        assert_eq!(genes[2].op, OpCode::EntropySurge);
    }
}
