import re

content = open('experiments/chimera-lang/src/tui/app/handlers/editing/enter.rs').read()

# Remove the duplicate match block at the bottom
lines = content.split('\n')
start_idx = -1
end_idx = -1
for i, line in enumerate(lines):
    if "ViewMode::Pandemonium" in line and start_idx == -1:
        start_idx = i
    if "ViewMode::Microscope | ViewMode::Heatmap | ViewMode::Catalyst => {" in line:
        end_idx = i + 3
        break

if start_idx != -1 and end_idx != -1:
    lines = lines[:start_idx] + lines[end_idx:]

new_content = '\n'.join(lines)


# Add helper functions at the top
helper_str = """
fn handle_genome_enter(vm: &mut ChimeraVM, app_state: &mut AppState) {
    match ChimeraParser::parse(Rule::gene, &app_state.input_buffer) {
        Ok(mut pairs) => {
            let pair = pairs.next().unwrap();
            match Gene::try_from_pair(pair) {
                Ok(gene) => {
                    if app_state.selected_strand < vm.dna.helix.strands.len()
                        && app_state.selected_gene
                            < vm.dna.helix.strands[app_state.selected_strand]
                                .genes
                                .len()
                    {
                        vm.dna.helix.strands[app_state.selected_strand].genes
                            [app_state.selected_gene] = gene;
                        app_state.status_msg = "Gene updated successfully".to_string();
                    }
                    app_state.input_mode = InputMode::Normal;
                    app_state.input_buffer.clear();
                }
                Err(e) => {
                    app_state.status_msg = format!("Parse Error: {}", e);
                }
            }
        }
        Err(e) => {
            app_state.status_msg = format!("Parse Error: {}", e);
        }
    }
}

#[cfg(feature = "nova")]
fn handle_crispr_enter(vm: &mut ChimeraVM, app_state: &mut AppState) {
    let guide_tokens: Vec<&str> = app_state.crispr_guide.split_whitespace().collect();
    let replace_tokens: Vec<&str> = app_state.crispr_replace.split_whitespace().collect();
    use std::str::FromStr;

    let mut guide_ops = Vec::new();
    for t in &guide_tokens {
        if let Ok(op) = crate::opcode::OpCode::from_str(t) {
            guide_ops.push(op);
        }
    }
    let mut replace_genes = Vec::new();
    for t in &replace_tokens {
        if let Ok(op) = crate::opcode::OpCode::from_str(t) {
            replace_genes.push(crate::ast::Gene { op, args: vec![] });
        }
    }

    if guide_ops.is_empty() {
        app_state.crispr_result = "Error: Empty Guide Pattern".to_string();
    } else {
        let s_idx = app_state.crispr_target_strand;
        if s_idx < vm.dna.helix.strands.len() {
            let strand = &mut vm.dna.helix.strands[s_idx];
            let mut new_genes = Vec::new();
            let mut i = 0;
            let mut matches = 0;
            while i < strand.genes.len() {
                let mut matched = true;
                for (j, op) in guide_ops.iter().enumerate() {
                    if i + j >= strand.genes.len() || strand.genes[i + j].op != *op {
                        matched = false;
                        break;
                    }
                }
                if matched {
                    new_genes.extend(replace_genes.clone());
                    i += guide_ops.len();
                    matches += 1;
                } else {
                    new_genes.push(strand.genes[i].clone());
                    i += 1;
                }
            }
            strand.genes = new_genes;
            app_state.crispr_result = format!("CRISPR: Replaced {} occurrences.", matches);
        } else {
            app_state.crispr_result = "Error: Invalid Strand".to_string();
        }
    }
}
"""

new_content = new_content.replace('pub(crate) fn handle_enter(', helper_str + '\npub(crate) fn handle_enter(')


# Replace the contents of ViewMode::Genome and ViewMode::Crispr with helper calls

genome_logic_pattern = re.compile(r'ViewMode::Genome => \{(.*?)\} \n\s*ViewMode::Grid =>', re.DOTALL)
def handle_genome_extract(match):
    return 'ViewMode::Genome => {\n                                    handle_genome_enter(vm, app_state);\n                                }\n                                ViewMode::Grid =>'
new_content = genome_logic_pattern.sub(handle_genome_extract, new_content)

crispr_logic_pattern = re.compile(r'ViewMode::Crispr => \{(.*?)\} \n\s*#\[cfg\(feature = "nova"\)\]\n\s*ViewMode::Logos =>', re.DOTALL)
def handle_crispr_extract(match):
    return 'ViewMode::Crispr => {\n                                    handle_crispr_enter(vm, app_state);\n                                }\n                                #[cfg(feature = "nova")]\n                                ViewMode::Logos =>'
new_content = crispr_logic_pattern.sub(handle_crispr_extract, new_content)

open('experiments/chimera-lang/src/tui/app/handlers/editing/enter.rs', 'w').write(new_content)
