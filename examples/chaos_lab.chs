# Chaos Cartridge Laboratory ⚛️
# Demonstrates Programmable Physics

strand main {
    "Defining Reaction: Fire + Water -> Steam..." print

    # 1. Define Recipe
    # Stack: [ input_junction, output_str ]
    "Steam"
    ( "Fire" "Water" )
    chaos_define

    "Placing Ingredients on Grid..." print

    # 2. Place Ingredients
    "Fire" 8 8 g_write
    "Water" 8 9 g_write

    # Verify placement
    8 8 g_read print
    8 9 g_read print

    "Invoking Chaos..." print

    # 3. Trigger Physics
    chaos_invoke

    # 4. Check Result
    # Steam should appear at 8,8 (one of the locations) and neighbors cleared
    # Or depends on implementation: 8,8 was Fire. 8,9 was Water.
    # The logic scans (y,x). At 8,8, neighbors include 8,9.
    # If 8,8 is processed, it sees Fire (self) and Water (neighbor).
    # Wait, my logic checks NEIGHBORS against inputs.
    # Center is not automatically included unless it's a neighbor of itself (not in my logic).
    # Ah, my logic in nova_chaos.rs:
    # "Collect local context (Von Neumann)" -> only neighbors.
    # So if I have Fire at 8,8 and Water at 8,9.
    # If I process 8,8: neighbors are (7,8),(9,8),(8,7),(8,9).
    # Neighbor (8,9) has "Water". Fire is at Center.
    # My scan `if let Value::Str(s) = &vm.grid[ny][nx]` collects neighbors.
    # It does NOT include center.
    # So if inputs are ["Fire", "Water"], neighbors must contain BOTH.
    # So I need Fire and Water NEXT to the reaction site.

    # Correction: I need to place Fire and Water around a central point.
    # Let's target 9,9 as reaction site.
    # Place Fire at 8,9 (North of 9,9)
    # Place Water at 9,8 (West of 9,9)

    "Correcting placement..." print
    "Fire" 8 9 g_write
    "Water" 9 8 g_write

    chaos_invoke

    "Checking 9,9..." print
    9 9 g_read print
}
