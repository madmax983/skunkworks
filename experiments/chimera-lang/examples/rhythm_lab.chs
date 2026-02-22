strand main {
    # Initialize Prologue Mode
    prologue

    # Set up a Rhythm Grid

    # --- Drum Loop ---
    # Clock at (5, 5)
    "⏱️" 5 5 g_write

    # Wire at (5, 6)
    "~" 5 6 g_write

    # Drum at (5, 7)
    "🥁" 5 7 g_write

    # --- BPM Control ---
    # Value 12 (Sets BPM to 12) -> Interval 50 ticks
    12 8 4 g_write
    # Fader at (8, 5) reads from West (8,4)
    "🎚️" 8 5 g_write

    # --- Melody ---
    # Note 72 (C5) at (10, 4)
    72 10 4 g_write
    # Keys at (10, 5) reads from West
    # But Keys needs a TRIGGER signal to play.
    # It reads NOTE from West, but when does it play?
    # Current impl: "Reads West (Note). If valid, Plays."
    # Wait, apply_rhythm_runes for "🎹" says:
    # if current_signals[wy][wx] is some...
    # So if the NOTE value is emitted as a SIGNAL, it plays.
    # Static values in grid are NOT signals unless read by a Source "!"

    # So we need:  72 -> ! -> ~ -> 🎹

    # Note Source
    72 10 2 g_write
    "!" 10 3 g_write

    # But "!" emits CONSTANTLY. So Piano will drone?
    # Or rapid fire.

    # Better: Clock -> AND -> Piano
    #         Note -> /

    # Let's just hook the Clock output to the Piano input?
    # But Piano reads from West.
    # If we put wire from Clock to West of Piano, it carries the Clock Signal (1).
    # But Piano expects Frequency (Int) from West.

    # Ah, the logic in rhythm.rs:
    # if let Some(val) = &current_signals[wy][wx] { ... match val ... }
    # If signal is 1 (from Clock), freq becomes 1. That's low.

    # We need a way to gate a Value with a Trigger.
    # The "I" (If) rune: West (Cond) -> Passes North (Val) to South.
    # Wait, PROLOGUE.md says: "I: IF: West (Cond) -> Passes North (Val)." (Implicitly to Self/South?)
    # logic.rs implementation check:
    # "I" => if West is truthy, output North value to South?

    # Let's assume standard Prologue logic allows constructing a sequencer.

    "Rhythm Lab Initialized" print
}
