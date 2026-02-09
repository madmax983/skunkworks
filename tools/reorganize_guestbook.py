import re
import os
import sys

# Priority mapping for sorting
PRIORITY = {
    "CRITICAL MASS": 0,
    "HIGH": 1,
    "FRESH": 2,
    "STABLE": 3,
    "STABLE TRAIL": 3,
    "EVAPORATING": 4,
    "CONDEMNED": 5,
    "TOXIC": 6,
    "EXECUTED": 7
}

def get_priority(level):
    return PRIORITY.get(level, 99)

def parse_guestbook(content):
    lines = content.split('\n')
    current_map = []
    history = []

    current_section = None
    buffer = []

    for line in lines:
        if line.strip().startswith("## 🧫 Current Pheromone Map"):
            current_section = "current"
            continue
        elif line.strip().startswith("## History/Decay"):
            if buffer:
                if current_section == "current":
                    current_map.append(buffer)
                elif current_section == "history":
                    history.append(buffer)
                buffer = []
            current_section = "history"
            continue

        if line.strip().startswith("### ["):
            if buffer:
                if current_section == "current":
                    current_map.append(buffer)
                elif current_section == "history":
                    history.append(buffer)
                buffer = []
            buffer.append(line)
        elif buffer:
            buffer.append(line)

    if buffer:
        if current_section == "current":
            current_map.append(buffer)
        elif current_section == "history":
            history.append(buffer)

    return current_map, history

def get_metadata(block):
    header = block[0]
    # Extract Level and Location
    match = re.search(r"### \[Concentration Level: (.*?)\] - Location: (.*)", header)
    if match:
        return match.group(1).strip(), match.group(2).strip()
    return "UNKNOWN", "UNKNOWN"

def main():
    filepath = "GUESTBOOK.md"
    if not os.path.exists(filepath):
        print("GUESTBOOK.md not found.")
        return

    with open(filepath, "r") as f:
        content = f.read()

    current_map, history = parse_guestbook(content)

    # Reorganize: Move graveyard items from current to history
    new_current = []

    # Map to track existing locations to avoid duplicates
    history_locations = {get_metadata(b)[1]: b for b in history if get_metadata(b)[1] != "UNKNOWN"}

    for block in current_map:
        level, loc = get_metadata(block)
        if loc == "UNKNOWN":
            continue

        if loc.startswith("graveyard/"):
            # Move to history
            if loc not in history_locations:
                history.append(block)
        else:
            new_current.append(block)

    # Deduplicate history
    final_history = []
    seen_history = set()
    for block in history:
        level, loc = get_metadata(block)
        if loc and loc not in seen_history:
            final_history.append(block)
            seen_history.add(loc)

    # Sort Logic
    def sort_key(block):
        level, loc = get_metadata(block)
        prio = get_priority(level)
        return (prio, loc) # Sort by Priority, then Location

    new_current.sort(key=sort_key)
    final_history.sort(key=sort_key)

    # Reconstruct File
    output = []
    output.append("## 🧫 Current Pheromone Map\n")
    for block in new_current:
        output.extend(block)
        # Ensure newline between blocks
        if block[-1].strip() != "":
            output.append("")

    output.append("## History/Decay\n")
    for block in final_history:
        output.extend(block)
        if block[-1].strip() != "":
            output.append("")

    with open(filepath, "w") as f:
        f.write("\n".join(output))

if __name__ == "__main__":
    main()
