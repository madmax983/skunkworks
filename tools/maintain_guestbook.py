
import re
import os

def parse_guestbook(content):
    entries = []
    current_entry = {}
    lines = content.split('\n')

    for line in lines:
        stripped = line.strip()
        if stripped.startswith('## '):
            continue

        if line.startswith('### ['):
            if current_entry:
                entries.append(current_entry)
            current_entry = {'raw': line, 'lines': [line]}
            # Extract level and location
            match = re.search(r'\[Concentration Level: (.*?)\] - Location: (.*)', line)
            if match:
                current_entry['level'] = match.group(1)
                current_entry['location'] = match.group(2).strip()
        elif current_entry:
            current_entry['lines'].append(line)

    if current_entry:
        entries.append(current_entry)
    return entries

def parse_mutations(content):
    hybrids = []
    lines = content.split('\n')
    in_spawned = False
    current_hybrid = {}

    for line in lines:
        stripped = line.strip()
        if stripped == '## 🌿 Spawned Hybrids' or stripped == '## 🧪 Recombination Techniques':
            in_spawned = True
            # Save previous hybrid if any (from previous section end? unlikely but safe)
            if current_hybrid and 'location' in current_hybrid:
                 hybrids.append(current_hybrid)
                 current_hybrid = {}
            continue
        if stripped.startswith('## ') and stripped not in ['## 🌿 Spawned Hybrids', '## 🧪 Recombination Techniques']:
            in_spawned = False
            if current_hybrid and 'location' in current_hybrid:
                 hybrids.append(current_hybrid)
            current_hybrid = {}
            continue

        if in_spawned:
            if line.startswith('### '):
                if current_hybrid and 'location' in current_hybrid:
                     hybrids.append(current_hybrid)
                current_hybrid = {'name': line.strip().replace('### ', '')}
            elif current_hybrid:
                if stripped.startswith('- **Parents**:'):
                    current_hybrid['parents'] = line.split(':', 1)[1].strip()
                elif stripped.startswith('- **Concept**:'):
                    current_hybrid['concept'] = line.split(':', 1)[1].strip()
                elif stripped.startswith('- **Status**:'):
                    current_hybrid['location'] = line.split(':', 1)[1].strip()
                elif stripped.startswith('- **Evaluation**:'):
                    current_hybrid['evaluation'] = line.split(':', 1)[1].strip()
                elif stripped.startswith('- **Novel trait**:'):
                    current_hybrid['trait'] = line.split(':', 1)[1].strip()

    if current_hybrid and 'location' in current_hybrid:
        hybrids.append(current_hybrid)
    return hybrids

def main():
    with open('GUESTBOOK.md', 'r', encoding='utf-8') as f:
        gb_content = f.read()

    with open('MUTATIONS.md', 'r', encoding='utf-8') as f:
        mut_content = f.read()

    gb_entries = parse_guestbook(gb_content)
    mut_hybrids = parse_mutations(mut_content)

    # Map mutations to guestbook entries
    new_entries = []
    existing_locations = {e.get('location'): e for e in gb_entries}

    for h in mut_hybrids:
        location = h.get('location')
        if not location or location in existing_locations:
            continue

        # Create new entry
        lines = []
        lines.append(f"### [Concentration Level: FRESH] - Location: {location}")
        lines.append(f"- **Scent Origin:** The Splice Surgeon 🧬")
        status = h.get('evaluation', 'Freshly spawned.')
        concept = h.get('concept', '')
        trait = h.get('trait', '')

        status_line = f"- **Status:** {status}"
        if concept:
            status_line += f" {concept}"
        if trait:
            status_line += f" Novel trait: {trait}"
        lines.append(status_line)

        if 'parents' in h:
            lines.append(f"- **Note:** Hybrid of {h['parents']}.")
        lines.append("") # Empty line

        new_entries.append({
            'level': 'FRESH',
            'location': location,
            'lines': lines
        })

    # Combine lists
    all_entries = gb_entries + new_entries

    # Separate Active vs Decay
    active_levels = ['TOXIC', 'HIGH', 'STABLE TRAIL', 'FRESH', 'VERIFIED']
    decay_levels = ['EXECUTED', 'EVAPORATING']

    active_entries = []
    decay_entries = []

    for e in all_entries:
        level = e.get('level', 'FRESH') # Default to FRESH if unknown
        location = e.get('location', '')
        if level in decay_levels or location.startswith('graveyard/'):
            decay_entries.append(e)
        else:
            active_entries.append(e)

    # Write output
    output = []
    output.append("## 🧫 Current Pheromone Map\n")

    # Priority: TOXIC > HIGH > STABLE TRAIL > FRESH > VERIFIED
    priority = {'TOXIC': 0, 'HIGH': 1, 'STABLE TRAIL': 2, 'FRESH': 3, 'VERIFIED': 4}
    active_entries.sort(key=lambda x: priority.get(x.get('level'), 5))

    for e in active_entries:
        output.extend(e['lines'])

    output.append("\n## 🍂 History/Decay\n")

    for e in decay_entries:
        output.extend(e['lines'])

    with open('GUESTBOOK.md', 'w', encoding='utf-8') as f:
        f.write('\n'.join(output))

    print(f"Added {len(new_entries)} new entries.")

if __name__ == '__main__':
    main()
